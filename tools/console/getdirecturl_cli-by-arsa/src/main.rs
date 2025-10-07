mod provider;
mod utils;
use clap::{Arg, Command};
use provider::get_provider_aliases;
use utils::normalize_provider;
mod handler;

fn main() {
    let cmd = Command::new("getdirecturl")
        .version("0.1.0")
        .author("arsa")
        .about("Tools CLI untuk mendapatkan direct URL dari web cloud storage")
        .subcommand(Command::new("provider").about("Menampilkan list provider yang didukung"))
        .subcommand(
            Command::new("manual")
                .about("Mode manual (tentukan provider & url)")
                .arg(
                    Arg::new("provider")
                        .long("provider")
                        .short('p')
                        .help("Nama provider (contoh: pixeldrain)")
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("url")
                        .long("url")
                        .short('u')
                        .help("URL web yang ingin didapatkan direct URL-nya")
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("download")
                        .long("download")
                        .short('d')
                        .help("Download file setelah mendapatkan direct URL")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("auto")
                .about("Mode auto (URL saja)")
                .arg(
                    Arg::new("url")
                        .long("url")
                        .short('u')
                        .help("URL web yang ingin didapatkan direct URL-nya")
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("download")
                        .long("download")
                        .short('d')
                        .help("Download file setelah mendapatkan direct URL")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .get_matches();
    let provider_aliases = get_provider_aliases();

    match cmd.subcommand() {
        Some(("manual", m)) => {
            let input_provider = m.get_one::<String>("provider").unwrap();
            let url = m.get_one::<String>("url").unwrap();
            let download = m.get_flag("download");

            match normalize_provider(input_provider, &provider_aliases) {
                Some(provider) => handler::manual::manual_handler(provider, url, download),
                None => {
                    eprintln!("❌ Provider '{}' tidak dikenal.", input_provider);
                    println!("Provider yang tersedia:");
                    for (key, aliases) in &provider_aliases {
                        println!("  {} (alias: {})", key, aliases.join(", "));
                    }
                }
            }
        }

        Some(("auto", a)) => {
            let url = a.get_one::<String>("url").unwrap();
            let download = a.get_flag("download");

            let _ = handler::auto::auto_handler(url, download);
        }

        Some(("provider", _)) => {
            println!("Provider yang didukung:");
            for (key, aliases) in &get_provider_aliases() {
                println!("  {} (alias: {})", key, aliases.join(", "));
            }
        }

        _ => println!("Gunakan `--help` untuk melihat perintah yang tersedia."),
    }
}
