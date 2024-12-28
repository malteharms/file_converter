use crate::errors::ArgumentErrorType;

#[derive(Debug, PartialEq)]
pub struct ArgumentError {
    pub error_type: ArgumentErrorType,
    value: String
}


impl ArgumentError {
    pub(crate) fn new(error_type: ArgumentErrorType, value: String) -> ArgumentError {
        ArgumentError {
            error_type,
            value
        }
    }

    pub fn display(&self) -> String {
        match self.error_type {
            ArgumentErrorType::MissingParameterKey => {
                format!("Parameter is missing a key: {}", self.value)
            }
            ArgumentErrorType::UnknownParameterKey => {
                format!("Unknown parameter key: {}", self.value)
            }
            ArgumentErrorType::MissingInputValue => {
                "Missing input value".to_string()
            }
            ArgumentErrorType::MissingOutputValue => {
                "Missing output value".to_string()
            }
            ArgumentErrorType::MissingArguments => {
                "Missing arguments".to_string()
            }
        }
    }
}