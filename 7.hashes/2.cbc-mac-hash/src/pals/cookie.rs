use std::collections::HashMap;

pub fn parse(cookie: &str) -> HashMap<&str, &str> {
    let mut result = HashMap::new();

    for pair in cookie.split('&') {
        let parts: Vec<&str> = pair.split('=').collect();

        if parts.len() == 2 {
            result.insert(parts[0], parts[1]);
        }
    }

    result
}
