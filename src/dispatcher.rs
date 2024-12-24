use std::path::PathBuf;

use crate::errors::DispatcherError;
use crate::models::results::DispatcherResult;
use crate::converters::text_file_converter::TextFileConverter;

pub fn determine_and_run_conversion(
    input_file: &PathBuf,
    output_file: &PathBuf
) -> Result<DispatcherResult, DispatcherError> {

    let input_extension: String;
    if let Some(extension) = input_file.extension() {
        if let Some(ext_str) = extension.to_str() {
            input_extension = ext_str.to_string();
        } else {
            return Err(DispatcherError::InputFileExtensionHasWrongFormat)
        }
    } else {
        return Err(DispatcherError::InputFileHasNoExtension);
    }

    match input_extension.as_str() {
        "txt" => {
            TextFileConverter::new(input_file.clone(), output_file.clone())
                .convert()
        }
        _ => { Err(DispatcherError::InputFileTypeNotSupported) }
    }
}
