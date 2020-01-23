use std::io::{stdout, stderr};
use image::GenericImageView;
use std::fs;
use term_size;

pub fn run(file_path: String) -> i32 {
    if let Err(err) = draw_image(file_path) {
        err.print_error(&mut stderr());
        err.exit_value()
    } else {
        0
    }
}

fn draw_image(file_path: String) -> Result<(), termimage::Error> {
    let terminal_size = get_terminal_size();
    let file = (file_path.to_owned(), fs::canonicalize(file_path).unwrap());
    let format = termimage::ops::guess_format(&file)?;
    let img = termimage::ops::load_image(&file, format)?;
    let img_size = termimage::ops::image_resized_size(img.dimensions(), terminal_size, true);
    let resized = termimage::ops::resize_image(&img, img_size);

    termimage::ops::write_ansi_truecolor(&mut stdout(), &resized);
    Ok(())
}

fn get_terminal_size() -> (u32, u32) {
    if let Some((w, h)) = term_size::dimensions() {
        (w as u32, h as u32)
    } else {
        (80, 45)
    }
}