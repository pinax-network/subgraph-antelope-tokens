use substreams::{matches_keys_in_parsed_expr, pb::substreams::Clock};

// Clock to date string
// ex: Clock => 2015-07-30
pub fn to_date(clock: &Clock) -> String {
    let timestamp = clock.timestamp.as_ref().expect("missing timestamp");
    timestamp.to_string().split('T').next().expect("missing date").to_string()
}

// Timestamp to date conversion
// ex: 2015-07-30 => 2015-07
pub fn block_date_to_month(block_date: &str) -> String {
    match block_date.split('-').take(2).collect::<Vec<&str>>().join("-").as_str() {
        date => date.to_string(),
    }
}

pub fn block_date_to_year(block_date: &str) -> String {
    match block_date.split('-').next() {
        Some(date) => date.to_string(),
        None => "".to_string(),
    }
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
    fn test_block_date_to_month() {
        assert_eq!(block_date_to_month("2015-07-30"), "2015-07");
        assert_eq!(block_date_to_month("2020-01-01"), "2020-01");
        assert_eq!(block_date_to_month("1999-12-31"), "1999-12");
        assert_eq!(block_date_to_month("2000-02-29"), "2000-02");
    }

    #[test]
    fn test_block_date_to_year() {
        assert_eq!(block_date_to_year("2015-07-30"), "2015");
        assert_eq!(block_date_to_year("2020-01-01"), "2020");
        assert_eq!(block_date_to_year("1999-12-31"), "1999");
        assert_eq!(block_date_to_year("2000-02-29"), "2000");
    }

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
