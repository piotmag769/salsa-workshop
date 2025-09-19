use crate::ir::Op;

pub struct Lexer<'a> {
    source_text: &'a str,
    position: usize,
}

impl<'db> Lexer<'db> {
    pub fn new(source_text: &'db str) -> Self {
        Self {
            source_text,
            position: 0,
        }
    }

    pub fn can_consume(&self) -> bool {
        self.peek().is_some()
    }

    pub fn cell_cords(&mut self) -> Option<(u32, u32)> {
        self.ch('$')?;
        let row = self.number()?;
        self.ch(':')?;
        let col = self.number()?;
        Some((row, col))
    }

    pub fn op(&mut self) -> Option<Op> {
        if self.ch('+').is_some() {
            Some(Op::Add)
        } else if self.ch('-').is_some() {
            Some(Op::Subtract)
        } else {
            None
        }
    }

    pub fn number(&mut self) -> Option<u32> {
        self.skip_whitespace();

        let mut s = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_numeric() {
                s.push(ch);
                self.consume(ch);
            } else {
                break;
            }
        }

        if s.is_empty() {
            None
        } else {
            str::parse(&s).ok()
        }
    }

    fn ch(&mut self, c: char) -> Option<()> {
        self.skip_whitespace();
        match self.peek() {
            Some(p) if c == p => {
                self.consume(c);
                Some(())
            }
            _ => None,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.consume(ch);
            } else {
                break;
            }
        }
    }

    fn consume(&mut self, ch: char) {
        debug_assert!(self.peek() == Some(ch));
        self.position += ch.len_utf8();
    }

    fn peek(&self) -> Option<char> {
        self.source_text[self.position..].chars().next()
    }
}
