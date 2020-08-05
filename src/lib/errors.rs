use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Custom(String),
    CryptoError(secp256k1::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            AppError::Custom(ref msg) => msg.to_string(),
            AppError::CryptoError(ref e) => format!("✘ Crypto Error!\n✘ {}", e),

        };
        f.write_fmt(format_args!("{}", msg))
    }
}

impl From<secp256k1::Error> for AppError {
    fn from(e: secp256k1::Error) -> AppError {
        AppError::CryptoError(e)
    }
}
