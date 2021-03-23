use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
    pub fn new() -> Team {
        Team {
            name: String::new(),
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
    map: String,
    mode: String,
    cts: Team,
    ts: Team,
    events: Vec<Event>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            map: String::new(),
            mode: String::new(),
            cts: Team::new(),
            ts: Team::new(),
            events: Vec::new(),
        }
    }

    pub fn map(&self) -> &String {
        &self.map
    }

    pub fn map_mut(&mut self) -> &mut String {
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
}
