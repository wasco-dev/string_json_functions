wit_bindgen::generate!({ generate_all });

use crate::exports::wasco_dev::string_json_functions::string_json::Guest;

struct Component;

impl Guest for Component {
    fn stringify(value: String) -> String {
        value
    }

    fn parse_json(input: String) -> String {
        let value: serde_json::Value = serde_json::from_str(&input).unwrap_or_default();
        serde_json::to_string(&value).unwrap_or_default()
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

    #[test]
    fn test_parse_json() {
        let input = r#"{
  "body": null,
  "headers": [
    [
      "accept",
      "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"
    ],
    [
      "accept-encoding",
      "gzip, deflate, br, zstd"
    ],
    [
      "accept-language",
      "en-GB,en;q=0.9,en-US;q=0.8,nl;q=0.7"
    ],
    [
      "cookie",
      "i18next=en-GB"
    ],
    [
      "host",
      "nda-agent.betty.app"
    ],
    [
      "priority",
      "u=0, i"
    ],
    [
      "sec-ch-ua",
      "\"Google Chrome\";v=\"149\", \"Chromium\";v=\"149\", \"Not)A;Brand\";v=\"24\""
    ],
    [
      "sec-ch-ua-mobile",
      "?0"
    ],
    [
      "sec-ch-ua-platform",
      "\"macOS\""
    ],
    [
      "sec-fetch-dest",
      "document"
    ],
    [
      "sec-fetch-mode",
      "navigate"
    ],
    [
      "sec-fetch-site",
      "none"
    ],
    [
      "sec-fetch-user",
      "?1"
    ],
    [
      "upgrade-insecure-requests",
      "1"
    ],
    [
      "user-agent",
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36"
    ],
    [
      "x-forwarded-for",
      "95.99.119.82"
    ],
    [
      "x-forwarded-host",
      "nda-agent.betty.app"
    ],
    [
      "x-forwarded-port",
      "443"
    ],
    [
      "x-forwarded-proto",
      "https"
    ],
    [
      "x-forwarded-scheme",
      "https"
    ],
    [
      "x-real-ip",
      "95.99.119.82"
    ],
    [
      "x-request-id",
      "ead29731ffd25d6484a5589507b0ce97"
    ],
    [
      "x-scheme",
      "https"
    ],
    [
      "bb-request-id",
      "7Fk2P2OwDclzKUr_UC8Iuw=="
    ]
  ],
  "method": "GET",
  "path": [],
  "query": []
}"#;
        let result = Component::parse_json(input.to_string());
        let input_val: serde_json::Value = serde_json::from_str(input).unwrap();
        let result_val: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(result_val, input_val);
    }
}
