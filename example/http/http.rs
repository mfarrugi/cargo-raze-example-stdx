extern crate reqwest;

fn main() {
    use std::io::Read;

    let mut resp = reqwest::get("https://www.rust-lang.org").unwrap();
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content);
    println!("{}", content);
}
