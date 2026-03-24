mod parser;
mod frontend_types;
mod ast;
mod ast_builder;
mod args;

use std::sync::{Arc, Mutex};


use args::parse_args;

const DEFAULT_CODE: &str = 
r#"f(x) = x^2
y = f(2*x)
b = {4^2}

g = [2, 3, 4]
f = (1.67,2)()
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
    
    let deps = Arc::new(Mutex::new(frontend_types::DependencyTracker::new()));

    let ast_result = parser::parse_file(&file, deps);
    match ast_result.1.len() {
        0 => {dbg!(ast_result.0);}
        _ => {
            for error in ast_result.1 {
                error.pretty_print(&file, "filename".to_string())
            }
        },
    }


}
