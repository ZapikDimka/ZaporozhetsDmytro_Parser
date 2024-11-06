use clap::{arg, command, Command};
use json_parser::{validate_json_schema, parse_partial_json, edit_json, convert_to_format, handle_large_json};
use serde_json::Value;
use std::fs;
use std::path::Path;
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();

    let matches = command!("Zaporozhets JSON Parser")
        .version("0.1.0")
        .about("Parses and processes JSON files with extended features")
        .subcommand(
            Command::new("validate")
                .about("Validates a JSON file against a schema")
                .arg(arg!(<input> "Path to the JSON file").required(true))
                .arg(arg!(<schema> "Path to the JSON schema file").required(true))
        )
        .subcommand(
            Command::new("parse-partial")
                .about("Parses only a specific part of the JSON file")
                .arg(arg!(<input> "Path to the JSON file").required(true))
                .arg(arg!(<key> "Key to parse from the JSON file").required(true))
        )
        .subcommand(
            Command::new("edit")
                .about("Edits a JSON file")
                .arg(arg!(<input> "Path to the JSON file").required(true))
                .arg(arg!(<key> "Key to edit").required(true))
                .arg(arg!(<value> "New value to set").required(true))
        )
        .subcommand(
            Command::new("convert")
                .about("Converts JSON to another format (yaml/xml)")
                .arg(arg!(<input> "Path to the JSON file").required(true))
                .arg(arg!(<format> "Target format (yaml/xml)").required(true))
        )
        .subcommand(
            Command::new("large-file")
                .about("Parses large JSON files in chunks")
                .arg(arg!(<input> "Path to the large JSON file").required(true))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("validate") {
        let input_path = matches.get_one::<String>("input").unwrap();
        let schema_path = matches.get_one::<String>("schema").unwrap();

        let json: Value = serde_json::from_str(&fs::read_to_string(input_path)?)?;
        let schema: Value = serde_json::from_str(&fs::read_to_string(schema_path)?)?;

        if validate_json_schema(&json, &schema).is_ok() {
            println!("JSON is valid against the schema.");
        } else {
            println!("JSON is not valid against the schema.");
        }
    }

    if let Some(matches) = matches.subcommand_matches("parse-partial") {
        let input_path = matches.get_one::<String>("input").unwrap();
        let key = matches.get_one::<String>("key").unwrap();

        let json: Value = serde_json::from_str(&fs::read_to_string(input_path)?)?;
        if let Some(part) = parse_partial_json(&json, key) {
            println!("Parsed part: {:#?}", part);
        } else {
            println!("Key not found in JSON.");
        }
    }

    if let Some(matches) = matches.subcommand_matches("edit") {
        let input_path = matches.get_one::<String>("input").unwrap();
        let key = matches.get_one::<String>("key").unwrap();
        let new_value: Value = serde_json::from_str(matches.get_one::<String>("value").unwrap())?;

        let mut json: Value = serde_json::from_str(&fs::read_to_string(input_path)?)?;
        edit_json(&mut json, key, new_value)?;

        fs::write(input_path, serde_json::to_string_pretty(&json)?)?;
        println!("JSON edited successfully.");
    }

    if let Some(matches) = matches.subcommand_matches("convert") {
        let input_path = matches.get_one::<String>("input").unwrap();
        let format = matches.get_one::<String>("format").unwrap();

        let json: Value = serde_json::from_str(&fs::read_to_string(input_path)?)?;
        match convert_to_format(&json, format) {
            Ok(output) => println!("Converted JSON:\n{}", output),
            Err(e) => println!("Error: {}", e),
        }
    }

    if let Some(matches) = matches.subcommand_matches("large-file") {
        let input_path = matches.get_one::<String>("input").unwrap();
        handle_large_json(Path::new(input_path))?;
        println!("Finished parsing large JSON file.");
    }

    Ok(())
}
