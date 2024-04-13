
pub struct WordCount {
    charactors: usize,
    words: usize,
    lines: u32
}

impl WordCount {

    pub fn initialize() -> WordCount {
        WordCount {
            charactors: 0,
            words: 0,
            lines: 0,
        }
    }

    pub fn count(&mut self, text: &str){
        self.lines += 1;
        self.words = text.split_whitespace().count();
        self.charactors = text.len();
    }

    pub fn print(&self){
        println!("Words = {}", self.words);
        println!("Charactors = {}", self.charactors);
        println!("Lines = {}", self.lines);
    }

}