use std::collections::BTreeMap;
use std::collections::HashMap;

use rgx::{suicide, switched_team};

use super::logs;
use super::rgx;
use super::state;
use super::utils;

pub struct Engine {
    delay: u64,
    reader: logs::Reader,
    game_state: state::GameState,
}

impl Engine {
    /// Create a new Engine that will look for log files within the specified directory, repeating every `delay` seconds
    ///
    /// ```rust
    /// let mut engine = csgo::core::Engine::new(std::path::PathBuf::from("path/to/log/dir"), 2); // Every 2 second, `engine` will look for log files within the directory at `path/to/log/dir`
    ///
    /// engine.run(); // Start the engine
    /// ```
    pub fn new(log_dir_path: std::path::PathBuf, delay: u64) -> Engine {
        Engine {
            delay,
            reader: logs::Reader::new(log_dir_path),
            game_state: state::GameState::new(),
        }
    }

    /// Start the engine!
    ///
    /// This will continuously check for new logs and parse any lines found
    pub fn run(&mut self) {
        // Continuously monitor the logs
        loop {
            // Read the latest log lines
            let data = self.reader.read_latest().unwrap_or_default();

            // Convert the data vec into a string
            let data = std::str::from_utf8(&data).unwrap_or_default();

            // If the data is not empty, split into lines and parse
            if !data.is_empty() {
                // Benchmarking, just for interest
                let now = std::time::Instant::now();
                let mut line_count = 0;

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

                    line_count += 1;
                }

                log::info!(
                    "Parsed {} lines in {} ms",
                    line_count,
                    now.elapsed().as_millis()
                );
            }

            // Sleep
            log::trace!("Sleeping for {}s", self.delay);
            std::thread::sleep(std::time::Duration::from_secs(self.delay as u64));
        }
    }

    /// Check which regex pattern matched for the given line, if any, and update the game state accordingly
    fn parse_line(&mut self, line: &str) -> Option<Vec<String>> {
        log::trace!("Parsing line: {}", line);

        match rgx::match_start(line) {
            Some(captures) => {
                log::debug!("Match Started: {:?}", &captures);

                // In case a new match was started before the previous one ended, clear out the game_state and start afresh
                self.game_state = state::GameState::new();
                *self.game_state.map_mut() = captures[1].to_owned();

                log::info!("{:?}", &self.game_state);
            }
            None => {}
        }

        match rgx::switched_team(line) {
            Some(captures) => {
                log::debug!("Switched Teams: {:?}", &captures);
            }
            None => {}
        }

        match rgx::attack(line) {
            Some(captures) => {
                log::debug!("Attack: {:?}", &captures);
            }
            None => {}
        }

        match rgx::kill(line) {
            Some(captures) => {
                log::debug!("Kill: {:?}", &captures);
            }
            None => {}
        }

        match rgx::assist(line) {
            Some(captures) => {
                log::debug!("Assist: {:?}", &captures);
            }
            None => {}
        }

        match rgx::suicide(line) {
            Some(captures) => {
                log::debug!("Suicide: {:?}", &captures);
            }
            None => {}
        }

        match rgx::blinded(line) {
            Some(captures) => {
                log::debug!("Blinded: {:?}", &captures);
            }
            None => {}
        }

        match rgx::bomb(line) {
            Some(captures) => {
                log::debug!("Bomb: {:?}", &captures);
            }
            None => {}
        }

        match rgx::hostage(line) {
            Some(captures) => {
                log::debug!("Hostage: {:?}", &captures);
            }
            None => {}
        }

        match rgx::chicken(line) {
            Some(captures) => {
                log::debug!("Chicken: {:?}", &captures);
            }
            None => {}
        }

        match rgx::game_over(line) {
            Some(captures) => {
                log::debug!("Match Ended: {:?}", &captures);

                let epoch = utils::timestamp_to_epoch(&captures[1]);
                let game_mode = &captures[2];
                let ct_score = &captures[3];
                let terrorist_score = &captures[4];
                let match_duration = &captures[5];

                *self.game_state.mode_mut() = game_mode.to_string();

                let mut event_detail = HashMap::new();
                event_detail.insert("event_type".to_owned(), "game_over".to_owned());
                event_detail.insert("timestamp".to_owned(), epoch.to_string());
                event_detail.insert("game_map".to_owned(), self.game_state.map().clone());
                event_detail.insert("game_duration".to_owned(), match_duration.to_owned());

                let event = state::Event::new(epoch, event_detail);

                // Now add events for winning/losing for each player
                // Ignore Armsrace ("gungameprogressive") games as these don't have a winning team
                // Also, Deathmatch mode doesn't track the winning team either, for some reason!
                if game_mode != "gungameprogressive" && game_mode != "deathmatch" {
                    // Who won? Or was it a draw?
                    let ct_score = ct_score.parse::<u32>().unwrap();
                    let terrorist_score = terrorist_score.parse::<u32>().unwrap();

                    match ct_score
                        .partial_cmp(&terrorist_score)
                        .expect("Impossible comparison!")
                    {
                        std::cmp::Ordering::Less => {
                            log::info!("Terrorists win!");
                        }
                        std::cmp::Ordering::Equal => {
                            log::info!("It's a draw!");
                        }
                        std::cmp::Ordering::Greater => {
                            log::info!("Counter-Terrorists win!");
                        }
                    }
                }

                self.game_state.events_mut().push(event);

                log::info!("{:?}", &self.game_state);

                let json = serde_json::to_value(&self.game_state).unwrap();

                log::info!("{}", serde_json::to_string_pretty(&json).unwrap());
            }
            None => {}
        }

        None
    }
}
