
mod reader;
mod wordcount;

use reader::Reader;
use wordcount::WordCount;



fn main() {

    let path = String::from("./file.txt");

    let lines = Reader::get_lines(path);

    let mut wordcount = WordCount::initialize();

    for line in lines {
        match line {
            Ok(text: String) => wordcount.count(&text),
            Err(e) => println!("Error {}", e)
        }
    }

    wordcount.print();

}
