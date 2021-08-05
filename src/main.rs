use std::env;
use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{}", args[1])

    let mut fl = match File::open(&args[1]) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => panic!("file doesn't exist"),
            _ => panic!("file could not be opened"),
        },
    };

    let mut msg = String::new();
    fl.read_to_string(&mut msg).unwrap();

    println!("{}", msg);
}
