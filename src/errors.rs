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
