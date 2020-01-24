use image::GenericImageView;
use std::{fs, error, fmt, io};
use term_size;
use termimage;

#[derive(Debug, Clone)]
pub struct VisualisationError;

impl error::Error for VisualisationError {}

impl fmt::Display for VisualisationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Visualisation error")
    }
}

impl From<termimage::Error, > for VisualisationError {
    fn from(_: termimage::Error) -> Self {
        VisualisationError
    }
}

impl From<std::io::Error, > for VisualisationError {
    fn from(_: std::io::Error) -> Self {
        VisualisationError
    }
}

pub fn draw_image(file_path: &str) -> Result<(), VisualisationError> {
    let terminal_size = get_terminal_size();
    let file = (file_path.to_owned(), fs::canonicalize(file_path)?);
    let format = termimage::ops::guess_format(&file)?;
    let img = termimage::ops::load_image(&file, format)?;
    let img_size = termimage::ops::image_resized_size(img.dimensions(), terminal_size, true);
    let resized = termimage::ops::resize_image(&img, img_size);

    termimage::ops::write_ansi_truecolor(&mut io::stdout(), &resized);
    Ok(())
}

fn get_terminal_size() -> (u32, u32) {
    if let Some((w, h)) = term_size::dimensions() {
        (w as u32, h as u32)
    } else {
        (80, 45)
    }
}