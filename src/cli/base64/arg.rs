use std::{fmt, path::Path, str::FromStr};

use clap::{Args, Subcommand, command};

#[derive(Debug, Subcommand)]
pub enum Base64SubCmds {
    /// Encode string to base64
    #[command(
        name = "encode",
        arg_required_else_help = true,
        disable_help_flag = true
    )]
    Encode(EncodeArgs),

    /// Decode base64 to string
    #[command(
        name = "decode",
        arg_required_else_help = true,
        disable_help_flag = true
    )]
    Decode(DecodeArgs),
}

#[derive(Debug, Args)]

pub struct EncodeArgs {
    /// Input file or string from `stdin` for encoding base64
    /// if not specified, stdin is used
    #[arg(
    required = true,
    short,
    long,
    value_name = "FILE/STDIN",
    value_parser=verify_input,
    default_value = "-",
    default_missing_value = "-",
    num_args=0..=1,)]
    pub input: String,

    /// Base64 formatter
    #[arg(
        short,
        long,
        value_name = "BASE64_FORMATTER",
        value_parser=verify_base64_formatter,
        default_value = "standard",
        default_missing_value = "standard",
        num_args=0..=1,)]
    pub formatter: Base64Formatter,
}

#[derive(Debug, clap::Args)]
pub struct DecodeArgs {
    /// Input file or base64 from `stdin` for decoding from base64 to string
    /// if not specified, stdin is used
    #[arg(required = true,
        short,
        long,
        value_name = "FILE/STDIN",
        value_parser=verify_input,
        default_value = "-",
        default_missing_value = "-",
        num_args=0..=1,)]
    pub input: String,

    /// Base64 formatter
    #[arg(
        short,
        long,
        value_name = "BASE64_FORMATTER",
        value_parser=verify_base64_formatter,
        default_value = "standard",
        default_missing_value = "standard",
        num_args=0..=1,)]
    pub formatter: Base64Formatter,
}

fn verify_input(s: &str) -> Result<String, String> {
    if s == "-" || Path::new(s).exists() {
        return Ok(s.into());
    }

    Err(format!("{} is not a valid file", s))
}

fn verify_base64_formatter(s: &str) -> Result<Base64Formatter, String> {
    match Base64Formatter::from_str(s) {
        Ok(formatter) => Ok(formatter),
        Err(_) => Err(format!("{} is not a valid base64 formatter", s)),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Formatter {
    Base64Standard,
    UrlSafe,
}

impl From<Base64Formatter> for &'static str {
    fn from(value: Base64Formatter) -> Self {
        match value {
            Base64Formatter::Base64Standard => "standard",
            Base64Formatter::UrlSafe => "urlsafe",
        }
    }
}

impl FromStr for Base64Formatter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Formatter::Base64Standard),
            "urlsafe" => Ok(Base64Formatter::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 formatter")),
        }
    }
}

impl fmt::Display for Base64Formatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
