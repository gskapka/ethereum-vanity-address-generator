use crate::lib::{ethereum_keys::EthereumKeys, types::Result};
use serde_json::Value as JsonValue;

pub fn generate_random_address() -> Result<JsonValue> {
    Ok(EthereumKeys::new_random_address()?.to_json())
}
