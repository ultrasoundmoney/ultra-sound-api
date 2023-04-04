//! Fns to read variables from the environment more conveniently and help other functions figure
//! out what environment they're running in.

use std::env;

use tracing::debug;

const SECRET_LOG_BLACKLIST: [&str; 3] = ["DATABASE_URL", "OPSGENIE_API_KEY", "ETHERSCAN_API_KEY"];

/// Get an environment variable, encoding found or missing as Option, and panic otherwise.
pub fn get_env_var(key: &str) -> Option<String> {
    let var = match env::var(key) {
        Err(env::VarError::NotPresent) => None,
        Err(err) => panic!("{}", err),
        Ok(var) => Some(var),
    };

    if let Some(ref existing_var) = var {
        if SECRET_LOG_BLACKLIST.contains(&key) {
            let mut last_four = existing_var.clone();
            last_four.drain(0..existing_var.len() - 4);
            debug!("env var {key}: ****{last_four}")
        } else {
            debug!("env var {key}: {existing_var}");
        }
    } else {
        debug!("env var {key} requested but not found")
    };

    var
}

/// Get an environment variable we can't run without.
pub fn get_env_var_unsafe(key: &str) -> String {
    get_env_var(key).unwrap_or_else(|| panic!("{key} should be in env"))
}
