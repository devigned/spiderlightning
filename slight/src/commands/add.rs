use anyhow::Result;

const DOWNLOAD_URL: &str = "https://raw.githubusercontent.com/deislabs/spiderlightning/main/wit/";

pub fn handle_add(interface_and_module: &str, auth_token: &Option<String>) -> Result<()> {
    todo!(
        "received '{}', and '{:?}' — but handle_add doesn't do anything yet",
        interface_and_module, auth_token
    );
}
