use pest_derive::Parser;
use pest::Parser;
use std::sync::{Arc, Mutex};

use crate::ast::Program;
use crate::ast_builder::AstBuilder;
use crate::frontend_types::*;

#[derive(Parser)]
#[grammar = "caraway.pest"]
pub struct CaraParser;

pub fn parse_file(input: &str, state: Arc<CompilerState>) -> (Option<Program>, Vec<ParseError>) {
    let parse_result = CaraParser::parse(Rule::program, input);  
    match parse_result {
        Ok(mut pairs) => { 
            let root = pairs.next().unwrap();
            
            // Pass the full state (Tracker + Condvar) to the builder
            let mut builder = AstBuilder::new(Arc::clone(&state)); 
            
            let program = builder.build_program(root); 
            (Some(program), builder.errors)
        },
        Err(pest_err) => { 
            let span = match pest_err.location {
                pest::error::InputLocation::Pos(p) => Span { start: p, end: p },
                pest::error::InputLocation::Span((start, end)) => Span { start, end },
            };
            
            let fatal_err = ParseError {
                span,
                message: format!("Fatal syntax error: {}", pest_err.variant.message()),
            };
            
            (None, vec![fatal_err])
        }
    }
}
