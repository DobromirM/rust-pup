use clap;
use core::fmt;
use curl::easy;
use duma;
use std::{env, error};

#[derive(Debug, Clone)]
pub(crate) struct DownloadError;

impl error::Error for DownloadError {}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Download error")
    }
}

pub(crate) fn get_image(
    args: &clap::ArgMatches,
    url_string: &str,
) -> Result<String, DownloadError> {
    let mut easy = easy::Easy::new();

    easy.url(url_string).map_err(|_| DownloadError)?;
    let mut data = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .map_err(|_| DownloadError)?;

        transfer.perform().map_err(|_| DownloadError)?;
    }

    let html = String::from_utf8(data).map_err(|_| DownloadError)?;

    let html_json: serde_json::Value = serde_json::from_str(&html).map_err(|_| DownloadError)?;
    let img_url = &html_json["message"]
        .as_str()
        .ok_or(DownloadError)
        .map_err(|_| DownloadError)?;

    let url = duma::utils::parse_url(img_url).map_err(|_| DownloadError)?;
    duma::download::http_download(url, args, clap::crate_version!()).map_err(|_| DownloadError)?;

    Ok(args
        .value_of("FILE")
        .ok_or(DownloadError)
        .map_err(|_| DownloadError)?
        .to_owned())
}
