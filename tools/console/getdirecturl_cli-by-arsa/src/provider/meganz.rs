use super::DirectResult;
use aes::Aes128;
use anyhow::anyhow;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use cbc::cipher::{BlockDecryptMut, KeyIvInit, block_padding::NoPadding};
use serde::Deserialize;
use std::str;

#[derive(Deserialize)]
struct MegaResponse {
    g: String,
    at: String,
}

fn xor_hex(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(x, y)| x ^ y).collect()
}

type Aes128CbcDec = cbc::Decryptor<Aes128>;

pub fn handle(url: &str) -> anyhow::Result<DirectResult> {
    let parts: Vec<&str> = url.split('#').collect();
    let id = parts
        .get(0)
        .and_then(|p| p.split("/file/").nth(1))
        .ok_or_else(|| anyhow!("Invalid MEGA URL: missing id"))?;
    let key = parts
        .get(1)
        .ok_or_else(|| anyhow!("Invalid MEGA URL: missing key"))?;

    let raw_key = URL_SAFE_NO_PAD
        .decode(key)
        .map_err(|e| anyhow!("Failed to decode base64 key: {}", e))?;
    if raw_key.len() < 32 {
        return Err(anyhow!("Invalid key length"));
    }

    let aes_key = xor_hex(&raw_key[0..16], &raw_key[16..32]);

    let body = format!(r#"[{{"a":"g","g":"1","p":"{}"}}]"#, id);

    // let resp = utils::HTTP_CLIENT
    //     .post("https://g.api.mega.co.nz/cs?id=&ak=")
    //     .body(body)
    //     .send()
    //     .await?
    //     .json::<Vec<MegaResponse>>()
    //     .await?;
    let resp = ureq::post("https://g.api.mega.co.nz/cs?id=&ak=")
        .send(body)?
        .body_mut()
        .read_json::<Vec<MegaResponse>>()?;

    let file_url = resp
        .get(0)
        .ok_or_else(|| anyhow!("Empty response from MEGA"))?
        .g
        .clone();
    let at = &resp[0].at;

    let mut data_decode = URL_SAFE_NO_PAD
        .decode(at)
        .map_err(|e| anyhow!("Failed to decode attribute: {}", e))?;

    let iv = [0u8; 16];
    let cipher = Aes128CbcDec::new_from_slices(&aes_key, &iv)
        .map_err(|e| anyhow!("Invalid key/iv length: {}", e))?;
    let plain_text_bytes = cipher
        .decrypt_padded_mut::<NoPadding>(&mut data_decode)
        .map_err(|_| anyhow!("Decryption error"))?;

    let mut plain_text = str::from_utf8(plain_text_bytes)
        .map_err(|e| anyhow!("Invalid UTF-8: {}", e))?
        .trim_end_matches('\u{0}')
        .to_string();

    if plain_text.len() > 4 {
        plain_text = plain_text[4..].to_string();
    }

    let meta: serde_json::Value =
        serde_json::from_str(&plain_text).map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;
    let file_name = meta["n"].as_str().unwrap_or("").to_string();

    Ok(DirectResult {
        url: file_url,
        title: file_name,
        download_support: true,
    })
}
