use std::collections::HashMap;

use serde_json::Value;

pub fn curl_parser(input: &String)-> String {
    // let mut temp_input = input.clone().replace("\\\n", "\\");
    let vect_input: Vec<&str> = input.split("\\").collect();
    let ret_url = vect_input[0];
    let mut method = String::from("GET");

    let url: Vec<&str> = ret_url.split("'").collect();
    if url[0].contains("--request") {

    } else {

    }

    if url.len() < 2 {
        return String::new();  
    }

    return url[1].to_string();
}

pub fn format_json_body(res: String)-> String {
        let formated_str = res
            .replace("\\n", "\r\n")
            .replace("\\\"", "\"");
        let trimmed = if formated_str.starts_with('"') && formated_str.ends_with('"') {
            formated_str.trim_matches('"').to_string()
        } else {
            formated_str
        };
        return trimmed;
}

pub fn json_string_to_hashmap(json_str: &str) ->HashMap<String, String> {
    let json_value: Value = serde_json::from_str(json_str).unwrap();
    
    let mut hashmap: HashMap<String, String> = HashMap::new();

    if let Value::Object(obj) = json_value {
        for (key, value) in obj {
            hashmap.insert(key, value.to_string());
        }
    }

    hashmap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_json_body() {
        let input = r#""{"key":"value"}""#;
        let expected_output = r#"{"key":"value"}"#;
        assert_eq!(format_json_body(input.to_string()), expected_output.to_string());
    }

    #[test]
    fn test_json_string_to_hashmap_empty() {
        let input = r#"{}"#;
        let expected_output = HashMap::new();

        assert_eq!(json_string_to_hashmap(input), expected_output);
    }
}