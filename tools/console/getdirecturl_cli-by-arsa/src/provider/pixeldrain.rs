use super::DirectResult;
use serde::Deserialize;

#[derive(Deserialize)]
struct PixelDrainInfo {
    name: String,
}

pub fn handle(url: &str) -> anyhow::Result<DirectResult> {
    let id = url.split("/u/").nth(1).unwrap_or_default();
    if id.is_empty() {
        return Err(anyhow::anyhow!("URL Pixeldrain tidak valid"));
    }

    let api_url = format!("https://pixeldrain.com/api/file/{}/info", id);
    let mut resp = ureq::get(&api_url).call()?;
    let info: PixelDrainInfo = resp.body_mut().read_json::<PixelDrainInfo>()?;

    Ok(DirectResult {
        url: format!("https://pixeldrain.com/api/file/{}", id),
        title: info.name,
        download_support: true,
    })
}
