pub mod csv;
pub mod genpwd;

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
}
