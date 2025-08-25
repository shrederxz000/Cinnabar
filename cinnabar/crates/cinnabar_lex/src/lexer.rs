use crate::cursor::Cursor;
use crate::tokens::{TokenType, Token};
use std::collections::HashMap;
use std::path::PathBuf;
use cinnabar_common::{address::Address, error, errors::Error};

/// структура для создания экземпляра лексера 
pub struct Lexer<'file_path, 'cursor> {
    line: u64,
    column: u16,
    cursor: Cursor<'cursor>,
    file_path: &'file_path PathBuf,
    tokens: Vec<Token>,
    keywords: HashMap<&'static str, TokenType>,
}

impl<'file_path, 'cursor> Lexer<'file_path, 'cursor> {
    /// создает новый список токенов
    pub fn new(code: &'cursor [char], file_path: &'file_path PathBuf) -> Self {
        // все клюяевые слова
        let keywords_map = HashMap::from([
            ("package", TokenType::Package),
            ("plug", TokenType::Plug),
            ("let", TokenType::Let),
            ("set", TokenType::Set),
            ("const", TokenType::Const),
            ("as", TokenType::As),
            ("match", TokenType::Match),
            ("loop", TokenType::Loop),
            ("continue", TokenType::Continue),
            ("break", TokenType::Break),
            ("void", TokenType::Void),
            ("take", TokenType::Take),
            ("return", TokenType::Take),
            ("lambda", TokenType::Lambda),
            ("scatch", TokenType::Scatch),
            ("impl", TokenType::Impl),
            ("iface", TokenType::Iface),
            ("new", TokenType::New),
            ("try", TokenType::Try),
            ("catch", TokenType::Catch),
            ("throw", TokenType::Throw),
            ("macro", TokenType::Macro),
            ("quote", TokenType::Quote),
            ("unquote", TokenType::Unquote),
            ("comptime", TokenType::Comptime),

            ("priv", TokenType::Private),
            ("pub", TokenType::Public),
            ("static", TokenType::Static),
            ("async", TokenType::Async),
            
            ("Nil", TokenType::Nil),
            ("int", TokenType::Int),
            ("int8", TokenType::Int8),
            ("int16", TokenType::Int16),
            ("int32", TokenType::Int32),
            ("int64", TokenType::Int64),
            ("int128", TokenType::Int128),
            ("uint", TokenType::Uint),
            ("uint8", TokenType::Uint8),
            ("uint16", TokenType::Uint16),
            ("uint32", TokenType::Uint32),
            ("uint64", TokenType::Uint64),
            ("uint128", TokenType::Uint128),
            ("float", TokenType::Float),
            ("float32", TokenType::Float32),
            ("float64", TokenType::Float64),
            ("bin", TokenType::Binary),
            ("hex", TokenType::Hexonal),
            ("string", TokenType::String),
            ("char", TokenType::Char),
            ("bool", TokenType::Bool),
            ("false", TokenType::False),
            ("true", TokenType::True),
            ("list", TokenType::List),
            ("tuple", TokenType::Tuple),
            ("array", TokenType::Array),
            ("hash", TokenType::Hash),
            ("hashlist", TokenType::Hashlist),
            
        ]);
        // Lexer
        Lexer {
            line: 1,
            column: 0,
            cursor: Cursor::new(code),
            file_path,
            tokens: vec![],
            keywords: keywords_map,
        }
    }

    /// Converts source code represented as `&'cursor [char]`
    /// To a `Vec<Token>` - tokens list.
    #[allow(clippy::nonminimal_bool)]
    pub fn lex(mut self) -> Vec<Token> {
        if !self.tokens.is_empty() {
            panic!("tokens len already > 0. report this error to the developer.")
        }
        while !self.cursor.is_at_end() {
            let ch = self.advance();
            match ch {
                '+' => { 
                    if self.is_match('=') {
                        self.add_tk(TokenType::AssignAdd, "+=");
                    }
                    else {
                        self.add_tk(TokenType::Add, "+");
                    }
                }
                '-' => { 
                    if self.is_match('=') {
                        self.add_tk(TokenType::AssignSub, "-=");} 
                    else {
                        self.add_tk(TokenType::Sub, "-");
                    }
                }
                '*' => { 
                    if self.is_match('=') {
                        self.add_tk(TokenType::AssignMul, "*=");
                    } 
                    else if self.is_match('*') {
                        if self.is_match('=') {
                            self.add_tk(TokenType::AssignPow, "**=");
                        }
                        else {
                            self.add_tk(TokenType::Pow, "**");
                        }
                    }
                    else {
                        self.add_tk(TokenType::Mul, "*");
                    }
                }
                '%' => {
                    if self.is_match('=') {
                        self.add_tk(TokenType::AssignMod, "%=");
                    } 
                    else {
                        self.add_tk(TokenType::Mod, "%");
                    }
                }
                '/' => {
                    if self.is_match('=') {
                        self.add_tk(TokenType::AssignDiv, "/=");
                    }
                    else if self.is_match('/') {
                        while !self.is_match('\n') && !self.cursor.is_at_end() {
                            self.advance();
                        }
                        self.new_line();
                    }
                    else if self.is_match('*') {
                        while !(
                            self.cursor.peek() == '*' 
                            && self.cursor.next() == '/'
                        )&& !self.cursor.is_at_end(){
                            if self.is_match('\n') {
                                self.new_line();
                                continue;
                            }
                            self.advance();
                        }
                        self.advance(); 
                        self.advance();
                    } 
                    else {
                        self.add_tk(TokenType::Div, "/");
                    }
                }
                '(' => {
                    self.add_tk(TokenType::LParen, "(");
                }
                ')' => {
                    self.add_tk(TokenType::RParen, ")");
                }
                '{' => {
                    self.add_tk(TokenType::LBrace, "{");
                }
                '}' => {
                    self.add_tk(TokenType::RBrace, "}");
                }
                '[' => {
                    self.add_tk(TokenType::LBracket, "[");
                }
                ']' => {
                    self.add_tk(TokenType::RBracket, "]");
                }
                ',' => {
                    self.add_tk(TokenType::Comma, ",");
                }
                '.' => {
                    self.add_tk(TokenType::Dot, ".");
                }
                '?' => {
                    self.add_tk(TokenType::Question, "?");
                }
                '$' => {
                    self.add_tk(TokenType::Dollar, "$");
                }             
                ':' => {
                    if self.is_match('=') {
                        self.add_tk(TokenType::Assign, ":=");
                    } 
                    else {
                        self.add_tk(TokenType::Colon, ":")
                    }
                }
                ';' => {
                    self.add_tk(TokenType::Semi, ";");
                }
                '<' => {
                    if self.is_match('=') {
                        self.add_tk(TokenType::LessEq, "<=");
                    } 
                    else {
                        self.add_tk(TokenType::Less, "<");
                    }
                }
                '>' => {
                    if self.is_match('=') {
                        self.add_tk(TokenType::GreaterEq, ">=");
                    } 
                    else {
                        self.add_tk(TokenType::Greater, ">");
                    }
                }
                '!' => {
                    if self.is_match('=') {
                        self.add_tk(TokenType::NotEq, "!=");
                    } 
                        else {
                            self.add_tk(TokenType::Bang, "!");
                        }
                }
                '=' => {
                    if self.is_match('=') {
                        self.add_tk(TokenType::Equal, "==");
                    }
                    else if self.is_match('>'){
                        self.add_tk(TokenType::Arrow, "=>");
                    } 
                    else {
                        self.add_tk(TokenType::ReAssign, "=");
                    }
                }
                '\r' => {}
                '\t' => {}
                '\0' => {}
                ' ' => {}
                '\n' => {self.new_line();}
                '\'' => {
                    let tk = self.scan_char();self.tokens.push(tk)
                }
                '\"' => {
                    let tk = self.scan_string();self.tokens.push(tk)
                }
                _ => {
                    // number literal
                    if self.is_digit(ch) {
                        let tk = if self.cursor.peek() == 'x' {
                            self.scan_hexadecimal_number()
                        } else if self.cursor.peek() == 'b' {
                            self.scan_binary_number()
                        } else {self.scan_number(ch)};

                    self.tokens.push(tk);
                    }
                    // identifier or keyword
                    else if self.is_id_start(ch) {
                        let tk = self.scan_id_or_keyword(ch);
                        self.tokens.push(tk);
                    }
                    // unexpected character
                    else {
                        error!(
                        Error::own(
                        Address::new(self.line, self.column, self.file_path.clone()),
                        format!("unexpected char: {ch}"), format!("delete char: {ch}"),));
                    }
                }

            }
        }
        self.tokens
    }

    /// Scans string. Implies quote is already ate. East ending quote.
    fn scan_string(&mut self) -> Token {
        // String text
        let mut text: String = String::new();
        let span_start = self.column;

        while self.cursor.peek() != '\"' {
            let ch = self.advance();

            if ch == '\\' && self.cursor.peek() == '\"' {
                text.push(self.advance());
            } else {
                text.push(ch);
            }

            if self.cursor.is_at_end() || self.is_match('\n') {
                error!(Error::new(
                    Address::new(self.line, self.column, self.file_path.clone(),),
                    "unclosed string quotes.",
                    "did you forget \" symbol?",
                ));
            }
        }

        self.advance();
        let span_end = self.column;

        Token {
            token_type: TokenType::Text,
            value: text,
            address: Address::span(self.line, span_start..span_end, self.file_path.clone()),
        }
    }

    // scan char
    fn scan_char(&mut self) -> Token {
    // Char literal
    let mut text: String = String::new();
    let span_start = self.column;

    if self.cursor.peek() == '\'' {
        error!(Error::new(
            Address::new(self.line, self.column, self.file_path.clone()),
            "empty char literal.",
            "char must contain exactly one character or escape sequence.",
        ));
    }

    let ch = self.advance();

    if ch == '\\' {
        let esc = self.advance();
        text.push(ch);
        text.push(esc);
    } else {
        text.push(ch);
    }
    if self.cursor.peek() != '\'' {
        error!(Error::new(
            Address::new(self.line, self.column, self.file_path.clone()),
            "invalid char literal.",
            "char must contain exactly one character or escape sequence.",
        ));
    }

    self.advance();

    let span_end = self.column;

    Token {
        token_type: TokenType::Symbol,
        value: text,
        address: Address::span(self.line, span_start..span_end, self.file_path.clone()),
    }
}

    /// Scans decimal and integer numbers
    ///
    /// # Arguments
    /// * `start`: starting char of token
    ///
    fn scan_number(&mut self, start: char) -> Token {
        // Start of span
        let span_start = self.column;
        // Number text
        let mut text: String = String::from(start);
        // If number is float
        let mut is_float: bool = false;

        while self.is_digit(self.cursor.peek()) || self.cursor.peek() == '.' {
            if self.cursor.peek() == '.' {
                if self.cursor.next() == '.' {
                    break;
                }
                if is_float {
                    error!(Error::new(
                        Address::new(self.line, self.column, self.file_path.clone(),),
                        "couldn't parse number with two dots",
                        "check your code.",
                    ));
                }
                is_float = true;
                text.push(self.advance());

                continue;
            }
            text.push(self.advance());
            if self.cursor.is_at_end() {
                break;
            }
        }

        let span_end = self.column;

        Token {
            token_type: TokenType::Number,
            value: text,
            address: Address::span(self.line, span_start..span_end, self.file_path.clone()),
        }
    }

    /// Scans hexadecimal numbers `0x{pattern}`
    fn scan_hexadecimal_number(&mut self) -> Token {
        // Start of span
        let span_start = self.column;
        // Skip 'x'
        self.advance();
        // Number text
        let mut text: String = String::from("0x");

        while self.cursor.peek().is_digit(16) {
            text.push(self.advance());
            if self.cursor.is_at_end() {
                break;
            }
        }

        let span_end = self.column;

        Token {
            token_type: TokenType::Hex,
            value: text,
            address: Address::span(self.line, span_start..span_end, self.file_path.clone()),
        }
    }

   

    /// Scans binary numbers `0b{pattern}`
    fn scan_binary_number(&mut self) -> Token {
        // Start of span
        let span_start = self.column;
        // Skip 'b'
        self.advance();
        // Number text
        let mut text: String = String::from("0b");

        while self.cursor.peek().is_digit(2) {
            text.push(self.advance());
            if self.cursor.is_at_end() {
                break;
            }
        }

        let span_end = self.column;

        Token {
            token_type: TokenType::Bin,
            value: text,
            address: Address::span(self.line, span_start..span_end, self.file_path.clone()),
        }
    }

    /// Scans identifier, and checks if it is keyword.
    /// Returns token with kind Identifier or Keyword.
    ///
    /// # Arguments
    ///
    /// * `start`: starting char of token
    ///
    fn scan_id_or_keyword(&mut self, start: char) -> Token {
    let span_start = self.column;

    let mut text = String::from(start);
    while self.is_id_continue(self.cursor.peek()) {
        text.push(self.advance());
        if self.cursor.is_at_end() {
            break;
        }
    }

    let span_end = self.column;

    // Проверяем, не ключевое ли это слово
    let token_type = self.keywords
        .get(text.as_str())
        .copied()
        .unwrap_or(TokenType::Id);

    Token {
        token_type,
        value: text,
        address: Address::span(self.line, span_start..span_end, self.file_path.clone()),
    }
}

    /// Adds 1 to `line` and resets to zero `column`
    fn new_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    /// Eats character from cursor and returns it,
    /// adding 1 to `column` and `cursor.current`
    fn advance(&mut self) -> char {
        let ch: char = self.cursor.char_at(0);
        self.cursor.current += 1;
        self.column += 1;
        ch
    }

    /// проверка нынешного символа с ch;
    /// если равно ch, то вызавет advance то вернет true 
    #[allow(clippy::wrong_self_convention)]
    fn is_match(&mut self, ch: char) -> bool {
        if !self.cursor.is_at_end() && self.cursor.char_at(0) == ch {
            self.advance();
            return true;
        }
        false
    }

    /// добавляет токен в список токенов (тип, значение, адрес(строка, колонка, путь))
    fn add_tk(&mut self, token_type: TokenType, token_value: &str) {
        self.tokens.push(Token::new(
            token_type,
            token_value.to_string(),
            Address::new(self.line, self.column, self.file_path.clone()),
        ));
    }

    /// Проверяет символы от 0..9
    fn is_digit(&self, ch: char) -> bool {
        ch.is_ascii_digit()
    }

    /// Проверяет символы 'a..z', 'A..Z', '_'
    fn is_letter(&self, ch: char) -> bool {
        ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || (ch == '_')
    }
/// проверка стартового символа id
    fn is_id_start(&self, ch: char) -> bool {
        self.is_letter(ch) || ch == '_'
    }
/// проверка остальных вимволов id
    fn is_id_continue(&self, ch: char) -> bool {
        self.is_letter(ch) || self.is_digit(ch) || ch == '_'
    }
}






#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn lex(input: &str) -> Vec<Token> {
        let chars: Vec<char> = input.chars().collect();
        Lexer::new(&chars, &PathBuf::from("<test>.cin")).lex()
    }

    fn types(input: &str) -> Vec<TokenType> {
        lex(input).into_iter().map(|t| t.token_type).collect()
    }

    fn values(input: &str) -> Vec<String> {
        lex(input).into_iter().map(|t| t.value).collect()
    }

    #[test]
    fn keyword_let_assign_number() {
        let ts = types("let x := 1");
        assert_eq!(ts, vec![TokenType::Let, TokenType::Id, TokenType::Assign, TokenType::Number]);
    }

    #[test]
    fn identifier_rules_start_letter_or_underscore() {
        let ts = types("_a1 a_b2");
        assert_eq!(ts, vec![TokenType::Id, TokenType::Id]);
        let vs = values("_a1 a_b2");
        assert_eq!(vs, vec!["_a1".to_string(), "a_b2".to_string()]);
    }

    #[test]
    fn identifier_cannot_start_with_digit() {
        // ожидаем: число "1", затем идентификатор "abc"
        let ts = types("1abc");
        assert_eq!(ts, vec![TokenType::Number, TokenType::Id]);
        let vs = values("1abc");
        assert_eq!(vs, vec!["1".to_string(), "abc".to_string()]);
    }

    #[test]
    fn numbers_int_and_float() {
        let ts = types("0 3 3.14 42.0");
        assert_eq!(ts, vec![TokenType::Number, TokenType::Number, TokenType::Number, TokenType::Number]);
        let vs = values("0 3 3.14 42.0");
        assert_eq!(vs, vec!["0", "3", "3.14", "42.0"]);
    }

    #[test]
    fn numbers_hex_and_bin() {
        let ts = types("0xFF 0b1010");
        assert_eq!(ts, vec![TokenType::Hex, TokenType::Bin]);
        let vs = values("0xFF 0b1010");
        assert_eq!(vs, vec!["0xFF".to_string(), "0b1010".to_string()]);
    }

    #[test]
    fn string_basic_and_escaped_quote() {
        // твой scan_string съедает обратный слэш перед кавычкой и кладёт в значение именно кавычку
        // т.е. "a \" b" -> значение: a " b
        let toks = lex("\"hi\" \"a \\\" b\"");
        assert_eq!(toks[0].token_type, TokenType::Text);
        assert_eq!(toks[0].value, "hi");
        assert_eq!(toks[1].token_type, TokenType::Text);
        assert_eq!(toks[1].value, "a \" b");
    }

    #[test]
    fn char_basic_and_escape() {
        let toks = lex("'a' '\\n'");
        assert_eq!(toks[0].token_type, TokenType::Symbol);
        assert_eq!(toks[0].value, "a");
        assert_eq!(toks[1].token_type, TokenType::Symbol);
        // scan_char сохраняет и обратный слэш, и букву
        assert_eq!(toks[1].value, "\\n");
    }

    #[test]
    fn line_and_block_comments_are_ignored() {
        let ts = types("x // comment\n y");
        assert_eq!(ts, vec![TokenType::Id, TokenType::Id]);

        let ts2 = types("x/* block */y");
        assert_eq!(ts2, vec![TokenType::Id, TokenType::Id]);
    }

    #[test]
    fn arrow_and_reassign_and_eq() {
        // =>, =, ==
        let ts = types("a => b = c == d");
        assert_eq!(
            ts,
            vec![
                TokenType::Id, TokenType::Arrow, TokenType::Id,
                TokenType::ReAssign, TokenType::Id,
                TokenType::Equal, TokenType::Id
            ]
        );
    }

    #[test]
    fn mul_assign_token_kind_should_be_assignmul() {
        let ts = types("*=");
        assert_eq!(ts, vec![TokenType::AssignMul]); // сейчас у тебя будет AssignMod
    }

    #[test]
    fn pow_should_be_plain_pow_not_assignpow() {
        let ts = types("**");
        assert_eq!(ts, vec![TokenType::Pow]); // сейчас у тебя будет AssignPow
    }

}

