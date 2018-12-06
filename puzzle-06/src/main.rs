
use std::{
    env,
    fs,
    io::{
        self,
        Read,
        BufRead,
    },
    time,
};

fn main() {
    let run = env::args().nth(1).unwrap_or("1".to_string());
    if run == "1" {
        match run1() {
            Ok(()) => {},
            Err(ref err) => eprintln!("{:?}", err),
        }
    } else if run == "2" {
        match run2() {
            Ok(()) => {},
            Err(ref err) => eprintln!("{:?}", err),
        }
    }
}

fn run1() -> Result<(), failure::Error> {
    let file = fs::File::open("input.txt")?;
    let mut input = io::BufReader::new(file);

    let text = {
        let mut string = String::new();
        input.read_to_string(&mut string)?;
        string
    };
    let lines = input.lines();

    Ok(())
}

fn run2() -> Result<(), failure::Error> {
    let file = fs::File::open("input.txt")?;
    let _input = io::BufReader::new(file);

    Ok(())
}
