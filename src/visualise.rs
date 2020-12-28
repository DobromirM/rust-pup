use std::{error, fmt};
use viuer::print_from_file;

#[derive(Debug, Clone)]
pub(crate) struct VisualisationError;

impl error::Error for VisualisationError {}

impl fmt::Display for VisualisationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Visualisation error")
    }
}

pub(crate) fn draw_image(file_path: &str) -> Result<(), VisualisationError> {
    print_from_file(file_path, &Default::default()).map_err(|_| VisualisationError)?;
    Ok(())
}
