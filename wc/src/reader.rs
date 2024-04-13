use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

pub struct Reader; 

impl Reader {

    pub fn get_lines(path: String) -> std::io::Lines<BufReader<File>> {

        let file = File::open(path).expect("Couldn't find the file");
        let buf_reader = BufReader::new(file);
        buf_reader.lines();

        // return buf_reader;
    }

}