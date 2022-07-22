use std::sync::{Arc, Mutex};

use anyhow::{bail, Result};

const DOWNLOAD_URL: &str = "https://raw.githubusercontent.com/deislabs/spiderlightning/main/wit";

use curl::easy::Easy;

pub fn handle_add(interface_and_version: &str, auth_token: &Option<String>) -> Result<()> {
    let (name, version) = separate_interface_and_version(interface_and_version)?;
    todo!(
        "want to download wit with name: '{}', and version: '{}', with auth_token: '{:?}' — but handle_add doesn't do anything yet",
        name,
        version,
        auth_token
    );
}

fn separate_interface_and_version(interface_and_version: &str) -> Result<(&str, &str)> {
    let mut vec_res = interface_and_version.split('@').collect::<Vec<&str>>();

    if !vec_res[0].chars().all(char::is_alphabetic) {
        // asumes no interfaces have numbers in their name
        bail!("'{}' doesn't match any known interfaces", vec_res[0]);
    }

    // vvv only name matched
    if vec_res.len() == 1 {
        vec_res.push("latest"); // indicate we want to use latest version
    }

    if vec_res[0].is_empty() || vec_res[1].is_empty() {
        bail!(
            "'{}' is not a valid input — do: `slight add --help` to see usage information",
            interface_and_version
        );
    }

    Ok((vec_res[0], vec_res[1]))
}

fn download_wit(name: &str, version: &str, token: &Option<String>) -> Result<String> {
    let wit = Arc::new(Mutex::new(String::new()));
    let mut easy = Easy::new();
    if let Some(token) = token {
        easy.url(&format!(
            "{}/{}.wit?token={}",
            DOWNLOAD_URL, /*version,*/ name, token
        ))?;
    } else {
        easy.url(&format!("{}/{}.wit", DOWNLOAD_URL, /*version,*/ name))?;
    }

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        *wit.lock().unwrap() = String::from_utf8(data.to_vec()).unwrap();
        Ok(data.len())
    })?;
    transfer.perform().unwrap();
    Ok(wit.clone().lock().unwrap().to_string())
}

#[cfg(test)]
mod unittests {
    use std::env;

    use super::{download_wit, separate_interface_and_version};
    use anyhow::Result;

    #[test]
    fn separate_interface_and_version_test() -> Result<()> {
        assert!(separate_interface_and_version("v0.1").is_err());
        assert!(separate_interface_and_version("@").is_err());
        assert!(separate_interface_and_version("@v0.1").is_err());
        assert_eq!(("kv", "latest"), separate_interface_and_version("kv")?);
        assert!(separate_interface_and_version("kvv0.1").is_err());
        assert!(separate_interface_and_version("kv@").is_err());
        assert_eq!(("kv", "v0.1"), separate_interface_and_version("kv@v0.1")?);
        Ok(())
    }

    #[test]
    fn download_wit_test() -> Result<()> {
        dbg!(download_wit(
            "kv",
            "v0.1",
            &Some(env::var("GITHUB_TOKEN")?)
        )?);
        Ok(())
    }
}
