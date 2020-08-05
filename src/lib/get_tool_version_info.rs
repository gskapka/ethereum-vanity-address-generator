use serde_json::{
    json,
    Value as JsonValue,
};
use crate::lib::{
    types::Result,
    constants::TOOL_VERSION,
};

pub fn get_tool_version_info() -> Result<JsonValue> {
    Ok(json!({"version": TOOL_VERSION }))
}
