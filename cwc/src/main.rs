mod parser;
mod frontend_types;
mod ast;
mod ast_builder;
mod args;
mod macros; // macro is a keyword i guess
mod macro_implementations;

use std::sync::{Arc, Mutex};


use args::parse_args;
use macro_implementations::HexColorMacro;

use crate::macros::MacroRegistry;

const DEFAULT_CODE: &str = 
r#"q = 3
f = @hex{FFFFFF}
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
    let macros = MacroRegistry::new().with_macro(HexColorMacro); // TODO: add macros

    let ast_result = parser::parse_file(&file, deps, macros);
    match ast_result.1.len() {
        0 => {dbg!(ast_result.0);}
        _ => {
            for error in ast_result.1 {
                error.pretty_print(&file, "filename".to_string())
            }
        },
    }


}
