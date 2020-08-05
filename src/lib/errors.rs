use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            AppError::Custom(ref msg) => msg.to_string(),
        };
        f.write_fmt(format_args!("{}", msg))
    }
}
