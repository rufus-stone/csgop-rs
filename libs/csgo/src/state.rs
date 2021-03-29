use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::geo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    name: String,
    origin: geo::Point,
    scale: u8,
}

impl Map {
    pub fn from_name(name: &str) -> Map {
        Map {
            name: name.to_owned(),
            origin: geo::Point::from_xyz(1, 2, 0),
            scale: 14,
        }
    }

    pub fn new() -> Map {
        Map {
            name: String::new(),
            origin: geo::Point::new(),
            scale: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Player {
    name: String,
    id: String,
}

impl Player {
    pub fn new(name: String, id: String) -> Player {
        Player { name, id }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    name: String,
    players: Vec<Player>,
}

impl Team {
    pub fn new(team_name: &str) -> Team {
        Team {
            name: team_name.to_owned(),
            players: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    epoch: i64,
    detail: HashMap<String, String>,
}

impl Event {
    pub fn new(epoch: i64, detail: HashMap<String, String>) -> Event {
        Event { epoch, detail }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    map: Map,
    mode: String,
    cts: Team,
    ts: Team,
    events: Vec<Event>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            map: Map::new(),
            mode: String::new(),
            cts: Team::new("CT"),
            ts: Team::new("TERRORIST"),
            events: Vec::new(),
        }
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn map_mut(&mut self) -> &mut Map {
        &mut self.map
    }

    pub fn mode(&self) -> &String {
        &self.mode
    }

    pub fn mode_mut(&mut self) -> &mut String {
        &mut self.mode
    }

    pub fn cts(&self) -> &Team {
        &self.cts
    }

    pub fn cts_mut(&mut self) -> &mut Team {
        &mut self.cts
    }

    pub fn ts(&self) -> &Team {
        &self.ts
    }

    pub fn ts_mut(&mut self) -> &mut Team {
        &mut self.ts
    }

    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn events_mut(&mut self) -> &mut Vec<Event> {
        &mut self.events
    }

    pub fn add_player_to_team(&mut self, new_player: Player, team_name: &str) {
        // First, remove the player if they're already in a team
        if self.cts().players.contains(&new_player) {
            self.cts_mut()
                .players
                .retain(|player| player != &new_player); // i.e. retain all the current players that are NOT equal to new_player
        } else if self.ts().players.contains(&new_player) {
            self.ts_mut().players.retain(|player| player != &new_player); // i.e. retain all the current players that are NOT equal to new_player
        }

        // Second, add the new player to their team
        match team_name {
            "CT" => {
                log::info!("Adding player {:?} to team {}", &new_player, &team_name);
                self.cts_mut().players.push(new_player);
            }
            "TERRORIST" => {
                log::info!("Adding player {:?} to team {}", &new_player, &team_name);
                self.ts_mut().players.push(new_player);
            }
            _ => {
                log::warn!("Unknown team: {}", team_name)
            }
        }

        log::info!("CT: {:?}", self.cts());
        log::info!("TERRORIST: {:?}", self.ts());
    }
}
