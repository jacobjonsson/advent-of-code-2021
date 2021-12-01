use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub fn read_input(p: &str) -> String {
    let f = File::open(p).unwrap();
    let mut file = BufReader::new(&f);
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    return input.trim_start().trim_end().to_string();
}
