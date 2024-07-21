use std::str::Chars;

pub struct Cursor<'a> {
    len: usize,
    chars: Chars<'a>,
    prev: char,
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            len: input.len(),
            chars: input.chars(),
            prev: EOF_CHAR,
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.chars.as_str()
    }

    pub(crate) fn prev(&self) -> char {
        self.prev
    }

    /// Getting `EOF_CHAR` doesn't always mean actual end of file,
    /// it should be checked with `is_eof` method.
    pub fn peek_first(&self) -> char {
        self.chars.clone().nth(0).unwrap_or(EOF_CHAR)
    }

    pub(crate) fn peek_second(&self) -> char {
        self.chars.clone().nth(1).unwrap_or(EOF_CHAR)
    }

    pub fn peek_third(&self) -> char {
        self.chars.clone().nth(2).unwrap_or(EOF_CHAR)
    }

    /// Checks if there is nothing more to consume.
    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Returns amount of already consumed symbols.
    pub(crate) fn len_advanced(&self) -> u32 {
        (self.len - self.chars.as_str().len()) as u32
    }

    /// Resets the number of bytes consumed to 0.
    pub(crate) fn reset_len(&mut self) {
        self.len = self.chars.as_str().len();
    }

    /// Moves to the next character.
    pub(crate) fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.prev = c;
        Some(c)
    }

    pub(crate) fn bump_n(&mut self, n: u32) {
        for _ in 0..n {
            self.bump();
        }
    }

    /// Eats symbols while predicate returns true or until the end of file is reached.
    pub(crate) fn bump_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek_first()) && !self.is_eof() {
            self.bump();
        }
    }
}
