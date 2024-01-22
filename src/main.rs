use mini_rust_compiler_components::lexer::Lexer;

fn main() {
    let lexer = Lexer::new("tests/test.mrs").unwrap();
    for token in lexer {
        match token {
            Ok(token) => {
                println!("{}", token);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
