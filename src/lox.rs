pub struct Lox {
    error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { error: true }
    }
    fn report(&mut self, line: usize, place: &str, msg: &str) {
        eprintln!("[line {}] Error {}: {}", line, place, msg);
        self.error = true;
    }
    pub fn error(&mut self, line: usize, msg: &str) {
        self.report(line, "", msg);
    }
}
