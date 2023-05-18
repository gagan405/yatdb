use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let file = "/Users/gagan/dev/yatdb/testFile1.txt";
    println!("Going to read from the file: {file}");

    match read_lines(file) {
        Ok(lines) => {
            for line in lines {
                if let Ok(l) = line {
                    println!("{}", l);
                }
            }
        },
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
