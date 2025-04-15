use clap::Parser;
use rcli::{Cli, process::process_csv, subcmd::SubCmds};
fn main() {
    // 启用Windows终端的彩色支持
    #[cfg(windows)]
    enable_windows_colors();

    let cli = Cli::parse();
    // println!("{:#?}", cli);
    match cli.sub_cmd {
        SubCmds::Csv(opts) => {
            process_csv(opts);
        }
    }
}

#[cfg(windows)]
fn enable_windows_colors() {
    let _ = colored::control::set_virtual_terminal(true);
    colored::control::set_override(true);
}
