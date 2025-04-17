use clap::Args;

use crate::cli::genpwd::{
    DEFAULT_PWD_LENGTH, DEFAULT_PWD_NUMBER, DEFAULT_PWD_SYMBOL, DEFAULT_PWD_UPPERCASE,
};

use super::DEFAULT_PWD_LENGTH_STR;

#[derive(Debug, Args)]
pub struct GenPwdArgs {
    /// length of generate a random password (Min:4, Max:255)
    #[arg(short, long, required=true, default_value_t = DEFAULT_PWD_LENGTH,
        default_missing_value=DEFAULT_PWD_LENGTH_STR,
        value_parser = clap::value_parser!(u8).range(4..=255),num_args=0..=1)]
    pub length: u8,
    /// generate a random password with uppercase or not
    #[arg(short, long, default_value = DEFAULT_PWD_UPPERCASE, default_missing_value=DEFAULT_PWD_UPPERCASE, value_parser=verify_bool, num_args=0..=1)]
    pub uppercase: bool,
    /// generate a random password with number or not
    #[arg(short, long, default_value = DEFAULT_PWD_NUMBER, default_missing_value=DEFAULT_PWD_NUMBER, value_parser=verify_bool, num_args=0..=1)]
    pub number: bool,
    /// generate a random password with symbol or not
    #[arg(short, long, default_value = DEFAULT_PWD_SYMBOL, default_missing_value=DEFAULT_PWD_SYMBOL, value_parser=verify_bool, num_args=0..=1)]
    pub symbol: bool,
}

fn verify_bool(input: &str) -> Result<bool, String> {
    input.parse::<bool>().map_err(|_| {
        format!(
            "`{}` is not a valid boolean value, please input `true` or `false`",
            input
        )
    })
}
