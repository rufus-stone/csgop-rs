use super::logs;

pub struct Engine {
    delay: u64,
    reader: logs::Reader,
}

impl Engine {
    pub fn new(log_dir_path: std::path::PathBuf, delay: u64) -> Engine {
        Engine {
            delay,
            reader: logs::Reader::new(log_dir_path),
        }
    }

    pub fn run(&mut self) {
        // Continuously monitor the logs
        loop {
            // Read the latest log lines
            let data = self.reader.read_latest().unwrap_or_default();

            // Convert the data vec into a string
            let data = std::str::from_utf8(&data).unwrap_or_default();

            // Split into lines
            for line in data.lines() {
                // Parse the line
                let json_vec = self.parse_line(line);

                // Dispatch the results
                match json_vec {
                    Some(json_vec) => {
                        for json in json_vec.iter() {
                            log::info!("JSON: {}", json);
                        }
                    }
                    None => {}
                }
            }

            // Sleep
            log::trace!("Sleeping for {}s", self.delay);
            std::thread::sleep(std::time::Duration::from_secs(self.delay as u64));
        }
    }

    fn parse_line(&mut self, line: &str) -> Option<Vec<String>> {
        log::info!("{}", line);
        None
    }
}
