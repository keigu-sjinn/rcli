use std::{
    fmt,
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::{Args, arg};

use super::{DEFAULT_DELIMITER, DEFAULT_HEADER, DEFAULT_INPUT_FILE, DEFAULT_OUTPUT_FILE};

#[derive(Debug, Args)]
pub struct CsvArgs {
    #[arg(
        help=format!("Input file path"),
        required = true,
        short,
        long,
        value_name = "FILE",
        value_parser=verify_input_file,
        default_value = DEFAULT_INPUT_FILE,
        default_missing_value = DEFAULT_INPUT_FILE,
        num_args=0..=1,
    )]
    pub input: String,

    /// CSV file has header or not
    #[arg(
        short,
        long,
        default_value = DEFAULT_HEADER,
        value_parser=verify_header,
        default_missing_value=DEFAULT_HEADER,
        num_args=0..=1
    )]
    pub header: bool,

    /// Set the delimiter
    #[arg(
        short,
        long,
        default_value=DEFAULT_DELIMITER,
        default_missing_value=DEFAULT_DELIMITER,
        value_parser=verify_delimiter,
        num_args=0..=1
    )]
    pub delimiter: char,

    /// Output file path
    #[arg(
        short,
        long,
        default_value = DEFAULT_OUTPUT_FILE,
        default_missing_value = DEFAULT_OUTPUT_FILE,
        value_name = "FILE.EXT",
        value_parser=verify_output_file,
        num_args=0..=1
    )]
    pub output: String,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormatter {
    Json,
    Csv,
    Yaml,
    Toml,
    Xml,
}

impl From<OutputFormatter> for &'static str {
    fn from(value: OutputFormatter) -> Self {
        match value {
            OutputFormatter::Json => "json",
            OutputFormatter::Csv => "csv",
            OutputFormatter::Yaml => "yaml",
            OutputFormatter::Toml => "toml",
            OutputFormatter::Xml => "xml",
        }
    }
}

impl FromStr for OutputFormatter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let file_ext = s.to_lowercase();
        match file_ext.as_str() {
            "json" => Ok(OutputFormatter::Json),
            "csv" => Ok(OutputFormatter::Csv),
            "yaml" => Ok(OutputFormatter::Yaml),
            "yml" => Ok(OutputFormatter::Yaml),
            "toml" => Ok(OutputFormatter::Toml),
            "xml" => Ok(OutputFormatter::Xml),
            fmt => Err(anyhow::anyhow!("`{}` is not a unsupported format", fmt)),
        }
    }
}

impl fmt::Display for OutputFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

fn verify_input_file(file_name: &str) -> Result<String, String> {
    if Path::new(file_name).exists() {
        Ok(file_name.to_owned())
    } else {
        Err(format!("File `{}` does NOT exist", file_name))
    }
}
fn verify_output_file(file_name: &str) -> Result<String, String> {
    let file_path = PathBuf::from(file_name);
    let opt_file_stem = file_path.file_stem().and_then(|name| name.to_str());
    // .ok_or(|| format!("Invalid file name `{}`", file_name));
    let file_stem = match opt_file_stem {
        None => return Err("Invalid file name".to_string()),
        Some(stem) => stem,
    };

    let opt_file_ext = file_path.extension().and_then(|ext| ext.to_str());
    let file_ext = match opt_file_ext {
        None => return Err("Invalid file extension".to_string()),
        Some(ext) => ext
            .parse::<OutputFormatter>()
            .map_err(|err| err.to_string())?,
    };
    // Ok(format!("{}.{}", file_stem, file_ext.into()));
    Ok(format!("{}.{}", file_stem, <&str>::from(file_ext)))
}
fn verify_delimiter(delimiter: &str) -> Result<char, String> {
    if delimiter.is_empty() {
        return Err("Delimiter can NOT be empty".to_owned());
    }

    if delimiter.chars().count() == 1 {
        Ok(delimiter.chars().next().unwrap())
    } else {
        Err(format!(
            "`{}` is not a valid delimiter of this CSV file",
            delimiter
        ))
    }
}

fn verify_header(header: &str) -> Result<bool, String> {
    header.parse::<bool>().map_err(|_| {
        format!(
            "`{}` is not a valid boolean value, please input `true` or `false`",
            header
        )
    })
}
