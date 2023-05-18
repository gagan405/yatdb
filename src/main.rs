use std::fs::File;
use std::io;
use std::io::{BufReader, Error, SeekFrom};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let file = "testFile.txt";
    println!("Going to read from the file: {file}");

    println!("1st way:");

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


    println!("2nd way:");

    match read_bytes(file) {
        Ok(bytes) => {
            for byte in bytes {
                if let Ok(b) = byte {
                    print!("{}", b as char);
                }
            }
        },
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };

    println!("3rd way:");
    println!();

    let file = File::open(file).expect("Problem opening file!");
    let size = file.metadata().unwrap().len();
    let buffer_len = 20;
    let num_of_chunks = size/buffer_len + 1;

    let mut buf_reader = BufReader::new(file);

    for chunk_num in 0..num_of_chunks {
        let to_read = if chunk_num == num_of_chunks - 1 && size < buffer_len * (chunk_num + 1) {
            size - (chunk_num * buffer_len)
        } else {
            buffer_len
        } as usize;

        match read_partial(buf_reader.by_ref(), chunk_num * buffer_len, to_read) {
            Ok(v) => println!("{}", String::from_utf8(v).expect("Found invalid UTF-8")),
            Err(error) => panic!("Problem reading the file: {:?}", error)
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn read_bytes<P>(filename: P) -> io::Result<io::Bytes<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).bytes())
}


fn read_partial(mut buf_reader: &mut BufReader<File>, start: u64, offset: usize) -> Result<Vec<u8>, Error> {
    let mut buf = vec![0u8; offset];

    buf_reader.seek(SeekFrom::Start(start))?;
    buf_reader.read_exact(&mut buf)?;

    Ok(buf)
}
