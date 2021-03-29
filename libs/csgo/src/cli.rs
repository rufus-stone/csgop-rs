use structopt::StructOpt;

use std::path::PathBuf;
use std::string::String;

#[derive(StructOpt, Debug)]
pub struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Directory containing your CS:GO server logs
    #[structopt(name = "DIR", parse(from_os_str))]
    pub log_directory: PathBuf,

    /// Output format to generate (json, xml, csv, etc.)
    #[structopt(short, long, default_value = "json")]
    pub generate: String,
}
