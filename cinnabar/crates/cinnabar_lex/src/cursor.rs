pub struct Cursor<'cursor> {
    code: &'cursor [char],
    pub(crate) current: usize,
}

impl<'cursor> Cursor<'cursor> {
    pub fn new(code: &'cursor [char]) -> Self {
        Cursor { code, current: 0 }
    }
    pub fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.char_at(0)
        }
    } 
    pub fn next(&self) -> char {
        if self.current + 1 >= self.code.len() {
            '\0'
        } else {
            self.char_at(1)
        }
    } 
    pub fn is_at_end(&self) -> bool {
        self.current >= self.code.len()
    }
    pub fn char_at(&self, offset: usize) -> char {
        let index = self.current + offset;
        if self.code.len() > index {
            self.code[index]
        } else {
            '\0'
        }
    }
}
