use std::env;
use std::fs;
use std::process;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    tab_width: u32,
}

impl Config {
    fn new() -> Config {
        Config { tab_width: 4}
    }
}

fn main() {
    let args_iter = env::args();
    
    let filename = match args_iter.skip(1).next(){
        Some(arg) => arg,
        None => {
            eprintln!("Insufficient arguments: missing the filename");
            process::exit(1);
        }
    };

    let config = match fs::read_to_string(".coderacer.json") {
        Ok(config_str) => {
            let parsed_config : Config = match serde_json::from_str(&config_str) {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("Parsing error on config file: {}", e);
                        Config::new()
                    }
            };
            parsed_config
        },
        Err(_) => {
            Config::new()
        }
    };

    println!("{} {:?}", filename, config);

    println!("{:?}", std::env::current_exe());
}

