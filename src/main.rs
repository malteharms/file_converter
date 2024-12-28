mod models;
mod converters;
mod dispatcher;
mod errors;

use std::io::Write;
use std::{env, process};
use log::{debug, error, info, warn, LevelFilter};

use models::argument_error::ArgumentError;
use models::arguments::Arguments;
use crate::errors::{DispatcherError, SettingsErrorType};
use crate::models::results::DispatcherResult;
use crate::models::settings::Settings;

fn initialize_logging(level: LevelFilter) {
    env_logger::builder()
        .filter_level(level)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

pub fn check_requirements(args: &Arguments, settings: &Settings) -> bool {
    let mut check_passed: bool = true;
    debug!("Checking requirements...");

    // In any case, input_file must exist!
    if !args.input.is_file() {
        error!("Error: Input file does not exist: {}", args.input.display());
        check_passed = false;
    }

    // The output_file can exist. But in this case, force needs to be
    // set to overwrite the file.
    if args.output.is_file() {
        if args.force || settings.general.default_overwrite {
            warn!("Output file already exists. It will be overwritten!");
        } else {
            error!("Output file {} already exists. Use --force to overwrite it", args.output.display());
            check_passed = false;
        }
    }

    // Both input and output need a file extension to detect the convert procedure
    if !args.input.extension().is_some() || !args.output.extension().is_some() {
        error!("Input and output file must have an extension!");
        check_passed = false;
    }

    check_passed
}

fn main() {
    info!("=== FILE CONVERTER CLI TOOL ===");

    let settings = Settings::load()
        .unwrap_or_else(|error: SettingsErrorType| {
            error!("Error while loading settings: {:?}", error);
            process::exit(1);
        });

    initialize_logging(settings.general.log_level);

    let args: Vec<String> = env::args().collect();
    let arguments: Arguments = Arguments::from_args(args)
        .unwrap_or_else(|error: ArgumentError| {
            error!("Error while parsing arguments: {:?}", error.display());
            process::exit(1);
        });


    if !check_requirements(&arguments, &settings) {
        error!("Environment requirements not met. Exiting...");
        process::exit(1);
    }

    let result: DispatcherResult = dispatcher::determine_and_run_conversion(
        &arguments.input,
        &arguments.output
    ).unwrap_or_else(|_error: DispatcherError| {
        // TODO print error message
        process::exit(1);
    });

    info!("Conversion finished. Duration: {} seconds", result.duration.as_secs());
}
