use super::DirectResult;
use scraper::{Html, Selector};

pub fn parse(url: &str) -> anyhow::Result<DirectResult> {
    let mut response = ureq::get(url).call()?;
    let text = response.body_mut().read_to_string()?;

    let docs = Html::parse_document(&text);
    let url_selector = Selector::parse("video#my-video").unwrap();
    let title_selector = Selector::parse("span.coin-name h5").unwrap();

    let direct_url = docs
        .select(&url_selector)
        .next()
        .and_then(|el| el.attr("data-src-url"))
        .unwrap_or_default()
        .to_string();

    let title = docs
        .select(&title_selector)
        .next()
        .map(|n| n.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    Ok(DirectResult {
        url: direct_url,
        title: title,
        download_support: true,
    })
}
