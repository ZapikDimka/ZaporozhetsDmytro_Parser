use anyhow::{Result, Error};
use serde_json::{Value, Deserializer};
use log::{info, error};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("File read error: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Schema validation failed")]
    SchemaValidationError,
}
pub fn parse_json(json_str: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(json_str)
}

pub fn validate_json_schema(json: &Value, schema: &Value) -> Result<(), ParserError> {
    if json.is_object() && schema.is_object() {
        if json.as_object().unwrap().keys().all(|key| schema.as_object().unwrap().contains_key(key)) {
            Ok(())
        } else {
            Err(ParserError::SchemaValidationError)
        }
    } else {
        Err(ParserError::SchemaValidationError)
    }
}

pub fn parse_partial_json(json: &Value, key: &str) -> Option<Value> {
    json.get(key).cloned()
}

pub fn edit_json(json: &mut Value, key: &str, new_value: Value) -> Result<(), Error> {
    if let Some(obj) = json.as_object_mut() {
        obj.insert(key.to_string(), new_value);
        Ok(())
    } else {
        Err(Error::msg("Invalid JSON structure for editing"))
    }
}

pub fn convert_to_format(json: &Value, format: &str) -> Result<String, Error> {
    match format {
        "yaml" => Ok(serde_yaml::to_string(json).unwrap()),
        "xml" => Ok(quick_xml::se::to_string(json).unwrap()),
        _ => Err(Error::msg("Unsupported format")),
    }
}

pub fn handle_large_json(file_path: &Path) -> Result<(), ParserError> {
    let file = fs::File::open(file_path)?;
    let stream = Deserializer::from_reader(file).into_iter::<Value>();

    for value in stream {
        match value {
            Ok(json_value) => info!("Parsed chunk: {:?}", json_value),
            Err(e) => error!("Error parsing chunk: {:?}", e),
        }
    }
    Ok(())
}
