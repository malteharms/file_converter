mod utils;

use std::{env, process};
use log::{error, info};

use utils::logging::initialize_logging;
use utils::arguments::{Arguments, ArgumentError};

fn main() {
    initialize_logging();
    info!("=== FILE CONVERTER CLI TOOL ===");

    let args: Vec<String> = env::args().collect();
    let _arguments: Arguments = Arguments::from_args(args)
        .unwrap_or_else(|error: ArgumentError| {
            error!("Error while parsing arguments: {:?}", error.display());
            process::exit(1);
        });

}
