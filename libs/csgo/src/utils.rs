/// Translate Steam ID
pub fn translate_steam_id(steam_id: &String) -> String {
    todo!();
}

pub fn timestamp_to_epoch(timestamp: &str) -> i64 {
    chrono::NaiveDateTime::parse_from_str(timestamp, "%m/%d/%Y - %H:%M:%S")
        .unwrap()
        .timestamp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_to_epoch() {
        assert_eq!(timestamp_to_epoch("01/01/1970 - 00:00:00"), 0);
        assert_eq!(timestamp_to_epoch("05/04/2020 - 21:49:01"), 1588628941);
        assert_eq!(timestamp_to_epoch("03/29/2021 - 15:39:30"), 1617032370);
    }
}
