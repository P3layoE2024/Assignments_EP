
// Code extracted from Module 3/04 Easy Assignment
/////////////////////////////////////////////////////////////////
use std::fs::File;
use std::io::prelude::*;

struct Config {
    username: String,
    api_key: String,
    port: u16,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let username = lines.next().unwrap().to_string();
        let api_key = lines.next().unwrap().to_string();
        let port = lines.next().unwrap().parse().unwrap();

        Config { username, api_key, port }
    }
}

fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("username: {}", config.username);
    println!("api key: {}", config.api_key);
    println!("port: {}", config.port);
}
/////////////////////////////////////////////////////////////////


// Main
fn main() {
    reading_from_file();
}
