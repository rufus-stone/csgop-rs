use std::fs::{File, Metadata};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::PathBuf;

pub struct Reader {
    log_dir_path: PathBuf,
    active_log: Option<PathBuf>,
    fm: Option<Metadata>,
    pos: usize,
}

impl Reader {
    pub fn new(log_dir_path: PathBuf) -> Reader {
        Reader {
            log_dir_path,
            active_log: None,
            pos: 0,
            fm: None,
        }
    }

    fn latest_log_file(log_dir_path: &PathBuf) -> Option<PathBuf> {
        // Find the most recent file in the directory (based on file name)
        let mut files: Vec<_> = std::fs::read_dir(&log_dir_path)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        if files.is_empty() {
            return None;
        }

        files.sort_by_key(|dir| dir.path()); // This sorts ascending
        files.reverse();

        log::trace!(
            "Files in directory: '{}'",
            std::fs::canonicalize(&log_dir_path).unwrap().display()
        );
        for file in &files {
            log::trace!("--> '{}'", file.path().display())
        }

        let full_path = std::fs::canonicalize(&files[0].path()).unwrap(); //.display();
        Some(full_path)
    }

    pub fn read_latest(&mut self) -> Option<Vec<u8>> {
        // Find the latest log file
        let latest_file = Reader::latest_log_file(&self.log_dir_path);

        if latest_file.is_none() {
            log::warn!("Failed to find latest log file!");
            return None;
        }

        // Is this the same as the file we were looking at previously?
        let mut new_log_file_detected = false;

        // Is there already an active log file?
        if self.active_log.is_some() {
            // Check the active log file still exists (in case it's been deleted/moved, etc.)
            if self.active_log.clone().unwrap().exists() {
                let active_log_path_str = self
                    .active_log
                    .clone()
                    .unwrap()
                    .canonicalize() // We want to compare the full canonical paths
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                let new_log_path_str = latest_file
                    .clone()
                    .unwrap()
                    .canonicalize() // We want to compare the full canonical paths
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                // Is the latest log file different to the currently active log file (just based on comparing the full paths)?
                // If so, we'll read from the active log file one last time...
                // .. and then update the active log file to point at the new file for the next loop
                if active_log_path_str != new_log_path_str {
                    log::info!("New log file {}", &new_log_path_str);
                    new_log_file_detected = true;
                }

            // ...or does it no longer exist? In which case, just use the latest log file
            } else {
                log::warn!(
                    "Active log file no longer exists! {}",
                    self.active_log.clone().unwrap().display()
                );

                self.active_log = latest_file.clone();
                self.pos = 0;
            }

        // If not, set the latest file as the active log file
        } else {
            log::info!(
                "Opened active log file '{}'",
                latest_file.as_ref().unwrap().display()
            );
            self.active_log = latest_file.clone();
        }

        // Open the active log file
        let fd = match File::open(self.active_log.as_ref().unwrap()) {
            Ok(result) => {
                log::debug!(
                    "Active file open: {}",
                    &self.active_log.clone().unwrap().display()
                );
                result
            }
            Err(_) => return None,
        };

        // Get the latest file metadata
        let latest_metadata = match fd.metadata() {
            Ok(result) => result,
            Err(_) => return None,
        };

        // Create an empty vector to store any new data we read in
        let mut file_data: Vec<u8> = Vec::new();

        // Only read in data if this is the first time looking at this file, or if the file size has changed since last time
        // TODO: Handle edge case where the active file is made shorter (not that this should happen!) - we don't want to try reading past the end of the file
        if self.pos == 0 || latest_metadata.len() > self.fm.as_ref().unwrap().len() {
            log::trace!("Reading file...");

            // Open the file with a BufReader
            let mut buf_reader = BufReader::new(&fd);

            // Skip ahead to where we last stopped reading
            match buf_reader.seek(SeekFrom::Start(self.pos as u64)) {
                Ok(_) => {}
                Err(_) => return None,
            }

            // Read from here to the end of the file and update the read position
            // As this is the first time reading, the new position will be equal to the number of bytes read
            let bytes_read = match buf_reader.read_to_end(&mut file_data) {
                Ok(bytes_read) => bytes_read,
                Err(_) => return None,
            };

            // Did we successfully read any actual data?
            if !file_data.is_empty() {
                log::debug!(
                    "Read {} bytes from position {} to {}",
                    &bytes_read,
                    &self.pos,
                    &self.pos + bytes_read
                );

                // Update self with the new seek position
                self.pos += bytes_read;
            }

            // Update self with the new metadata
            self.fm = Some(latest_metadata);

            // Update self with the new active file, if a new file was detected, and reset the reader position to 0
            if new_log_file_detected {
                self.active_log = latest_file;
                self.pos = 0;
            }

            // Return the bytes read
            Some(file_data)
        } else {
            // Update self with the new active file, if a new file was detected, and reset the reader position to 0
            if new_log_file_detected {
                self.active_log = latest_file;
                self.pos = 0;
            }

            None
        }
    }
}
