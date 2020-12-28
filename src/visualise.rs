use std::{error, fmt};
use viuer::{print_from_file, Config};

#[derive(Debug, Clone)]
pub(crate) struct VisualisationError;

impl error::Error for VisualisationError {}

impl fmt::Display for VisualisationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Visualisation error")
    }
}

pub(crate) fn draw_image(file_path: &str) -> Result<(), VisualisationError> {
    let terminal_size = get_terminal_size();

    let conf = Config {
        width: Some(terminal_size.0),
        height: Some(terminal_size.1),
        ..Default::default()
    };

    print_from_file(file_path, &conf).map_err(|_| VisualisationError)?;
    Ok(())
}

fn get_terminal_size() -> (u32, u32) {
    if let Some((w, h)) = term_size::dimensions() {
        (w as u32, h as u32)
    } else {
        (80, 45)
    }
}
