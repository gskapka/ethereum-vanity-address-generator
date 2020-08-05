use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Custom(String),
    CryptoError(secp256k1::Error),
    RecvError(std::sync::mpsc::RecvError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            AppError::Custom(ref msg) => msg.to_string(),
            AppError::RecvError(ref e) => format!("✘ Recv error: {}", e),
            AppError::CryptoError(ref e) => format!("✘ Crypto Error: {}", e),

        };
        f.write_fmt(format_args!("{}", msg))
    }
}

impl From<secp256k1::Error> for AppError {
    fn from(e: secp256k1::Error) -> AppError {
        AppError::CryptoError(e)
    }
}

impl From<std::sync::mpsc::RecvError> for AppError {
    fn from(e: std::sync::mpsc::RecvError) -> AppError {
        AppError::RecvError(e)
    }
}
