use std::env;

pub fn get_username() -> String {

    env::var("USER").unwrap_or("unknown".to_string())
}
