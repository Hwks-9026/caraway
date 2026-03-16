mod parser;
mod args;

use pest::Parser;

use args::parse_args;
use parser::{CaraParser, Rule};

const DEFAULT_CODE: &str = 
r#"f(x) = x^2
y = f(2*x)
b = {4^2}

g = 2
"#;
fn main() {
    let args = parse_args();

    let file = match args.input {
        Some(path) => {
            match std::fs::read_to_string(path) {
                Ok(str) => str,
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    DEFAULT_CODE.to_string()
                }
            }
        }
        _ => {DEFAULT_CODE.to_string()}
    };

    let parser = CaraParser::parse(Rule::program, &file).unwrap();
    dbg!(parser);


}
