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

pub fn run(filename: &str, config: &Config) -> Result<(), std::io::Error>{
    let file_content = io::read_file(filename)?;
    let buffer = io::parse_file(&file_content);

    println!("{:?}", config);
    println!("{}", buffer[0]);
    println!("{}", buffer[3]);

    Ok(())
}

pub mod io {
    use std::fs;

    pub fn read_file(filename: &str) -> Result<String, std::io::Error >{
        let file_content = fs::read_to_string(filename)?;
        Ok(file_content) 
    }

    pub fn parse_file<'a>(file_content: &'a str) -> Vec<&'a str>{
        let mut buffer = Vec::new();

        for line in file_content.lines() {
            buffer.push(line);
        }
        buffer
    }
 }
