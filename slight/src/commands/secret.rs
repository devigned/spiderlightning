use anyhow::Result;
use spiderlightning::core::{secret::create_secret, slightfile::Slightfile};
use std::fs::File;

pub fn handle_secret(
    key: &str,
    value: &str,
    toml: &mut Slightfile,
    toml_file: &mut File,
) -> Result<()> {
    toml.secret_store = Some("usersecrets_configs".to_string());
    create_secret(key, value, toml, toml_file)
}
