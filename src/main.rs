use clap::{command, Arg};
use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("string")
            .short('s')
            .long("string")
            .required(true))
        .get_matches();

    let s = matches.get_one::<String>("string");
    println!("s = {:?}", s.unwrap());
    let mut hasher = Sha256::new();
    hasher.input_str(s.unwrap());
    let hex = hasher.result_str();
    println!("sh256(s) = {}", &hex);
    println!("first 25 = {}", truncate(&hex, 25));
    println!("first 8  = {}", truncate(&hex, 8));
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

#[cfg(test)]
mod tests {
    use sha2::Sha256;
    use super::*;

    #[test]
    fn sha256() {
        let mut hasher = Sha256::new();
        hasher.input_str("hello world");
        let hex = hasher.result_str();
        assert_eq!(hex,
                   concat!("b94d27b9934d3e08a52e52d7da7dabfa",
                   "c484efe37a5380ee9088f7ace2efcde9"));
    }
}
