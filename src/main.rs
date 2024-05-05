use fallible_iterator::FallibleIterator;

use mini_rust_compiler_components::lexer::Lexer;

fn main() {
    let lexer = Lexer::new("tests/test.mrs").unwrap();
    for token in lexer.iterator() {
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
