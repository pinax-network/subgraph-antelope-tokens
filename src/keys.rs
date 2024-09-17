use antelope::{ExtendedSymbol, Name};

pub fn balance_key(token: &ExtendedSymbol, owner: &Name) -> String {
    format!("{}:{}", token, owner)
}
