use curl::easy;
use serde_json;
use duma;
use std::{env, string, error};
use clap;
use failure;
use core::fmt;
use url;

#[derive(Debug, Clone)]
pub struct DownloadError;

impl error::Error for DownloadError {}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Download error")
    }
}

impl From<curl::Error> for DownloadError {
    fn from(_: curl::Error) -> Self {
        DownloadError
    }
}

impl From<string::FromUtf8Error> for DownloadError {
    fn from(_: string::FromUtf8Error) -> Self {
        DownloadError
    }
}

impl From<curl::easy::WriteError> for DownloadError {
    fn from(_: curl::easy::WriteError) -> Self {
        DownloadError
    }
}

impl From<std::io::Error> for DownloadError {
    fn from(_: std::io::Error) -> Self {
        DownloadError
    }
}

impl From<serde_json::error::Error> for DownloadError {
    fn from(_: serde_json::error::Error) -> Self {
        DownloadError
    }
}

impl From<failure::Error> for DownloadError {
    fn from(_: failure::Error) -> Self {
        DownloadError
    }
}

impl From<url::ParseError> for DownloadError {
    fn from(_: url::ParseError) -> Self {
        DownloadError
    }
}

pub fn get_image(args: &clap::ArgMatches) -> Result<String, DownloadError> {
    let mut easy = easy::Easy::new();

    easy.url("https://dog.ceo/api/breeds/image/random")?;
    let mut data = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;

        transfer.perform()?;
    }

    let html = String::from_utf8(data)?;

    let html_json: serde_json::Value = serde_json::from_str(&html)?;
    let img_url = &html_json["message"].as_str().ok_or(DownloadError)?;

    let url = duma::utils::parse_url(img_url)?;

    duma::download::http_download(url, &args, clap::crate_version!())?;

    let path = env::current_dir()?;
    let file_name = args.value_of("FILE").ok_or(DownloadError)?;

    Ok(String::from(format!("{}/{}", path.to_str().ok_or(DownloadError)?, file_name)))
}
