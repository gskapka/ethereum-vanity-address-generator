use serde_json::Value as JsonValue;
use crate::lib::{
    types::Result,
    ethereum_keys::EthereumKeys,
};

pub fn generate_vanity_address(prefix_hex: String) -> Result<JsonValue> {
    Ok(EthereumKeys::new_vanity_address(prefix_hex)?.to_json())
}
