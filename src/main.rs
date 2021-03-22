use std::path::PathBuf;

use simple_logger::SimpleLogger;
use structopt::StructOpt;

mod csgo;

fn main() {
    // Get the CLI args
    let args = csgo::cli::Opt::from_args();

    // Set the logging level
    match &args.verbose {
        0 => SimpleLogger::new()
            .with_level(log::LevelFilter::Error)
            .init()
            .unwrap(),
        1 => SimpleLogger::new()
            .with_level(log::LevelFilter::Warn)
            .init()
            .unwrap(),
        2 => SimpleLogger::new()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap(),
        3 => SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()
            .unwrap(),
        _ => SimpleLogger::new()
            .with_level(log::LevelFilter::Trace)
            .init()
            .unwrap(),
    }

    log::info!("{:?}", &args);

    // Get the latest log file from the directory
    let log_dir: PathBuf = args.log_directory;

    // Create the parsing engine
    let mut engine = csgo::core::Engine::new(log_dir, 2);

    // Start watching for logs and parsing
    engine.run();
}
