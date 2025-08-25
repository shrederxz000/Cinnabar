use std::path::PathBuf;
use std::fs;
use cinnabar_lex::lexer::Lexer; // твой crate с лексером
use cinnabar_lex::tokens::Token;


fn main() {
    // 1. Читаем файл (или просто строку)
    let source = r#"
    void main() {
        set x := 42;
        set x := "string"
        let y :hex= 0xFF; //255
        let y = 0xA6 
        /* block comment */
        const PI:float = 3.14;
    }
    "#;

    // 2. Преобразуем строку в Vec<char>
    let chars: Vec<char> = source.chars().collect();

    // 3. Создаем лексер
    let file_path = PathBuf::from("<example>.cin");
    let lexer = Lexer::new(&chars, &file_path);

    // 4. Лексим
    let tokens: Vec<Token> = lexer.lex();

    // 5. Выводим
    for token in tokens {
        println!("{:?} -> {:?}", token.token_type, token.value);
    }
}

