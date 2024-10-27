use antelope::{Asset, Name};

pub fn parse_json_asset(data_json: &str, key: &str) -> Option<Asset> {
    let v = serde_json::from_str::<serde_json::Value>(data_json);
    match v {
        Ok(data) => {
            let value_str = data[key].as_str().unwrap_or("");
            let value = value_str.parse::<Asset>();
            match value {
                Ok(asset) => Some(asset),
                Err(_e) => None,
            }
        }
        Err(_e) => None,
    }
}

pub fn parse_json_name(data_json: &str, key: &str) -> Option<Name> {
    let v = serde_json::from_str::<serde_json::Value>(data_json);
    match v {
        Ok(data) => {
            let value_str = data[key].as_str().unwrap_or("");
            let value = value_str.parse::<Name>();
            match value {
                Ok(name) => Some(name),
                Err(_e) => None,
            }
        }
        Err(_e) => None,
    }
}
