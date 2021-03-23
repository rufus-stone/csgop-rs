/// Translate Steam ID
pub fn translate_steam_id(steam_id: &String) -> String {
    return steam_id.to_string();
}

pub fn timestamp_to_epoch(timestamp: &str) -> i64 {
    chrono::NaiveDateTime::parse_from_str(timestamp, "%m/%d/%Y - %H:%M:%S")
        .unwrap()
        .timestamp()
}
