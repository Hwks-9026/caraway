use pest_derive::Parser;
use pest::Parser;
use std::sync::{Arc, Mutex};

use crate::ast::Program;
use crate::ast_builder::AstBuilder;
use crate::frontend_types::*;
use crate::macros::MacroRegistry;

#[derive(Parser)]
#[grammar = "caraway.pest"]
pub struct CaraParser;

pub fn parse_file(input: &str, deps: Arc<Mutex<DependencyTracker>>, macros: MacroRegistry) -> (Option<Program>, Vec<ParseError>) {
    let parse_result = CaraParser::parse(Rule::program, input);  // Run Pest parser
    
    match parse_result {
        Ok(mut pairs) => { // No structural errors in the code: OK to try building AST
            let root = pairs.next().unwrap();
            let mut builder = AstBuilder::new(deps, macros);
            
            let program = builder.build_program(root); // Build AST and collect all errors
            (Some(program), builder.errors)
        },
        Err(pest_err) => { // Output structural error (for some reason pest only outputs one)
            
            let span = match pest_err.location {
                pest::error::InputLocation::Pos(p) => Span { start: p, end: p },
                pest::error::InputLocation::Span((start, end)) => Span { start, end },
            };
            
            let fatal_err = ParseError {
                span,
                // TODO: Add 'Colored' crate and color error messages properly
                message: format!("Fatal syntax error: {}", pest_err.variant.message()),
                                                                    
            };
            
            (None, vec![fatal_err])
        }
    }
}
