use serde::{Deserialize, Serialize};
use termion::{raw::IntoRawMode};
use std::io::{self, stdout, stdin, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    tab_width: u8,
}

impl Config {
    pub fn new() -> Config {
        Config { tab_width: 4}
    }
}

pub struct Screen {
    lines: u16,
    cols: u16,
}

impl Screen {
    pub fn new() -> Result<Screen, io::Error> {
        let size = termion::terminal_size()?;
        Ok(Screen {
            cols: size.0,
            lines: size.1,
        })
    }
}

pub struct Game {
    current_line: usize,
    current_col: usize,
    cursor:(u16, u16)
}

impl Game {
    pub fn new() -> Game {
        Game {
            current_col: 0,
            current_line:0,
            cursor: (1,1)
        }
    }
}
pub fn run(filename: &str, config: &Config) -> Result<(), std::io::Error>{
    // Initializations  
    let buffer = file_processing::read_file_to_buffer(filename, config.tab_width)?;
    let screen = Screen::new()?;
    let mut game = Game::new();
    let mut stdout = stdout().into_raw_mode()?;

    write!(stdout, "{} {}", termion::clear::All, termion::cursor::Goto(1,1))?;
    text_viewer::refresh_screen(& mut stdout, &buffer, screen.lines, 0)?;
    write!(stdout, "{}", termion::cursor::Goto(1,1))?;
    stdout.flush().unwrap();

    let stdin = stdin(); 
    input_layer::wait_input(stdin, & mut stdout, & mut game, & buffer)?;
    Ok(())
}

mod input_layer {

    use termion::{event::Key, input::TermRead};
    use std::io::{Stdin, Stdout}; 
    use super::{ Game, game_operations};

    pub fn wait_input(stdin: Stdin, stdout: & mut Stdout, game: & mut Game, buffer: & Vec<String>) -> Result<(), std::io::Error>{

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('q') => break,
                Key::Char(c)   => {
                    let is_correct = game_operations::check_user_input(c, buffer, game);
                    if is_correct {
                        game_operations::next_position(game, stdout)?;
                    }
                },
                _ => println!("other"), 
            } 
        }  
        Ok(())
    }
}

mod game_operations {
    use super::Game;
    use std::io::{Write, Stdout };
    use termion;
    
    pub fn check_user_input(input: char, buffer: & Vec<String>, game: & Game) -> bool {
        if input == buffer[game.current_line].chars().nth(game.current_col).unwrap() { true } else { false } 
    }

    pub fn next_position(game: & mut Game, stdout: & mut Stdout) -> Result<(), std::io::Error>{
        game.cursor.0 += 1;
        game.current_col += 1;
        write!(stdout, "{}", termion::cursor::Right(1))?;
        stdout.flush().unwrap();
        Ok(())
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
        for row_index in starting_row .. starting_row + screen_rows as u32 - 1 { 
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
