//command: rcli csv -i input.csv -o output.json --header -d ','

use std::path::Path;

use anstyle::{AnsiColor, Effects};
use clap::{
    Args, Parser, Subcommand,
    builder::{Styles, styling::Style},
};

#[derive(Debug, Parser)]
#[command(name="rcli",version, about, long_about = None,color = clap::ColorChoice::Always,
styles = Styles::styled()
.header(anstyle::Style::new().fg_color(Some(AnsiColor::BrightCyan.into())).effects(Effects::BOLD)) // 标题
.usage(Style::new().fg_color(Some(AnsiColor::BrightGreen.into())).effects(Effects::BOLD)) // 用法
.literal(Style::new().fg_color(Some(AnsiColor::BrightCyan.into())).effects(Effects::BOLD)) // 字面量
.placeholder(Style::new().fg_color(Some(AnsiColor::BrightBlue.into()))) // 占位符
)]
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
    /// CSV file has header or not
    #[arg(short, long, default_value = "true")]
    header: Option<bool>,
    /// Set the delimiter
    #[arg(short,long, default_value = ",", value_parser = verify_delimiter)]
    delimiter: char,
    /// Output file path
    #[arg(short, long, default_value = "output.json", value_name = "FILE")]
    output: String,
}

fn main() {
    // 启用Windows终端的彩色支持
    #[cfg(windows)]
    use colored::Colorize;
    let _ = colored::control::set_virtual_terminal(true);
    // 强制启用颜色输出
    colored::control::set_override(true);

    let cli = Cli::parse();
    println!("{:#?}", cli);

    println!("{}", "这是红色文本".red());
    println!("{}", "这是绿色文本".green());
    println!("{}", "这是蓝色文本".blue());
}

fn verify_input_file(file_name: &str) -> Result<String, String> {
    if Path::new(file_name).exists() {
        Ok(file_name.to_owned())
    } else {
        Err(format!("File `{}` does NOT exist", file_name))
    }
}

fn verify_delimiter(delimiter: &str) -> Result<char, String> {
    if delimiter.chars().count() == 1 {
        Ok(delimiter.chars().next().unwrap())
    } else {
        Err(format!(
            "`{}` is not a valid delimiter of this CSV file",
            delimiter
        ))
    }
}
