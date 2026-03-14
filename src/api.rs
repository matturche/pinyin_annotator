use gloo_net::http::Request;

use std::collections::HashMap;

use crate::utils::{HskDictEntry, HskVersion};

pub async fn fetch_hsk_dict(path: &str) -> HashMap<String, HskDictEntry> {
    let mut hsk_dict: HashMap<String, HskDictEntry> = HashMap::new();
    let resp = Request::get(&format!(
        "https://raw.githubusercontent.com/matturche/hskgrader/refs/heads/main/data/{}",
        path
    ))
    .send()
    .await
    .expect("Failed send request for hanzi pairs");
    let text = resp.text().await.expect("Failed to get text from response");
    let lines: Vec<String> = text.lines().map(str::to_owned).collect();
    for line in lines.iter() {
        let splits: Vec<&str> = line.split(',').collect();
        let characters = splits[0].to_string();
        let pinyin = splits[1].to_string();
        let level = splits[2].to_string().parse::<u32>().unwrap_or(0);
        hsk_dict.insert(characters, HskDictEntry { pinyin, level });
    }
    hsk_dict
}

pub async fn fetch_hsk_dict_and_extension(
    hsk_version: HskVersion,
    with_extension: bool,
) -> HashMap<String, HskDictEntry> {
    let mut dict: HashMap<String, HskDictEntry> = HashMap::new();
    let version_name = match hsk_version {
        HskVersion::Hsk20 => "hsk2-0",
        HskVersion::Hsk30 => "hsk3-0",
    };
    let hsk_dict = fetch_hsk_dict(&format!("new_{version_name}.csv")).await;
    dict.extend(hsk_dict.into_iter());
    if with_extension {
        let extansion_dict = fetch_hsk_dict("hsk_dict_expansion.csv").await;
        dict.extend(extansion_dict.into_iter());
    }
    dict
}
