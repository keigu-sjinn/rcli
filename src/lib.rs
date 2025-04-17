pub mod cli;

use anstyle::{AnsiColor, Effects, Style};
use clap::{Parser, builder::Styles};
use cli::SubCmds;

#[derive(Debug, Parser)]
#[command(name="rcli",version, about, long_about = None,color = clap::ColorChoice::Always,
styles = Styles::styled()
.error(Style::new().fg_color(Some(AnsiColor::BrightRed.into())).effects(Effects::BOLD)) // 错误
.header(anstyle::Style::new().fg_color(Some(AnsiColor::BrightGreen.into())).effects(Effects::BOLD)) // 标题
.usage(Style::new().fg_color(Some(AnsiColor::BrightGreen.into())).effects(Effects::BOLD)) // 用法
.literal(Style::new().fg_color(Some(AnsiColor::BrightCyan.into())).effects(Effects::BOLD)) // 字面量
.placeholder(Style::new().fg_color(Some(AnsiColor::BrightBlue.into()))) // 占位符
)]
pub struct Cli {
    #[command(subcommand)]
    pub sub_cmd: SubCmds,
}
