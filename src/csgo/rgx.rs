use lazy_static::lazy_static;

//const MATCH_START: &str = r#"^L \d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}: World triggered "Match_Start" on "([^"]+)""#;
//const GAME_OVER: &str = r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): Game Over: ([^\s]+).+?score (\d+):(\d+) after (\d+) min"#;
//const SWITCHED_TEAM: &str = r#"^L \d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}: "([^<]+)<\d{1,3}><(STEAM[^>]+)>" switched from team <(?:Unassigned|CT|TERRORIST)> to <(CT|TERRORIST)>"#;
//const ATTACK: &str = r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" \[(-?\d{1,5} -?\d{1,5} -?\d{1,5})\] attacked "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" \[(-?\d{1,5} -?\d{1,5} -?\d{1,5})\] with "([^"]+)" \(damage "(\d+)"\) \(damage_armor "(\d+)"\) \(health "(\d+)"\) \(armor "(\d+)"\) \(hitgroup "([^"]+)"\)"#;
//const KILL: &str = r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" \[(-?\d{1,5} -?\d{1,5} -?\d{1,5})\] killed "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" \[(-?\d{1,5} -?\d{1,5} -?\d{1,5})\] with "([^"]+)"(?: \(([^)]+)\))?"#;
//const ASSIST: &str = r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" assisted killing "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>""#;
//const SUICIDE: &str = r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" \[-?\d{1,5} -?\d{1,5} -?\d{1,5}\] committed suicide with "[a-zA-Z0-9]{1,20}""#;
//const BLINDED: &str = r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" blinded for (\d\.\d{2}) by "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" from flashbang entindex [0-9\s]{1,5}"#;
//const BOMB: &str = r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" triggered "(Got_The_Bomb|Dropped_The_Bomb|Planted_The_Bomb|Begin_Bomb_Defuse_With(?:out)?_Kit|Defused_The_Bomb)""#;
//const HOSTAGE: &str = r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" triggered "(Touched_A_Hostage|Rescued_A_Hostage|Killed_A_Hostage)""#;
//const CHICKEN: &str = r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" .+? killed other "chicken.+? with "([^"]+)""#;

pub fn match_start(input: &str) -> Option<regex::Captures> {
    lazy_static! {
        static ref MATCH_START: regex::Regex = regex::Regex::new(r#"^L \d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}: World triggered "Match_Start" on "([^"]+)""#).unwrap();
    }

    MATCH_START.captures(input)
}

pub fn game_over(input: &str) -> Option<regex::Captures> {
    lazy_static! {
        static ref GAME_OVER: regex::Regex = regex::Regex::new(r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): Game Over: ([^\s]+).+?score (\d+):(\d+) after (\d+) min"#).unwrap();
    }

    GAME_OVER.captures(input)
}

pub fn switched_team(input: &str) -> Option<regex::Captures> {
    lazy_static! {
        static ref SWITCHED_TEAM: regex::Regex = regex::Regex::new(r#"^L \d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}: "([^<]+)<\d{1,3}><(STEAM[^>]+)>" switched from team <(?:Unassigned|CT|TERRORIST)> to <(CT|TERRORIST)>"#).unwrap();
    }

    SWITCHED_TEAM.captures(input)
}

pub fn attack(input: &str) -> Option<regex::Captures> {
    lazy_static! {
        static ref ATTACK: regex::Regex = regex::Regex::new(r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" \[(-?\d{1,5} -?\d{1,5} -?\d{1,5})\] attacked "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" \[(-?\d{1,5} -?\d{1,5} -?\d{1,5})\] with "([^"]+)" \(damage "(\d+)"\) \(damage_armor "(\d+)"\) \(health "(\d+)"\) \(armor "(\d+)"\) \(hitgroup "([^"]+)"\)"#).unwrap();
    }

    ATTACK.captures(input)
}

pub fn kill(input: &str) -> Option<regex::Captures> {
    lazy_static! {
        static ref KILL: regex::Regex = regex::Regex::new(r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" \[(-?\d{1,5} -?\d{1,5} -?\d{1,5})\] killed "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" \[(-?\d{1,5} -?\d{1,5} -?\d{1,5})\] with "([^"]+)"(?: \(([^)]+)\))?"#).unwrap();
    }

    KILL.captures(input)
}

pub fn assist(input: &str) -> Option<regex::Captures> {
    lazy_static! {
        static ref ASSIST: regex::Regex = regex::Regex::new(r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" assisted killing "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>""#).unwrap();
    }

    ASSIST.captures(input)
}

pub fn suicide(input: &str) -> Option<regex::Captures> {
    lazy_static! {
        static ref SUICIDE: regex::Regex = regex::Regex::new(r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" \[-?\d{1,5} -?\d{1,5} -?\d{1,5}\] committed suicide with "[a-zA-Z0-9]{1,20}""#).unwrap();
    }

    SUICIDE.captures(input)
}

pub fn blinded(input: &str) -> Option<regex::Captures> {
    lazy_static! {
        static ref BLINDED: regex::Regex = regex::Regex::new(r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" blinded for (\d\.\d{2}) by "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" from flashbang entindex [0-9\s]{1,5}"#).unwrap();
    }

    BLINDED.captures(input)
}

pub fn bomb(input: &str) -> Option<regex::Captures> {
    lazy_static! {
        static ref BOMB: regex::Regex = regex::Regex::new(r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" triggered "(Got_The_Bomb|Dropped_The_Bomb|Planted_The_Bomb|Begin_Bomb_Defuse_With(?:out)?_Kit|Defused_The_Bomb)""#).unwrap();
    }

    BOMB.captures(input)
}

pub fn hostage(input: &str) -> Option<regex::Captures> {
    lazy_static! {
        static ref HOSTAGE: regex::Regex = regex::Regex::new(r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" triggered "(Touched_A_Hostage|Rescued_A_Hostage|Killed_A_Hostage)""#).unwrap();
    }

    HOSTAGE.captures(input)
}

pub fn chicken(input: &str) -> Option<regex::Captures> {
    lazy_static! {
        static ref CHICKEN: regex::Regex = regex::Regex::new(r#"^L (\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2}): "([^<]+)<\d{1,3}><(STEAM[^>]+)><(CT|TERRORIST)>" .+? killed other "chicken.+? with "([^"]+)""#).unwrap();
    }

    CHICKEN.captures(input)
}
