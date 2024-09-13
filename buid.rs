fn main() {
    substreams_antelope::Abigen::new("Contract", "abi/eosio.token.abi.json")
        .expect("failed to load abi")
        .generate()
        .expect("failed to generate contract")
        .write_to_file("src/abi.rs")
        .expect("failed to write contract");
}
