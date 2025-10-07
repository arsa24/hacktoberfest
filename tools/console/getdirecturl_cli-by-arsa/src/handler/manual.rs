use crate::provider::get_provider_aliases;
use crate::utils::{handle_provider, normalize_provider};

pub fn manual_handler(provider: &str, url: &str, download: bool) {
    let aliases = get_provider_aliases();

    match normalize_provider(provider, &aliases) {
        Some(normalized) => {
            match handle_provider(normalized, url) {
                Ok(result) => {
                    println!("✅ Direct URL ditemukan!");
                    println!("Title: {}", result.title);
                    println!("Direct URL: {}", result.url);

                    if download && result.download_support {
                        println!("⬇️  Mulai download dari: {}", result.url);
                        // TODO: panggil fungsi download_file(result.url)
                    } else if download {
                        println!("⚠️  Provider ini belum mendukung auto-download.");
                    }
                }
                Err(err) => eprintln!("❌ Gagal: {}", err),
            }
        }
        None => {
            eprintln!("❌ Provider '{}' tidak dikenal.", provider);
            println!("Provider yang tersedia:");
            for (key, alias) in &aliases {
                println!("  {} (alias: {})", key, alias.join(", "));
            }
        }
    }
}
