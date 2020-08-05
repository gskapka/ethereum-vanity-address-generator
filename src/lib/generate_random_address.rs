use serde_json::Value as JsonValue;
use crate::lib::{
    types::Result,
    ethereum_keys::EthereumKeys,
};

pub fn generate_random_address() -> Result<JsonValue> {
    Ok(EthereumKeys::new_random_key()?.to_json())
}
