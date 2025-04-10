//command: rcli csv -i input.csv -o output.json --header -d ','

use std::path::Path;

use clap::{Args, Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(name="rcli",author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    sub_cmd: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    /// Show CSV, or covert CSV to other formats
    #[command(name = "csv", arg_required_else_help = true, disable_help_flag = true)]
    Csv(CsvArgs),
}
#[derive(Debug, Args)]
struct CsvArgs {
    /// Input file path
    #[arg(short, long,value_parser=verify_input_file, required = true, value_name = "FILE")]
    input: String,
    /// Output file path
    #[arg(short, long, default_value = "output.json", value_name = "FILE")]
    output: String,
    /// CSV file has header or not
    #[arg(short, long, default_value = "true")]
    header: Option<bool>,
    /// Set the delimiter
    #[arg(short,long, default_value = ",", value_parser = verify_delimiter)]
    delimiter: char,
}

fn main() {
    let cli = Cli::parse();
    println!("{:#?}", cli);
}

fn verify_input_file(file_name: &str) -> Result<String, String> {
    if Path::new(file_name).exists() {
        Ok(file_name.to_owned())
    } else {
        Err(format!("`{}` does NOT exist", file_name))
    }
}

fn verify_delimiter(delimiter: &str) -> Result<char, String> {
    if delimiter.len() == 1 {
        Ok(delimiter.chars().next().unwrap())
    } else {
        Err(format!("`{}` is not a valid delimiter", delimiter))
    }
}
