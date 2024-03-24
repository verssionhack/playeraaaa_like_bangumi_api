use serde::{Deserialize, Serialize, Deserializer};
use reqwest::blocking::ClientBuilder;
use crate::crypto_utils::get_url;

fn de_from_string_to_opstring<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where D: Deserializer<'de>
{
    let v = String::deserialize(deserializer)?;
    Ok(if v.len() != 0 { Some(v) } else { None })
}



#[derive(Deserialize, Serialize, Debug)]
pub struct PlayerMetadata {
    pub flag: String,
    pub encrypt: u64,
    pub trysee: u64,
    pub points: u64,
    pub link: String,
    #[serde(deserialize_with = "de_from_string_to_opstring")]
    pub link_next: Option<String>,
    #[serde(deserialize_with = "de_from_string_to_opstring")]
    pub link_pre: Option<String>,
    pub url: String,
    #[serde(deserialize_with = "de_from_string_to_opstring")]
    pub url_next: Option<String>,
    pub from: String,
    pub server: String,
    pub note: String,
    pub id: String,
    pub sid: u64,
    pub nid: u64,
}

impl PlayerMetadata {
    pub fn get_m3u8_url(&self, txt: String) -> String {
        let getvideo_matcher = regex::Regex::new(r#"getVideoInfo\("(?<m3u8_url>.*)"\)"#).unwrap();
        let token_iv_matcher = regex::Regex::new("var le_token *= *\"(?<token_iv>.*)\";").unwrap();
        let url = getvideo_matcher.captures(&txt)
            .unwrap().name("m3u8_url").unwrap();
        let token_key = b"A42EAC0C2B408472";
        let token_iv = token_iv_matcher.captures(&txt)
            .unwrap().name("token_iv").unwrap();
        get_url(url.as_str(), token_key, token_iv.as_str().as_bytes()).unwrap()
    }
}
