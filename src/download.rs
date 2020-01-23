use curl::easy::Easy;
use serde_json::Value;
use std::path::Path;
use std::future::Future;
use std::io::Write;
use duma::{utils, download};
use clap::{crate_version, ArgMatches};
use failure::{format_err, Fallible};
use std::env;

pub fn run() -> String {
    let mut easy = Easy::new();
    easy.url("https://dog.ceo/api/breeds/image/random").unwrap();

    let mut html: String = String::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            html = String::from_utf8(Vec::from(data)).unwrap();
            Ok(data.len())
        }).unwrap();

        transfer.perform().unwrap();
    }

    let html_json: Value = serde_json::from_str(&html).unwrap();
    let img_url = &html_json["message"].as_str().unwrap();

    let url = utils::parse_url(img_url).unwrap();
    let args = ArgMatches::default();

    let a = url.path_segments().unwrap().next_back().unwrap();
    let a = a.to_owned();
    println!("{:?}", a);

    download::http_download(url, &args, crate_version!());

    let path = env::current_dir().unwrap();
    String::from(format!("{}/{}", path.to_str().unwrap(), a))
}
