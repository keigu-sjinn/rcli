pub mod csv;

use self::csv::arg::CsvArgs;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum SubCmds {
    /// Show CSV, or covert CSV to other formats
    #[command(name = "csv", arg_required_else_help = true, disable_help_flag = true)]
    Csv(CsvArgs),
}
