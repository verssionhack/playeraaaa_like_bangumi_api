pub mod api;
pub mod crypto_utils;
pub mod r#type;

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Proxy;
    use std::fs;

    #[test]
    fn test_url() {
        let event_loop = tokio::runtime::Runtime::new().unwrap();
        let api = api::Api::builder()
            .proxy(Some(Proxy::all("http://localhost:15777").unwrap()))
            .host(Some("https://mikudm.com"))
            .build();
        event_loop.block_on(async {
            for i in 1..=23 {
                let file_name = format!("/home/kurumin/tmp/playeraaaa_test_dir/{}.m3u8", i);
                let res = loop {
                    let url = api.download_url_form(1444, 1, i).await.unwrap();
                    println!("[{i}]{}", &url);
                    let txt = api
                        .client()
                        .get(&url)
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    if txt.len() > 0 {
                        break txt;
                    } else {
                        eprintln!("Retry {} {}", i, &url);
                    }
                };
                fs::write(file_name, res).unwrap();
            }
        });
    }

    //#[test]
    fn test_metadata() {
        let event_loop = tokio::runtime::Runtime::new().unwrap();
        let api = api::Api::builder()
            .proxy(Some(Proxy::all("http://localhost:15777").unwrap()))
            .host(Some("https://www.lldm.net"))
            .build();
        //println!("{:?}", api.unpack_url("https://www.lldm.net/index.php/vod/play/id/1617/sid/1/nid/30.html#"));
        event_loop.block_on(async {
            let mut metadata = Some(api.metadata(2549).await.unwrap());
            let mut link = metadata.unwrap().link_next;
            loop {
                if let Some(link_next) = link {
                    println!("{}", &link_next);
                    let (id, sid, nid) = api.unpack_url(link_next);
                    let download_url = api.download_url_form(id, sid, nid).await;
                    println!("{:?}", download_url);
                    metadata = Some(api.metadata_index(2549, nid).await.unwrap());
                    link = metadata.unwrap().link_next;
                } else {
                    break;
                }
            }
        });
    }
}
