use std::path::PathBuf;
use crate::errors::DispatcherError;
use crate::models::results::DispatcherResult;

const AVAILABLE_OUTPUT_FORMATS: [&str; 1] = [
    "txt"
];

pub struct TextFileConverter {
    _input_file: PathBuf,
    output_file: PathBuf
}

impl TextFileConverter {
    pub fn new(_input_file: PathBuf, output_file: PathBuf) -> Self {
        Self {
            _input_file,
            output_file
        }
    }

    fn supports_output_format(&self, output_format: &str) -> bool {
        AVAILABLE_OUTPUT_FORMATS.contains(&output_format)
    }

    pub fn convert(&self) -> Result<DispatcherResult, DispatcherError> {

        let output_extension: &str = self.output_file.extension().unwrap().to_str().unwrap();
        if !self.supports_output_format(output_extension) {
            return Err(DispatcherError::OutputFileTypeNotSupported);
        }

        let result: DispatcherResult = DispatcherResult::new();
        Ok(result)
    }
}