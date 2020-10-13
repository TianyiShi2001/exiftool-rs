pub mod exifdata;
pub mod utils;
use crate::exifdata::ExifData;
use crate::utils::BoxedError;
use serde_json;
use std::process::Command;

pub struct ExifTool {
    args: Vec<String>,
}

impl ExifTool {
    pub fn new(path: &str) -> Self {
        Self {
            args: vec![path.to_owned(), "-j".to_owned()],
        }
    }
    pub fn with_args(args: Vec<String>) -> Self {
        Self { args }
    }
    pub fn exec(&self) -> Result<ExifData, BoxedError> {
        let output = Command::new("exiftool").args(&self.args).output()?;
        let res: ExifData = serde_json::from_slice(&output.stdout)?;
        Ok(res)
    }
}
