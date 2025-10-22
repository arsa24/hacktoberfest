use crate::{provider, utils};

pub fn auto_handler(url: &str, download: bool) {
    let result = match utils::get_provider_name(url) {
        Ok(provider) => match provider.as_str() {
            "krakenfiles" => provider::krakenfile::parse(url),
            "mega" => provider::meganz::handle(url),
            "pixeldrain" => provider::pixeldrain::handle(url),
            // "otakufiles" => provider::DirectResult::odfile(url),
            _ => Err(anyhow::anyhow!("")),
        },
        Err(e) => Err(anyhow::anyhow!(
            "Failed to detect provider for {}: {:?}",
            url,
            e
        )),
    };

    match result {
        Ok(result) => {
            println!("✅ Direct URL ditemukan!");
            println!("Title: {}", result.title);
            println!("Direct URL: {}", result.url);

            if download && result.download_support {
                println!("⬇️  Mulai download dari: {}", result.url);
                // TODO: panggil fungsi download_file(result.url) 
                // nanti kalau gak malas
            } else if download {
                println!("⚠️  Provider ini belum mendukung auto-download.");
            }
        }
        Err(err) => eprintln!("❌ Gagal: {}", err),
    };
}

pub fn get_provider_name(url: &str) -> Result<String, String> {
    let parsed_url = Url::parse(url).map_err(|e| e.to_string())?;
    let domain = parsed_url.domain().ok_or("No domain found")?;
    let parts: Vec<&str> = domain.split('.').collect();

    // Heuristic: return the second-level domain when available (e.g. "www.pixeldrain.com" -> "pixeldrain")
    let provider = if parts.len() >= 2 {
        parts[parts.len() - 2]
    } else {
        parts[0]
    };

    Ok(provider.to_lowercase())
}
