use crate::crypto_utils::get_url;
use crate::r#type::PlayerMetadata;
use lazy_static::lazy_static;
use regex::Regex;
pub use reqwest::{Client, Proxy};
use std::fs::File;

pub struct Api {
    host: String,
    client: Client,
}

impl Api {
    pub fn builder() -> ApiBuilder {
        ApiBuilder::new()
    }

    pub fn host(&self) -> &str {
        self.host.as_str()
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn url(&self, uri: &str) -> String {
        format!("{}{}", self.host(), uri)
    }

    pub async fn metadata(&self, id: u64) -> Option<PlayerMetadata> {
        let uri = format!("/index.php/vod/play/id/{id}/sid/1/nid/1.html");
        let res = self
            .client
            .get(self.url(&uri))
            .send()
            .await
            .ok()?
            .text()
            .await
            .ok()?;
        let player_aaaa_matcher =
            regex::Regex::new(r".*var player_aaaa *= *(?P<data>\{.*\}).*?").ok()?;
        serde_json::from_str(player_aaaa_matcher.captures(&res)?.name("data")?.as_str()).ok()?
    }

    pub async fn metadata_index(&self, id: u64, nid: u64) -> Option<PlayerMetadata> {
        let uri = format!("/index.php/vod/play/id/{id}/sid/1/nid/{nid}.html");
        let res = self
            .client
            .get(self.url(&uri))
            .send()
            .await
            .ok()?
            .text()
            .await
            .ok()?;
        let player_aaaa_matcher =
            regex::Regex::new(r".*var player_aaaa *= *(?P<data>\{.*\}).*?").ok()?;
        serde_json::from_str(player_aaaa_matcher.captures(&res)?.name("data")?.as_str()).ok()?
    }

    pub fn unpack_url(&self, url: impl ToString) -> (u64, u64, u64) {
        lazy_static! {
            static ref unpack_url_matcher: Regex = Regex::new(r"(https?://.+)?/index.php/vod/play/id/(?<id>\d+)/sid/(?<sid>\d+)/nid/(?<nid>\d+)\.html.*").unwrap();
        }
        let url = url.to_string();
        (
            unpack_url_matcher
                .captures(&url)
                .unwrap()
                .name("id")
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap(),
            unpack_url_matcher
                .captures(&url)
                .unwrap()
                .name("sid")
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap(),
            unpack_url_matcher
                .captures(&url)
                .unwrap()
                .name("nid")
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap(),
        )
    }

    pub async fn download_url(&self, url: impl ToString) -> Option<String> {
        let res = self
            .client
            .get(url.to_string())
            .send()
            .await
            .ok()?
            .text()
            .await
            .ok()?;
        let player_aaaa_matcher =
            regex::Regex::new(r".*var player_aaaa *= *(?P<data>\{.*\}).*?").ok()?;
        let player_aaaa: PlayerMetadata =
            serde_json::from_str(player_aaaa_matcher.captures(&res)?.name("data")?.as_str())
                .ok()?;
        let url = "https://bf.sbdm.cc/m3u8.php";
        let res = self
            .client
            .get(url)
            .query(&[("url", &player_aaaa.url)])
            .header(reqwest::header::REFERER, self.host())
            .send()
            .await
            .ok()?
            .text()
            .await
            .ok()?;
        Some(player_aaaa.get_m3u8_url(res))
    }

    pub async fn download_url_form(&self, id: u64, sid: u64, nid: u64) -> Option<String> {
        let uri = format!("/index.php/vod/play/id/{id}/sid/{sid}/nid/{nid}.html");
        self.download_url(self.url(&uri)).await
    }
}

#[derive(Default)]
pub struct ApiBuilder {
    host: Option<String>,
    proxy: Option<Proxy>,
}

impl ApiBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn host(mut self, host: Option<&str>) -> Self {
        self.host = host.map(|v| v.to_string());
        self
    }

    pub fn build(self) -> Api {
        let mut client_builder = Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36");
        if let Some(proxy) = self.proxy {
            client_builder = client_builder.proxy(proxy);
        }
        Api {
            host: self.host.unwrap(),
            client: client_builder.build().unwrap(),
        }
    }

    pub fn proxy(mut self, v: Option<Proxy>) -> Self {
        self.proxy = v;
        self
    }
}
