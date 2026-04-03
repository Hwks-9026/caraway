// I couldn't think of a better name for this file. It just defines a bunch of useful types for
// building the AST that aren't exactly part of it, so I didn't want to put it in either ast.rs or
// ast_builder.rs


// TODO: Remove before first release, and make sure no warnings persist
#![allow(dead_code)]
use colored::Colorize;
use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, Condvar};

use crate::ast::Program;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl<'a> From<pest::Span<'a>> for Span {
    fn from(span: pest::Span<'a>) -> Self {
        Span { start: span.start(), end: span.end() }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub span: Span,
    pub message: String,
}
impl ParseError {
    pub fn pretty_print(&self, source: &String, filename: String) {
        let mut line_number = 1;
        let mut line_start = 0;
        let mut current_line = "";

        for (i, line) in source.lines().enumerate() {
            // I'm not adding a windows check. This will be wrong on windows because
            // for some reason they use \r\n for newlines sometimes????
            let line_len = line.len() + 1; 
            
            if line_start + line_len > self.span.start {
                line_number = i + 1;
                current_line = line;
                break;
            }
            line_start += line_len;
        }

        let prefix = &source[line_start..self.span.start];
        let col_number = prefix.chars().count();
        
        let err_len = if self.span.end > self.span.start {
            let end_idx = self.span.end.min(line_start + current_line.len());
            let err_str = &source[self.span.start..end_idx];
            err_str.chars().count().max(1)
        } else {
            1
        };

        // I based this on the way rust errors are printed
        println!("{}: {}", "error".bright_red().bold(), self.message.bold());
        println!("  {} {}:{}:{}", "-->".blue().bold(), filename, line_number, col_number + 1);
        println!("   {}", "|".blue().bold());
        
        println!("{:<2} {} {}", line_number.to_string().blue().bold(), "|".blue().bold(), current_line);
        
        let padding = " ".repeat(col_number);
        let pointers = "^".repeat(err_len).bright_red().bold();
        println!("   {} {}{}\n", "|".blue().bold(), padding, pointers);
    }
}

#[derive(Debug, Clone)]
pub enum FileState {
    Pending,
    Processing,
    Done(Program),
    Failed(Vec<ParseError>),
}

pub struct DependencyTracker {
    pub files: HashMap<String, FileState>,
    pub queue: VecDeque<String>,
    pub active_workers: usize,
}

impl DependencyTracker {
    pub fn new() -> Self {
        Self { files: HashMap::new(), queue: VecDeque::new(), active_workers: 0}
    }

    /// Registers a newly discovered dependency if it hasn't been seen yet
    /// returns true if newly added 
    pub fn add_dependency(&mut self, path: String) -> bool {
        if !self.files.contains_key(&path) {
            println!("{} Added dependency {}", "[AST]".bold(),&path.bright_blue().bold());
            self.files.insert(path.clone(), FileState::Pending);
            self.queue.push_back(path);
            return true;
        }
        return false;
    }
}

pub struct CompilerState {
    pub tracker: Mutex<DependencyTracker>,
    pub cvar: Condvar,
}
