use serde::{Deserialize, Serialize};
use std::io::{self, stdin, stdout, Write};
use termion::raw::IntoRawMode;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    tab_width: u8,
}

impl Config {
    pub fn new() -> Config {
        Config { tab_width: 4 }
    }
}

pub struct Screen {
    lines: u16,
    start_line: usize
}

impl Screen {
    pub fn new() -> Result<Screen, io::Error> {
        let size = termion::terminal_size()?;
        Ok(Screen { 
            lines: size.1, 
            start_line: 0
        })
    }
}

pub struct Game {
    current_line: usize,
    current_col: usize,
    cursor: (u16, u16),
}

impl Game {
    pub fn new() -> Game {
        Game {
            current_col: 0,
            current_line: 0,
            cursor: (1, 1),
        }
    }
}

pub fn run(filename: &str, config: &Config) -> Result<(), io::Error> {
    // Initializations
    let buffer = file_processing::read_file_to_buffer(filename, config.tab_width)?;
    let mut screen = Screen::new()?;
    let mut game = Game::new();
    let mut stdout = stdout().into_raw_mode()?;

    write!(
        stdout,
        "{} {}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )?;
    text_viewer::refresh_screen(&mut stdout, &buffer, screen.lines, 0)?;
    write!(stdout, "{}", termion::cursor::Goto(1, 1))?;
    stdout.flush().unwrap();

    let stdin = stdin();
    input_layer::wait_input(stdin, &mut stdout, &mut game, &buffer, & mut screen)?;
    Ok(())
}

mod input_layer {

    use super::{game_operations, Game, Screen};
    use std::io::{Stdin, Stdout};
    use termion::{event::Key, input::TermRead};

    pub fn wait_input(
        stdin: Stdin,
        stdout: &mut Stdout,
        game: &mut Game,
        buffer: &Vec<String>,
        screen: & mut Screen
    ) -> Result<(), std::io::Error> {
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('q') => break,
                Key::Char(c) => {
                    let is_correct = game_operations::check_user_input(c, buffer, game);
                    if is_correct {
                        let is_finish = game_operations::next_position(buffer, game, stdout, screen)?;
                        if is_finish {
                            break;
                        }
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }
}

mod game_operations {

    use super::{Game, Screen, text_viewer};
    use std::io::{Stdout, Write};
    use termion;

    pub fn check_user_input(input: char, buffer: &Vec<String>, game: &Game) -> bool {
        if input == buffer[game.current_line].chars().nth(game.current_col).unwrap() {
            true
        } else {
            false
        }
    }

    pub fn next_position(
        buffer: &Vec<String>,
        game: &mut Game,
        stdout: &mut Stdout,
        screen: & mut Screen
    ) -> Result<bool, std::io::Error> {

        if game.current_col + 1 >= buffer[game.current_line].chars().count() {
            let mut lines_down = 0;
            loop {
                if buffer.len() <= game.current_line + 1 {
                    return Ok(true);
                }
                lines_down += 1; 
                game.current_line += 1;
                game.cursor.1 += 1;
                game.cursor.0 = 1;
                if !buffer[game.current_line].is_empty() {
                    break;
                }
            }
            game.cursor.0 = skip_indentation(&buffer[game.current_line]);
            game.current_col = game.cursor.0 as usize - 1;
            if game.current_line > screen.lines as usize -1 {
                screen.start_line += lines_down; 
                text_viewer::refresh_screen(stdout, buffer, screen.lines, screen.start_line)?;
            }
        } else {
            game.current_col += 1;
            game.cursor.0 += 1;
        }

        write!(
            stdout,
            "{}",
            termion::cursor::Goto(game.cursor.0, game.cursor.1)
        )?;
        stdout.flush().unwrap();
        Ok(false)
    }

    fn skip_indentation(line: &str) -> u16{
        let mut index = 0;
        for c in line.chars() {
            index += 1;
            if ! c.is_whitespace() {
                break;
            }
        }
        index
    }
}

mod file_processing {
    use std::fs;

    pub fn read_file_to_buffer(
        filename: &str,
        tab_width: u8,
    ) -> Result<Vec<String>, std::io::Error> {
        let file_content = fs::read_to_string(filename)?;

        let mut buffer = Vec::new();
        for lines in file_content.lines() {
            let mut line = String::new();
            for c in lines.chars() {
                if c == '\t' {
                    let mut spaces = 0;
                    while spaces < tab_width && spaces < 8 {
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

    pub fn refresh_screen(
        stdout: &mut io::Stdout,
        buffer: &Vec<String>,
        screen_lines: u16,
        starting_row: usize,
    ) -> Result<(), std::io::Error> {
        let mut buffer_output = String::new();
        for row_index in starting_row..starting_row + screen_lines as usize - 1{
            if row_index as usize >= buffer.len() {
                break;
            }
            buffer_output.push_str(&buffer[row_index as usize]);
            buffer_output.push_str("\n\r");
        }
        if starting_row + (screen_lines as usize) <= buffer.len() {
            buffer_output.push_str(&buffer[starting_row + screen_lines as usize - 1]);
        }
        write!(stdout, "{}", buffer_output)?;
        stdout.flush().unwrap();
        Ok(())
    }
}
