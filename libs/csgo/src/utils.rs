use super::config;
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::Sha256;

fn map_steam_id(steam_id: &str, mappings: &[config::SteamIdTranslationMapping]) -> Option<String> {
    for mapping in mappings {
        if mapping.steam_id == steam_id {
            return Some(mapping.name.clone());
        }
    }
    None
}

fn to_md5(data: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

fn to_sha1(data: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

fn to_sha256(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

fn hash_steam_id(steam_id: &str, algo: &config::HashAlgo) -> String {
    match algo {
        config::HashAlgo::MD5 => to_md5(steam_id),
        config::HashAlgo::SHA1 => to_sha1(steam_id),
        config::HashAlgo::SHA256 => to_sha256(steam_id),
    }
}

/// Translate Steam ID
pub fn translate_steam_id(steam_id: &str, config: &config::Config) -> String {
    match &config.steam_id_translation {
        Some(translation) => {
            if translation.active {
                match (&translation.hash, &translation.mappings) {
                    (None, None) => steam_id.to_owned(),
                    (None, Some(mappings)) => match map_steam_id(steam_id, &mappings) {
                        Some(mapped_name) => mapped_name,
                        None => steam_id.to_owned(),
                    },
                    (Some(algo), None) => hash_steam_id(steam_id, algo),
                    // If a mapping AND a hash are specified, prefer the mapping and only hash if no mapping fits
                    (Some(algo), Some(mappings)) => match map_steam_id(steam_id, &mappings) {
                        Some(mapped_name) => mapped_name,
                        None => hash_steam_id(steam_id, algo),
                    },
                }
            } else {
                steam_id.to_owned()
            }
        }
        None => steam_id.to_owned(),
    }
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

    #[test]
    fn test_hashes() {
        assert_eq!(to_md5("Hello, World!"), "65a8e27d8879283831b664bd8b7f0ad4");
        assert_eq!(
            to_sha1("Hello, World!"),
            "0a0a9f2a6772942557ab5355d76af442f8f65e01"
        );
        assert_eq!(
            to_sha256("Hello, World!"),
            "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
        );
    }

    #[test]
    fn test_steam_id_translation() {
        let mut config = config::Config::default();
        config.steam_id_translation = Some(config::SteamIdTranslation {
            active: true,
            hash: Some(config::HashAlgo::MD5),
            mappings: Some(vec![
                config::SteamIdTranslationMapping {
                    steam_id: "STEAM_1:1:00000001".to_owned(),
                    name: "Player One".to_owned(),
                },
                config::SteamIdTranslationMapping {
                    steam_id: "STEAM_1:1:00000002".to_owned(),
                    name: "Player Two".to_owned(),
                },
            ]),
        });

        assert_eq!(
            translate_steam_id("STEAM_1:1:00000001", &config),
            "Player One"
        );

        assert_eq!(
            translate_steam_id("STEAM_1:1:00000002", &config),
            "Player Two"
        );

        assert_eq!(
            translate_steam_id("STEAM_1:1:12345678", &config),
            "6eb8da106687919a50b73f9fc6e097ac"
        );
    }
}
