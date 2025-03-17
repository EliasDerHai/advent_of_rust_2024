use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader.lines().collect()
}

pub fn read_chars<P>(filename: P) -> io::Result<impl Iterator<Item=char>>
    where
        P: AsRef<Path>,
{
    File::open(filename)
        .map(|f| io::BufReader::new(f))
        .map(|buf_reader| buf_reader.bytes()
            .map(|byte| byte.unwrap() as char))
}

pub fn read_string<P>(filename: P) -> io::Result<String>
    where
        P: AsRef<Path>,
{
    let mut content = String::new();
    let file = File::open(filename)?;
    let mut buf_reader = io::BufReader::new(file);
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

