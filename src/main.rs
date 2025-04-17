use clap::Parser;
use rcli::{
    Cli,
    cli::{
        SubCmds,
        base64::{
            arg::Base64SubCmds,
            process::{process_base64_decode, process_base64_encode},
        },
        csv::process::process_csv,
        genpwd::process::process_genpwd,
    },
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

        SubCmds::Base64(bs64subcomd) => match bs64subcomd {
            Base64SubCmds::Encode(encode_args) => {
                process_base64_encode(&encode_args.input, &encode_args.formatter)?
            }

            Base64SubCmds::Decode(decode_args) => {
                process_base64_decode(&decode_args.input, &decode_args.formatter)?
            }
        },
    }
    Ok(())
}

#[cfg(windows)]
fn enable_windows_colors() {
    let _ = colored::control::set_virtual_terminal(true);
    colored::control::set_override(true);
}
