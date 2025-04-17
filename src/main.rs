use clap::Parser;
use rcli::{
    Cli,
    cli::{SubCmds, csv::process::process_csv, genpwd::process::process_genpwd},
};
fn main() -> anyhow::Result<()> {
    // 启用Windows终端的彩色支持
    #[cfg(windows)]
    enable_windows_colors();

    let cli = Cli::parse();

    match cli.sub_cmd {
        SubCmds::Csv(opts) => process_csv(opts.input, opts.header, opts.delimiter, opts.output)?,
        SubCmds::GenPwd(opts) => {
            process_genpwd(opts.length, opts.uppercase, opts.number, opts.symbol)?
        }
    }
    Ok(())
}

#[cfg(windows)]
fn enable_windows_colors() {
    let _ = colored::control::set_virtual_terminal(true);
    colored::control::set_override(true);
}
