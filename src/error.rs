pub struct Error;

impl Error {
    pub fn write(message: &str, line: usize, column: usize) {
        println!("Error: {} at line {}, column {}", message, line, column);
    }
}
