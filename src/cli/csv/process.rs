use std::{fs, path::PathBuf, vec};

use anyhow::{Context, anyhow};
use csv::StringRecord;
use serde_json::Value;

use super::arg::OutputFormatter;

pub fn process_csv(
    input: String,
    header: bool,
    delimiter: char,
    output: String,
) -> anyhow::Result<()> {
    let fmt = get_output_formatter(&output)?;
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter.to_string().as_bytes()[0])
        .has_headers(header)
        .from_path(&input)
        .with_context(|| format!("Failed to read the CSV file `{}`", &input))?;

    let mut vec_rcds: Vec<Value> = Vec::with_capacity(128);
    let first_row = reader
        .headers()
        .with_context(|| format!("Failed to get header of this CSV file `{}`", &input))?;
    let headers;
    if !header {
        // without header
        headers = first_row
            .clone()
            .iter()
            .zip(1..)
            .map(|(_v, num)| format!("Column {num}"))
            .collect::<StringRecord>();

        let json_value: Value = headers.iter().zip(first_row.iter()).collect();
        vec_rcds.push(json_value);
    } else {
        // with header
        headers = first_row.clone();
    }
    for record in reader.records() {
        let result = record.with_context(|| "Failed to get record")?;
        let json_value: Value = headers.iter().zip(result.iter()).collect();
        vec_rcds.push(json_value);
    }
    crate_output_file(&output, &vec_rcds, fmt)?;
    Ok(())
}

fn crate_output_file(
    output: &str,
    vec_data: &Vec<Value>,
    fmt: OutputFormatter,
) -> anyhow::Result<()> {
    let file_path = PathBuf::from(output);
    let content = match fmt {
        OutputFormatter::Json => serde_json::to_string_pretty(vec_data)
            .with_context(|| "Failed to convert json `Value` to String")?,

        OutputFormatter::Csv => {
            let mut wtr = csv::WriterBuilder::new().from_writer(vec![]);
            if let Some(first_row) = vec_data.first() {
                if let Some(obj) = first_row.as_object() {
                    let header: Vec<String> = obj.keys().map(|key| key.to_owned()).collect();
                    println!("Header: {:#?}", header);
                    wtr.write_record(&header)
                        .with_context(|| "Failed to write CSV headers")?;
                    for item in vec_data {
                        if let Some(map) = item.as_object() {
                            let row: Vec<String> = header
                                .iter()
                                .map(|key| match map.get(key) {
                                    Some(Value::String(val)) => val.clone(),
                                    Some(Value::Number(val)) => val.to_string(),
                                    Some(Value::Bool(val)) => val.to_string(),
                                    Some(Value::Null) => String::new(),
                                    _ => String::new(),
                                })
                                .collect();
                            wtr.write_record(&row)
                                .with_context(|| "Failed to write CSV record")?;
                        }
                    }
                }
            }
            wtr.flush()
                .with_context(|| "Failed to flush the CSV writer")?;

            String::from_utf8(
                wtr.into_inner()
                    .with_context(|| "Failed to flush the CSV writer")?,
            )?
        }
        OutputFormatter::Yaml => serde_yaml::to_string(vec_data)
            .with_context(|| "Failed to convert yaml from json value")?,
        OutputFormatter::Toml => {
            let root = serde_json::json!({"data": vec_data});
            toml::to_string_pretty(&root)
                .with_context(|| "Failed to convert toml from json value")?
        }
        OutputFormatter::Xml => todo!(),
    };
    fs::write(file_path.clone(), content)
        .with_context(|| format!("Failed to Write data in CSV file `{}`", file_path.display()))
}

fn get_output_formatter(output: &str) -> anyhow::Result<OutputFormatter> {
    let file_path = PathBuf::from(output);

    let opt_file_ext = file_path.extension().and_then(|ext| ext.to_str());
    let file_ext = match opt_file_ext {
        None => return Err(anyhow!("Invalid file extension".to_string())),
        Some(ext) => ext.parse::<OutputFormatter>()?,
    };
    Ok(file_ext)
}
