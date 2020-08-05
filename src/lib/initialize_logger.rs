use simplelog::*;
use log::LevelFilter;
use crate::lib::{
    types::Result,
    errors::AppError,
    get_cli_args::CliArgs,
};

pub fn maybe_initialize_logger_and_return_cli_args(cli_args: CliArgs) -> Result<CliArgs> {
    match &cli_args.flag_logLevel[..] {
        "none" => Ok(cli_args),
        _ => match TermLogger::init(
            match &cli_args.flag_logLevel[..] {
                "info" => Ok(LevelFilter::Info),
                "warn" => Ok(LevelFilter::Warn),
                "debug" => Ok(LevelFilter::Debug),
                "error" => Ok(LevelFilter::Error),
                "trace" => Ok(LevelFilter::Trace),
                _ => Err(AppError::Custom(format!("✘ Not a valid log level: '{}'", cli_args.flag_logLevel)))
            }?,
            Config::default(),
            TerminalMode::Mixed,
        ) {
            Ok(_) => {
                info!("✔ Logger initialized successfully!");
                Ok(cli_args)
            },
            Err(e) => Err(AppError::Custom(e.to_string()))
        }
    }
}
