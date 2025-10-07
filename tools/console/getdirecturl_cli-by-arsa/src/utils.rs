use std::collections::HashMap;

use url::Url;

use crate::provider::{krakenfile, meganz, pixeldrain, DirectResult};

pub fn normalize_provider<'a>(
    input: &str,
    aliases: &'a HashMap<&'a str, Vec<&'a str>>,
) -> Option<&'a str> {
    for (main, list) in aliases {
        if list.iter().any(|&alias| alias.eq_ignore_ascii_case(input)) {
            return Some(main);
        }
    }
    None
}

pub fn handle_provider(provider: &str, url: &str) -> anyhow::Result<DirectResult> {
    match provider {
        "pixeldrain" => pixeldrain::handle(url),
        "meganz" => meganz::handle(url),
        "krakenfile" => krakenfile::parse(url),
        
        _ => Err(anyhow::anyhow!("Provider '{}' belum didukung", provider)),
    }
}

pub fn get_provider_name(url: &str) -> Result<String, String> {
    let parsed_url = Url::parse(url).map_err(|e| e.to_string())?;
    let domain = parsed_url.domain().ok_or("")?;
    let provider = domain.split('.').next().ok_or("")?;
    Ok(provider.to_string())
}
