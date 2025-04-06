use std::env;

pub fn get_domain() -> String {
    env::var("DOMAIN").unwrap_or_else(|_| "http://localhost:8080".to_string())
}