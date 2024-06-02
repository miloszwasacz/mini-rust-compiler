use mini_rust_compiler_components::parser::Parser;

fn main() {
    let parser = Parser::new("tests/test.mrs").unwrap();
    let crt = parser.parse();
    match crt {
        Ok(crt) => {
            println!("{}", crt);
        }
        Err(err) => {
            println!("Error while parsing the file: {}", err);
        }
    }
}
