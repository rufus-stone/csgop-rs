use std::fs::{File, Metadata};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::PathBuf;

pub fn latest_log_file(path: &PathBuf) -> Option<PathBuf> {
    // Find the most recent file in the directory (based on file name)
    let mut files: Vec<_> = std::fs::read_dir(&path)
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
        std::fs::canonicalize(&path).unwrap().display()
    );
    for file in &files {
        log::trace!("--> '{}'", file.path().display())
    }

    let full_path = std::fs::canonicalize(&files[0].path()).unwrap(); //.display();
    Some(full_path)
}

#[derive(Debug)]
pub struct Watcher {
    path: PathBuf,
    fd: File,
    fm: Metadata,
    delay: u8,
    pos: u64,
}

impl Watcher {
    pub fn new(path: PathBuf) -> Result<Watcher, std::io::Error> {
        let fd = File::open(&path)?; // File descriptor to the log file
        let fm = fd.metadata()?; // Metadata about the file - we'll use this to find out whether the file size has changed
        let delay = 1; // Default to 1 second between file checks
        let pos = 0; // Start reading from the beginning of the file

        Ok(Watcher {
            path: path,
            fd: fd,
            fm: fm,
            delay: delay,
            pos: pos,
        })
    }

    pub fn read_latest(&mut self) -> std::io::Result<Vec<u8>> {
        // Get the latest file metadata
        let latest_metadata = self.fd.metadata()?;

        // Create an empty vector to store any new data we read in
        let mut file_data: Vec<u8> = Vec::new();

        // If this if the first time, just read everything
        if self.pos == 0 {
            // Open the file with a BufReader
            let mut buf_reader = BufReader::new(&self.fd);

            // Skip ahead to where we last stopped reading
            buf_reader.seek(SeekFrom::Start(self.pos))?;

            // Read from here to the end of the file
            buf_reader.read_to_end(&mut file_data)?;

            // Did we successfully read any actual data?
            if !file_data.is_empty() {
                // Update self with the new seek position
                self.pos = self.fm.len();

                // Update self with the new metadata
                self.fm = latest_metadata;
            }

        // Otherwise, compare the current file length to the previous length...
        // ...and read the new lines if it's bigger
        } else if latest_metadata.len() > self.fm.len() {
            // Open the file with a BufReader
            let mut buf_reader = BufReader::new(&self.fd);

            // Skip ahead to where we last stopped reading
            buf_reader.seek(SeekFrom::Start(self.pos))?;

            // Read from here to the end of the file
            buf_reader.read_to_end(&mut file_data)?;

            // Did we successfully read any actual data?
            if !file_data.is_empty() {
                // Update self with the new seek position
                self.pos = self.fm.len();

                // Update self with the new metadata
                self.fm = latest_metadata;
            }
        }

        Ok(file_data)
    }
}
