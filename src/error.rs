#[derive(Debug)]
pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    pub fn error(line: usize, message: String) -> LoxError {
        LoxError { line, message }
    }

    pub fn report(&mut self, location: String) {
        eprintln!("[line {}] Error{}: {}", self.line, location, self.message);
    }
}
