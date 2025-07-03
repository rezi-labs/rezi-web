use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
}

impl User {
    pub fn new(id: String, email: String) -> Self {
        User { id, email }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn initials(&self) -> String {
        let mut split = self.email().split(".");
        let first_name = split.next().unwrap_or("Unknown");
        let last_name = split.next().unwrap_or("User");
        format!(
            "{}{}",
            first_name.chars().next().unwrap_or(' '),
            last_name.chars().next().unwrap_or(' ')
        )
    }
}

pub fn decode_jwt_unsafe(token: &str) -> Result<User, String> {
    // Split the JWT into its three parts
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid JWT format: must have 3 parts".to_string());
    }

    // Decode the payload (second part)
    let payload = parts[1];

    // Add padding if needed for base64 decoding
    let payload_padded = add_base64_padding(payload);

    // Decode from base64
    let decoded_bytes =
        base64_decode(&payload_padded).map_err(|_| "Failed to decode base64 payload")?;

    // Convert to string
    let payload_str = String::from_utf8(decoded_bytes).map_err(|_| "Invalid UTF-8 in payload")?;

    // Parse JSON to extract user fields
    let json_value: serde_json::Value =
        serde_json::from_str(&payload_str).map_err(|e| format!("Failed to parse JSON: {e}"))?;

    // Extract id and email from the payload
    let id = json_value
        .get("id")
        .and_then(|v| v.as_str())
        .or_else(|| json_value.get("sub").and_then(|v| v.as_str()))
        .ok_or("Missing 'id' or 'sub' field in JWT payload")?
        .to_string();

    let email = json_value
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'email' field in JWT payload")?
        .to_string();

    Ok(User { id, email })
}

fn add_base64_padding(input: &str) -> String {
    let mut padded = input.to_string();
    while padded.len() % 4 != 0 {
        padded.push('=');
    }
    padded
}

fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    // Simple base64 decoder implementation
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut char_map = HashMap::new();

    for (i, c) in chars.chars().enumerate() {
        char_map.insert(c, i as u8);
    }

    let input = input.replace("-", "+").replace("_", "/");
    let mut result = Vec::new();
    let mut buffer = 0u32;
    let mut bits = 0;

    for c in input.chars() {
        if c == '=' {
            break;
        }

        let value = char_map.get(&c).ok_or("Invalid base64 character")?;
        buffer = (buffer << 6) | (*value as u32);
        bits += 6;

        if bits >= 8 {
            result.push((buffer >> (bits - 8)) as u8);
            bits -= 8;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_decode() {
        let encoded = "eyJpZCI6IjEyMyIsImVtYWlsIjoidGVzdEBleGFtcGxlLmNvbSJ9";
        let decoded = base64_decode(encoded).unwrap();
        let json_str = String::from_utf8(decoded).unwrap();
        assert_eq!(json_str, r#"{"id":"123","email":"test@example.com"}"#);
    }

    #[test]
    fn test_decode_jwt_unsafe() {
        // This is a sample JWT with payload: {"id":"123","email":"test@example.com"}
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjEyMyIsImVtYWlsIjoidGVzdEBleGFtcGxlLmNvbSJ9.signature";
        let user = decode_jwt_unsafe(token).unwrap();
        assert_eq!(user.id, "123");
        assert_eq!(user.email, "test@example.com");
    }
}
