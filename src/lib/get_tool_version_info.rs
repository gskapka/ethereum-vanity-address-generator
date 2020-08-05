use crate::lib::{
    types::Result,
    constants::TOOL_VERSION,
};

pub fn get_tool_version_info() -> Result<String> {
    Ok(TOOL_VERSION.to_string())
}
