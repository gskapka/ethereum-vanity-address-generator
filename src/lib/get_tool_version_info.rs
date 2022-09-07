use crate::lib::{constants::TOOL_VERSION, types::Result};
use serde_json::{json, Value as JsonValue};

pub fn get_tool_version_info() -> Result<JsonValue> {
    Ok(json!({ "version": TOOL_VERSION }))
}
