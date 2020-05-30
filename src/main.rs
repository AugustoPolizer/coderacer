use std::env;
use std::process;

pub struct Config {
    pub tab_width: u32,
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
    println!("{}", filename)
}

