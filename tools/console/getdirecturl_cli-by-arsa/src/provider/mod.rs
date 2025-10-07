use std::collections::HashMap;
pub mod pixeldrain;
pub mod meganz;
pub mod krakenfile;

pub struct DirectResult {
    pub url: String,
    pub title: String,
    pub download_support: bool,
}

pub fn get_provider_aliases() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();
    map.insert("pixeldrain", vec!["pixeldrain", "pdrain", "pd"]);
    map.insert("meganz", vec!["meganz", "mega", "mn", "mz"]);
    map.insert("krakenfile", vec!["krakenfile", "kraken", "kf"]);

    map
}

