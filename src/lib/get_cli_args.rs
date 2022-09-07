use docopt::Docopt;
use crate::lib::{
    types::Result,
    errors::AppError,
    usage_info::USAGE_INFO,
};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct CliArgs {
    pub cmd_version: bool,
    pub arg_prefix: String,
    pub flag_logLevel: String,
    pub cmd_generateRandomAddress: bool,
    pub cmd_generateVanityAddress: bool,
}

pub fn get_cli_args() -> Result<CliArgs> {
    match Docopt::new(USAGE_INFO)
        .and_then(|d| d.deserialize()) {
            Ok(cli_args) => Ok(cli_args),
            Err(e) => Err(AppError::Custom(e.to_string()))
        }
}
