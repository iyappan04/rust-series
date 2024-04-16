// unwrap with example code. 

// Unwrap in Rust returns the result of the operation for Option and Result enums. If unwrap encounters an error Err or a None, it will panic and stop the program execution.

fn main() {
    let paths = std::fs::read_dir("/home/user").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display());

    }
}


pub fn unwrap(self) -> T {
    match self {
          Ok(t) => t,
          Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", e),
    }
}
  
  
// pub fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<ReadDir>