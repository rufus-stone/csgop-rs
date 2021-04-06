use simple_logger::SimpleLogger;
use structopt::StructOpt;

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

    log::debug!("{:?}", &args);

    // Load the config from file
    let mut cfg = csgo::config::Config::from_file_or_default();

    // Get the latest log file from the directory
    let log_dir: std::path::PathBuf = args.log_directory;

    // Override the log_dir if one was provided
    cfg.log_dir = log_dir;

    // Create the parsing engine
    let mut engine = csgo::core::Engine::new(cfg);

    // Start watching for logs and parsing
    engine.run();
}
