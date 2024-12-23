use std::path::PathBuf;


#[derive(Debug, PartialEq)]
enum ArgumentErrorType {
    MissingParameterKey,
    UnknownParameterKey,
    MissingInputValue,
    MissingOutputValue,
    MissingArguments,
}

#[derive(Debug, PartialEq)]
pub struct ArgumentError {
    error_type: ArgumentErrorType,
    value: String
}


impl ArgumentError {
    fn new(error_type: ArgumentErrorType, value: String) -> ArgumentError {
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

enum State {
    ExpectingKey,
    ExpectInputValue,
    ExpectOutputValue,
}

#[derive(Debug, PartialEq)]
pub struct Arguments {
    input: PathBuf,
    output: PathBuf,
    force: bool
}

impl Arguments {
    fn new() -> Arguments {
        Arguments {
            input: PathBuf::new(),
            output: PathBuf::new(),
            force: false
        }
    }

    pub fn from_args(mut args: Vec<String>) -> Result<Arguments, ArgumentError> {
        let mut state = State::ExpectingKey;
        let mut resulting_arguments: Arguments = Arguments::new();

        // First element is always the called binary
        if args.len() <= 1 {
            return Err(ArgumentError::new(
                ArgumentErrorType::MissingArguments,
                "".to_string()
            ));
        }
        args.remove(0);

        for arg in args {
            match state {
                State::ExpectingKey => {
                    if !arg.starts_with("--") {
                        return Err(ArgumentError::new(
                            ArgumentErrorType::MissingParameterKey,
                            arg
                        ));
                    }

                    let key = &arg[2..];    // removing the trailing "--"
                    match key {
                        // key followed by a value
                        "input" => { state = State::ExpectInputValue }
                        "output" => { state = State::ExpectOutputValue }

                        // boolean keys
                        "force" => { resulting_arguments.force = true }
                        _ => { return Err(ArgumentError::new(
                            ArgumentErrorType::UnknownParameterKey,
                            key.to_string()
                        )) }
                    }
                }

                State::ExpectInputValue => {
                    if arg.starts_with("--") {
                        return Err(ArgumentError::new(
                            ArgumentErrorType::MissingInputValue,
                            "".to_string()
                        ));
                    }

                    resulting_arguments.input = PathBuf::from(arg);
                    state = State::ExpectingKey;
                }

                State::ExpectOutputValue => {
                    if arg.starts_with("--") {
                        return Err(ArgumentError::new(
                            ArgumentErrorType::MissingOutputValue,
                            "".to_string()
                        ));
                    }

                    resulting_arguments.output = PathBuf::from(arg);
                    state = State::ExpectingKey;
                }
            }
        }

        // checking if required values are empty
        if resulting_arguments.input.as_os_str().is_empty() {
            return Err(ArgumentError::new(
                ArgumentErrorType::MissingInputValue,
                "".to_string()
            ));
        } else if resulting_arguments.output.as_os_str().is_empty() {
            return Err(ArgumentError::new(
                ArgumentErrorType::MissingOutputValue,
                "".to_string()
            ));
        }

        Ok(resulting_arguments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_arguments_without_force() {
        let args = vec![
            "BINARY".to_string(),
            "--input".to_string(), "input.txt".to_string(),
            "--output".to_string(), "output.txt".to_string()
        ];

        let result = Arguments::from_args(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_correct_arguments_with_force() {
        let args = vec![
            "BINARY".to_string(),
            "--input".to_string(), "input.txt".to_string(),
            "--output".to_string(), "output.txt".to_string(),
            "--force".to_string()
        ];

        let result = Arguments::from_args(args);
        assert!(result.is_ok());
        assert!(result.unwrap().force)
    }

    #[test]
    fn test_missing_input_value() {
        let args = vec![
            "BINARY".to_string(),
            "--input".to_string(),
            "--output".to_string(), "output.txt".to_string()
        ];

        let result = Arguments::from_args(args);
        assert_eq!(
            result.unwrap_err().error_type,
            ArgumentErrorType::MissingInputValue
        )
    }

    #[test]
    fn test_missing_output_value() {
        let args = vec![
            "BINARY".to_string(),
            "--input".to_string(), "input.txt".to_string(),
            "--output".to_string()
        ];

        let result = Arguments::from_args(args);
        assert_eq!(
            result.unwrap_err().error_type,
            ArgumentErrorType::MissingOutputValue
        )
    }

    #[test]
    fn test_unknown_parameter_key() {
        let args = vec![
            "BINARY".to_string(),
            "--input".to_string(), "input.txt".to_string(),
            "--output".to_string(), "output.txt".to_string(),
            "--unknown".to_string()
        ];

        let result = Arguments::from_args(args);
        assert_eq!(
            result.unwrap_err().error_type,
            ArgumentErrorType::UnknownParameterKey
        )
    }

    #[test]
    fn test_missing_parameter_key() {
        let args = vec![
            "BINARY".to_string(),
            "input.txt".to_string(),
            "--output".to_string(), "output.txt".to_string(),
        ];

        let result = Arguments::from_args(args);
        assert_eq!(
            result.unwrap_err().error_type,
            ArgumentErrorType::MissingParameterKey
        )
    }

    #[test]
    fn test_empty_arguments() {
        let args = vec![];

        let result = Arguments::from_args(args);
        assert_eq!(
            result.unwrap_err().error_type,
            ArgumentErrorType::MissingArguments
        )
    }
}
