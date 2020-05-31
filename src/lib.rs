use serde::{Deserialize, Serialize};
use termion::raw::IntoRawMode;

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
    let file_content = file_preprocessing::read_file(filename)?;
    let buffer = file_preprocessing::parse_file_to_buffer(&file_content);
    
    let mut stdout = std::io::stdout().into_raw_mode()?;
    print!("{} {}", termion::clear::All, termion::cursor::Goto(1,1));
    text_viewer::write_buffer_to_screen(& mut stdout, & buffer, config.tab_width)?;
    
    Ok(())
}

pub fn input_layer() {
    
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
    use std::io::{self, Write};

    pub fn write_buffer_to_screen(stdout : & mut io::Stdout,text_buffer: & Vec<& str>, tab_width: u32) -> Result<(), std::io::Error>{
        let mut buffer_output = String::new(); 
        for lines in text_buffer {
            for c in lines.chars() {
                if c == '\t' {
                    let mut spaces = 0;
                    while spaces < tab_width {
                        buffer_output.push(' ');
                        spaces += 1;
                    }
                }
                else {
                    buffer_output.push(c);
                }
            }
            buffer_output.push('\n');
            buffer_output.push('\r');
        } 

        write!(stdout, "{}", buffer_output)?;
        Ok(())
    }
}
