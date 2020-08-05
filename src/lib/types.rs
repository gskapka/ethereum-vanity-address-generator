use std::result;
use crate::lib::errors::AppError;

pub type Result<T> = result::Result<T, AppError>;
