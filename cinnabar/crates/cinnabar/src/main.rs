use std::path::PathBuf;
use std::fs;
use cinnabar_lex::lexer::Lexer; // твой crate с лексером
use cinnabar_lex::tokens::Token;


fn main() {
    // 1. Читаем файл
    let path = "tests/binary_search.cbar"; // путь к исходнику
    let contents = fs::read_to_string(path).expect("Cannot read file");

    // 2. Превращаем в вектор символов
    let chars: Vec<char> = contents.chars().collect();

    // 3. Создаем лексер
    let file_path = PathBuf::from(path);
    let mut lexer = Lexer::new(&chars, &file_path);

    // 4. Лексим
    let tokens: Vec<Token> = lexer.lex();

    // 5. Выводим все токены в читаемом виде
    for token in tokens {
        println!("{:?} -> {:?}", token.token_type, token.value);
    }
}

