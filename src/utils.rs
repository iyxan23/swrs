use json::JsonValue;
use crate::error::{ParseError, SWRSError, SWRSResult};

/// A simple function that returns Err if a key doesn't exist
pub fn get_and_check(json: &JsonValue, key: &str) -> SWRSResult<JsonValue> {
    if !json.has_key(key) {
        Err(SWRSError::ParseError(ParseError {
            title: format!("Malformed JSON"),
            description: format!("Missing key {} on json: {}", key, json.to_string())
        }))
    } else {
        Ok(json[&key])
    }
}