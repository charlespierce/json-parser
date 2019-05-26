pub struct Input {
    source: String,
    cur: Option<char>,
    pos: usize,
}

impl Input {
    pub fn new(source: String) -> Self {
        let first_char = source.chars().next();

        Input {
            source,
            cur: first_char,
            pos: 0,
        }
    }

    pub fn cur(&self) -> Option<char> {
        self.cur
    }

    pub fn bump(&mut self) {
        if let Some(c) = self.cur {
            self.pos += c.len_utf8();
            self.cur = self.source[self.pos..].chars().next();
        } else {
            unreachable!("bump() must not be called if cur is None");
        }
    }
}
