use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    tab_width: u32,
}

impl Config {
    pub fn new() -> Config {
        Config { tab_width: 4}
    }
}

pub fn run(filename: &str, config: &Config) {
    println!("{} {:?}", filename, config);
}

pub mod io {

}
