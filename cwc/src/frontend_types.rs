// I couldn't think of a better name for this file. It just defines a bunch of useful types for
// building the AST that aren't exactly part of it, so I didn't want to put it in either ast.rs or
// ast_builder.rs


// TODO: Remove before first release, and make sure no warnings persist
#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileState {
    Pending,
    Processing,
    Done,
}

pub struct DependencyTracker {
    pub files: HashMap<String, FileState>,
}

impl DependencyTracker {
    pub fn new() -> Self {
        Self { files: HashMap::new() }
    }

    /// Registers a newly discovered dependency if it hasn't been seen yet
    pub fn add_dependency(&mut self, path: String) {
        self.files.entry(path).or_insert(FileState::Pending);
    }
}
