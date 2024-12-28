#[derive(Debug, PartialEq)]
pub enum ArgumentErrorType {
    MissingParameterKey,
    UnknownParameterKey,
    MissingInputValue,
    MissingOutputValue,
    MissingArguments,
}

#[derive(Debug, PartialEq)]
pub enum DispatcherError {
    InputFileHasNoExtension,
    InputFileExtensionHasWrongFormat,
    InputFileTypeNotSupported,
    OutputFileTypeNotSupported,
}

#[derive(Debug, PartialEq)]
pub enum SettingsErrorType {
    SettingsFileNotFound,
    InvalidSettingsFile(String),
    SectionNotFound,
    KeyNotFound
}
