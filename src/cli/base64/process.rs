use std::{fs::File, io::Read, path::Path};

use anyhow::Context;

use super::{DEFAULT_BUFF_SIZE, arg::Base64Formatter};
use base64::prelude::*;

pub fn process_base64_encode(input: &str, formatter: &Base64Formatter) -> anyhow::Result<()> {
    let input_data = read_input_data(input)?;
    let encode = match *formatter {
        Base64Formatter::Base64Standard => BASE64_STANDARD.encode(input_data),
        Base64Formatter::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(input_data),
    };
    println!("{}", encode);
    Ok(())
}

pub fn process_base64_decode(input: &str, formatter: &Base64Formatter) -> anyhow::Result<()> {
    let input_data = read_input_data(input)?;
    let decode = match *formatter {
        Base64Formatter::Base64Standard => BASE64_STANDARD
            .decode(input_data)
            .with_context(|| "Fail to decode by BASE64 STANDARD Engine")?,
        Base64Formatter::UrlSafe => BASE64_URL_SAFE_NO_PAD
            .decode(input_data)
            .with_context(|| "Fail to decode by BASE64 URL SAFE Engine")?,
    };
    println!("{}", String::from_utf8(decode)?);
    Ok(())
}

fn read_input_data(input: &str) -> anyhow::Result<String> {
    let mut rdr: Box<dyn Read> = match input {
        "-" => Box::new(std::io::stdin()),
        file => Box::new(File::open(Path::new(file))?),
    };
    let mut buf: String = String::with_capacity(DEFAULT_BUFF_SIZE);
    rdr.read_to_string(&mut buf)
        .with_context(|| "Failed to read input data")?;
    Ok(buf.trim().to_owned())
}
