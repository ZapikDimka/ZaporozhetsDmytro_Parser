use json_parser::{parse_json, validate_json_schema, parse_partial_json, edit_json, convert_to_format, handle_large_json};

#[cfg(test)]

mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_parse_simple_json() {
        let json_data = r#"{ "name": "John", "age": 30, "city": "New York" }"#;
        assert!(parse_json(json_data).is_ok());
    }

    #[test]
    fn test_validate_json_schema() {
        let json_data = json!({ "name": "John", "age": 30 });
        let schema = json!({ "name": "", "age": 0 });
        let result = validate_json_schema(&json_data, &schema);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_partial_json() {
        let json_data = json!({ "name": "John", "age": 30, "city": "New York" });
        let result = parse_partial_json(&json_data, "city");
        assert_eq!(result, Some(json!("New York")));
    }

    #[test]
    fn test_edit_json() {
        let mut json_data = json!({ "name": "John", "age": 30 });
        let result = edit_json(&mut json_data, "age", json!(35));
        assert!(result.is_ok());
        assert_eq!(json_data["age"], 35);
    }

    #[test]
    fn test_convert_to_yaml() {
        let json_data = json!({ "name": "John", "age": 30 });
        let result = convert_to_format(&json_data, "yaml");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("name: John"));
    }

    #[test]
    fn test_handle_large_json() {
        // Create a temporary large JSON file
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("large_test.json");
        let mut file = File::create(&file_path).unwrap();
        write!(file, "[{}]", (0..1000).map(|_| "{\"key\": \"value\"}").collect::<Vec<_>>().join(",")).unwrap();

        let result = handle_large_json(&file_path);
        assert!(result.is_ok());
    }
}
