//controls byte position tracking
//responsible for moving through the input source code
//provides methods to peek and advance through characters


pub struct Cursor<'a> {
    pub source: &'a str,
    pub position: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source,position: 0 }
    }

    // checking the next char
    pub fn peek(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }
    ///moves by one and returns next char
    pub fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += ch.len_utf8();
        Some(ch)
    }

    ///byte position in the input.
    pub fn position(&self) -> usize {
        self.position
    } self.position >= self.source.len()

}
