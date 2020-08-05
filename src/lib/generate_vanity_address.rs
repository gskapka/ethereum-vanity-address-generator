use crate::lib::types::Result;
use serde_json::{
    json,
    Value as JsonValue,
};

pub fn generate_vanity_address(hex: &str) -> Result<JsonValue> {
    Ok(json!({"some":"json"}))
}
