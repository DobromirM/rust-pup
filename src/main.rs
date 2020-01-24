use clap::{Arg, App};
use std::{fs,process};

mod visualise;
mod download;


fn main() {
    let image_name = "dog.jpg";

    let args = App::new("")
        .version("0.1")
        .arg(Arg::with_name("quiet")
            .required(false)
            .value_name("quiet")
            .default_value("true")
            .takes_value(true))
        .arg(Arg::with_name("FILE")
            .required(false)
            .value_name("FILE")
            .default_value(&image_name)
            .takes_value(true))
        .arg(Arg::with_name("hd")
            .required(false)
            .value_name("hd")
            .long("hd")
            .short("h")
            .help("Displays the image in HD")
            .takes_value(false))
        .get_matches();

    let file_path = match download::get_image(&args) {
        Ok(value) => value,
        Err(_) => {
            println!("Failed to retrieve dog :(");
            process::exit(1);
        }
    };

    if args.is_present("hd") {
        if process::Command::new("feh")
            .arg("&")
            .arg("-x")
            .arg("-FZ")
            .arg(&file_path)
            .output().is_err() {
            println!("The dog is hiding :(");
        }
    } else {
        if visualise::draw_image(&file_path).is_err() {
            println!("The dog is hiding :(");
        }
    }

    if fs::remove_file(&file_path).is_err() {
        println!("The dog got stuck :( \nYou can find it in {}", &file_path);
        process::exit(3);
    }
}
