pub mod base64;
pub mod csv;
pub mod genpwd;

use self::base64::arg::Base64SubCmds;
use self::csv::arg::CsvArgs;
use self::genpwd::arg::GenPwdArgs;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum SubCmds {
    /// Show CSV, or covert CSV to other formats
    #[command(name = "csv", arg_required_else_help = true, disable_help_flag = true)]
    Csv(CsvArgs),

    /// Generate a random password with given length and complexity
    #[command(
        name = "genpwd",
        arg_required_else_help = true,
        disable_help_flag = true
    )]
    GenPwd(GenPwdArgs),

    /// Encode or decode base64
    #[command(
        //元组里面还是子命令，需要添加subcommand，否则会报错，因为元组里面是子命令，而不是子命令的参数，所以需要添加subcommand
        subcommand,
        name = "base64",
        arg_required_else_help = true,
        disable_help_flag = true
    )]
    Base64(Base64SubCmds),
}
