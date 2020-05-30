
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

pub fn run(filename: &str, _config: &Config) -> Result<(), std::io::Error>{
    let file_content = file_preprocessing::read_file(filename)?;
    let buffer = file_preprocessing::parse_file_to_buffer(&file_content);

    text_viewer::write_buffer_to_screen(& buffer)?;

    Ok(())
}

pub mod file_preprocessing{
    use std::fs;

    pub fn read_file(filename: &str) -> Result<String, std::io::Error >{
        let file_content = fs::read_to_string(filename)?;
        Ok(file_content) 
    }

    pub fn parse_file_to_buffer<'a>(file_content: &'a str) -> Vec<&'a str>{
        let mut buffer = Vec::new();

        for line in file_content.lines() {
            buffer.push(line);
        }
        buffer
    }
}

pub mod text_viewer {
    extern crate termion;
    use std::io::Write;

    pub fn write_buffer_to_screen(buffer: & Vec<& str>) -> Result<(), std::io::Error>{
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        print!("{} {}", termion::clear::All, termion::cursor::Goto(1,1));
        for lines in buffer {
            handle.write(lines.as_bytes())?;
            handle.write("\n".as_bytes())?;
        } 
        Ok(())
    }
}
