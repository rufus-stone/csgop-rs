use std::collections::HashMap;

use super::config;
use super::geo;
use super::logs;
use super::rgx;
use super::state;
use super::utils;

pub struct Engine {
    reader: logs::Reader,
    game_state: state::GameState,
    config: config::Config,
}

impl Engine {
    /// Create a new Engine with the specified Config
    ///
    /// ```rust
    /// let mut engine = csgo::core::Engine::new(std::path::PathBuf::from("path/to/log/dir"), 2); // Every 2 second, `engine` will look for log files within the directory at `path/to/log/dir`
    ///
    /// engine.run(); // Start the engine
    /// ```
    pub fn new(config: config::Config) -> Engine {
        log::info!("Using config: {:?}", &config);
        Engine {
            reader: logs::Reader::new(&config.log_dir),
            game_state: state::GameState::new(),
            config,
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
                    if let Some(json_vec) = json_vec {
                        Engine::dispatch(json_vec);

                        /*for json in json_vec.iter() {
                            log::info!("JSON: {}", json);
                        }*/
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
            log::trace!("Sleeping for {}s", self.config.delay);
            std::thread::sleep(std::time::Duration::from_secs(self.config.delay as u64));
        }
    }

    /// Check which regex pattern matched for the given line, if any, and update the game state accordingly
    fn parse_line(&mut self, line: &str) -> Option<Vec<String>> {
        log::trace!("Parsing line: {}", line);

        if let Some(captures) = rgx::match_start(line) {
            log::debug!("Match Started: {:?}", &captures);

            // In case a new match was started before the previous one ended, clear out the game_state and start afresh
            self.game_state = state::GameState::new();
            *self.game_state.map_mut() = state::Map::from_name(&captures[1]);

            log::info!("{:?}", &self.game_state);
        }

        if let Some(captures) = rgx::switched_team(line) {
            log::info!("Switched Teams: {:?}", &captures);

            let player_name = captures[1].to_owned();
            let player_id = utils::translate_steam_id(&captures[2], &self.config);
            let player = state::Player::new(player_name, player_id);

            let team = &captures[3];

            // Add the player to the specified team, removing them from the other team if they were already in-game);
            self.game_state.add_player_to_team(player, team);
        }

        if let Some(captures) = rgx::attack(line) {
            log::debug!("Attack: {:?}", &captures);

            // Timestamp
            let epoch = utils::timestamp_to_epoch(&captures[1]);

            // Attacking player
            let player_name = captures[2].to_owned();
            let player_id = utils::translate_steam_id(&captures[3], &self.config);
            let player_team = captures[4].to_owned();
            let player_position = captures[5].to_owned();
            let player_lat_lon = geo::game_pos_to_decimal_degrees(&player_position);

            // Victim of attack
            let victim_name = captures[6].to_owned();
            let victim_id = utils::translate_steam_id(&captures[7], &self.config);
            let victim_team = captures[8].to_owned();
            let victim_position = captures[9].to_owned();
            let victim_lat_lon = geo::game_pos_to_decimal_degrees(&victim_position);

            // Distance between players
            let distance = geo::metres_between_points(&player_position, &victim_position);

            // Weapon used
            let weapon = captures[10].to_owned();

            // Damage dealt to victim
            let damage_health = captures[11].to_owned();
            let damage_armor = captures[12].to_owned();

            // Victim health remaining
            let health_remaining = captures[13].to_owned();
            let armor_remaining = captures[14].to_owned();

            // Bodypart hit
            let hitgroup = captures[15].to_owned();

            let mut event_detail = HashMap::new();
            event_detail.insert("event_type".to_owned(), "attack".to_owned());
            event_detail.insert("timestamp".to_owned(), epoch.to_string());

            // Was it self-inflicted?
            if player_id == victim_id {
                event_detail.insert("self_inflicted".to_owned(), "true".to_owned());
            }

            event_detail.insert("player".to_owned(), player_name);
            event_detail.insert("player_id".to_owned(), player_id);
            event_detail.insert("player_team".to_owned(), player_team);
            event_detail.insert("player_position".to_owned(), player_position);
            event_detail.insert("player_lat_lon".to_owned(), player_lat_lon);

            event_detail.insert("victim".to_owned(), victim_name);
            event_detail.insert("victim_id".to_owned(), victim_id);
            event_detail.insert("victim_team".to_owned(), victim_team);
            event_detail.insert("victim_position".to_owned(), victim_position);
            event_detail.insert("victim_lat_lon".to_owned(), victim_lat_lon);

            event_detail.insert("distance".to_owned(), distance);
            event_detail.insert("weapon".to_owned(), weapon);

            event_detail.insert("damage_health".to_owned(), damage_health);
            event_detail.insert("damage_armor".to_owned(), damage_armor);
            event_detail.insert("health_remaining".to_owned(), health_remaining);
            event_detail.insert("armor_remaining".to_owned(), armor_remaining);
            event_detail.insert("hitgroup".to_owned(), hitgroup);

            event_detail.insert(
                "game_map".to_owned(),
                self.game_state.map().name().to_owned(),
            );

            let event = state::Event::new(epoch, event_detail);

            // Add the event to the game_state events buffer
            self.game_state.events_mut().push(event);
        }

        if let Some(captures) = rgx::kill(line) {
            log::debug!("Kill: {:?}", &captures);
        }

        if let Some(captures) = rgx::assist(line) {
            log::debug!("Assist: {:?}", &captures);
        }

        if let Some(captures) = rgx::suicide(line) {
            log::debug!("Suicide: {:?}", &captures);
        }

        if let Some(captures) = rgx::blinded(line) {
            log::debug!("Blinded: {:?}", &captures);
        }

        if let Some(captures) = rgx::bomb(line) {
            log::debug!("Bomb: {:?}", &captures);
        }

        if let Some(captures) = rgx::hostage(line) {
            log::debug!("Hostage: {:?}", &captures);
        }

        if let Some(captures) = rgx::chicken(line) {
            log::debug!("Chicken: {:?}", &captures);
        }

        if let Some(captures) = rgx::game_over(line) {
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
            event_detail.insert(
                "game_map".to_owned(),
                self.game_state.map().name().to_owned(),
            );
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

            //log::info!("{:?}", &self.game_state);

            let json = serde_json::to_value(&self.game_state).unwrap();

            log::info!("{}", serde_json::to_string_pretty(&json).unwrap());
        }

        None
    }

    fn dispatch(events: Vec<String>) {
        for event in events {
            log::info!("JSON: {}", event)
        }
    }
}
