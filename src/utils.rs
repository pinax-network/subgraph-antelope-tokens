use substreams::{matches_keys_in_parsed_expr, pb::substreams::Clock};

// Clock to date string
// ex: Clock => 2015-07-30
pub fn to_date(clock: &Clock) -> String {
    let timestamp = clock.timestamp.as_ref().expect("missing timestamp");
    timestamp.to_string().split('T').next().expect("missing date").to_string()
}

pub fn is_match(query: Vec<String>, params: &str) -> bool {
    // match all if wildcard is used
    // `eosio:onblock` actions are excluded from wildcard
    if query.len() > 0 && params == "*" {
        return true;
    }
    match matches_keys_in_parsed_expr(&query, &params) {
        Ok(true) => {
            return true;
        }
        Ok(false) => {
            return false;
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match() {
        assert_eq!(is_match(vec!["code:eosio".to_string()], "code:eosio"), true);
        assert_eq!(is_match(vec!["code:eosio".to_string()], "code:eosio.token"), false);
        assert_eq!(is_match(vec!["db.table:accounts".to_string()], "db.table:accounts"), true);
        assert_eq!(is_match(vec!["db.table:accounts".to_string(), "db.table:stat".to_string()], "db.table:accounts || db.table.stat"), true);
        assert_eq!(is_match(vec!["db.table:accounts".to_string(), "db.table:stat".to_string()], "db.table:stat"), true);
        assert_eq!(is_match(vec!["db.table:accounts".to_string(), "db.table:stat".to_string()], "db.table:accounts"), true);
        assert_eq!(is_match(vec!["db.table:accounts".to_string(), "db.table:stat".to_string()], "db.table:producers"), false);
    }
}
