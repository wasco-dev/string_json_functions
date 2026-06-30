wit_bindgen::generate!({ generate_all });

use crate::exports::betty_blocks::string_json_functions::string_json::Guest;

struct Component;

impl Guest for Component {
    fn stringify(value: String) -> String {
        value
    }

    fn parse_json(input: String) -> String {
        input
    }

    fn form_urlencoded_parse(input: String) -> String {
        let map: std::collections::HashMap<String, String> =
            form_urlencoded::parse(input.as_bytes())
                .into_owned()
                .collect();
        serde_json::to_string(&map).unwrap_or_default()
    }
}

export! {Component}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_urlencoded_parse() {
        let input = "command=%2Fvraag-ai&text=Schrijf+een+kort+gedicht&response_url=https%3A%2F%2Fhooks.slack.com";
        let result = Component::form_urlencoded_parse(input.to_string());
        let expected = serde_json::json!({
            "command": "/vraag-ai",
            "text": "Schrijf een kort gedicht",
            "response_url": "https://hooks.slack.com"
        });
        let result_val: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(result_val, expected);
    }
}
