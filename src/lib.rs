use serde::{Deserialize, Serialize};
use termion::{raw::IntoRawMode, event::Key, input::TermRead};
use std::io::{self, stdout, stdin};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    tab_width: u8,
}

impl Config {
    pub fn new() -> Config {
        Config { tab_width: 4}
    }
}

struct Screen {
    rows: u16,
    cols: u16,
    cursor:(u16, u16)
}

impl Screen {
    pub fn new() -> Result<Screen, io::Error> {
        let size = termion::terminal_size()?;
        Ok(Screen {
            cols: size.0,
            rows: size.1,
            cursor: (1,1)
        })
    }
}




pub fn run(filename: &str, config: &Config) -> Result<(), std::io::Error>{
    // Initializations  
    let buffer = file_processing::read_file_to_buffer(filename, config.tab_width)?;
    let screen = Screen::new()?;
    let mut stdout = stdout().into_raw_mode()?;

    print!("{} {}", termion::clear::All, termion::cursor::Goto(1,1));
    text_viewer::refresh_screen(& mut stdout, &buffer, screen.rows, 0)?;

    let stdin = stdin(); 
    input_layer(stdin);
    Ok(())
}



fn input_layer(stdin: io::Stdin) {
  
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => break,
            Key::Char(c)   => println!("{}", c),
            _              => println!("Other"),
        } 
    }  
}

mod file_processing {
    use std::fs;

    pub fn read_file_to_buffer(filename: &str, tab_width: u8) -> Result<Vec<String>, std::io::Error >{
        let file_content = fs::read_to_string(filename)?;
        
        let mut buffer = Vec::new();
        for lines in file_content.lines(){
            let mut line = String::new();
            for c in lines.chars() {
                if c == '\t' {
                    let mut spaces = 0;
                    while spaces < tab_width && spaces < 8{
                        line.push(' ');
                        spaces += 1;
                    }
                } else {
                    line.push(c);
                }
            }
            buffer.push(line);
        }
        Ok(buffer) 
    }
}

mod text_viewer {
    use std::io::{self, Write};

    pub fn refresh_screen(stdout : & mut io::Stdout,buffer: & Vec<String>, screen_rows: u16, starting_row: u32)
        -> Result<(), std::io::Error> {
        let mut buffer_output = String::new(); 
        for row_index in starting_row .. starting_row + screen_rows as u32 + 1{ 
            if row_index as usize >= buffer.len() {
                break;
            }
            buffer_output.push_str(&buffer[row_index as usize]);
            buffer_output.push_str("\n\r");
        } 

        write!(stdout, "{}", buffer_output)?;
        Ok(())
    }
}
