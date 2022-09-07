use crate::lib::{ethereum_keys::EthereumKeys, types::Result};
use serde_json::Value as JsonValue;

pub fn generate_vanity_address(prefix_hex: String) -> Result<JsonValue> {
    Ok(EthereumKeys::new_vanity_address(prefix_hex)?.to_json())
}
