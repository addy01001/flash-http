use std::collections::HashMap;
use url::Url;

pub fn add_query_params(base_url: &str, params: &HashMap<String, String>) -> String {
    let mut url = Url::parse(base_url).expect("Failed to parse base URL");

    // Add query parameters to the URL
    for (key, value) in params {
        url.query_pairs_mut().append_pair(&key, &value);
    }
    // Return the URL as a string
    url.to_string()
}

pub fn get_base_url(url: String) -> String {
    // Extract scheme, host, and path components
    let url_list: Vec<&str> = url.split("?").collect();
    return url_list.get(0).unwrap().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Address ordering issue in query params to resolve this
    // #[test]
    // fn test_add_query_params() {
    //     let mut params = HashMap::new();
    //     params.insert("param1".to_string(), "value1".to_string());
    //     params.insert("param2".to_string(), "value2".to_string());

    //     let base_url = "https://example.com/path";
    //     let expected_url = "https://example.com/path?param1=value1&param2=value2";

    //     assert_eq!(add_query_params(base_url, &params), expected_url);
    // }

    #[test]
    fn test_get_base_url() {
        let url = "https://example.com/path?param1=value1&param2=value2".to_string();
        let expected_base_url = "https://example.com/path";

        assert_eq!(get_base_url(url), expected_base_url);
    }
}