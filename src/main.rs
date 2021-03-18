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
    let latest_log = csgo::logs::latest_log_file(&log_dir).unwrap();
    log::info!("Latest log file: '{}'", &latest_log.display());

    // Create a log watcher
    let mut watcher = csgo::logs::Watcher::new(latest_log).unwrap();

    let lines = watcher.read_latest().unwrap();

    log::info!("{} bytes", &lines.len());

    //for line in lines {
    log::info!("{}", std::str::from_utf8(&lines).unwrap());
    //}
}
