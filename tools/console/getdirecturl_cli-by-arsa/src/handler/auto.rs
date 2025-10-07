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
