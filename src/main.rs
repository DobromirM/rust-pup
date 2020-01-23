use termimage;
use image;

use std::process::exit;
use std::process::Command;
use std::fs;

mod visualise;
mod download;

fn main() {
    let file_path = download::run();


    Command::new("feh")
        .arg("&")
        .arg("-x")
        .arg(&file_path)
        .output();

    let result = visualise::run(file_path.to_owned());
    fs::remove_file(file_path);
    exit(result);
}



