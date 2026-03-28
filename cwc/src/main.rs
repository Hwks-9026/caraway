mod parser;
mod frontend_types;
mod ast;
mod ast_builder;
mod args;
mod graph;
mod resolve;

use colored::Colorize;
use std::sync::{Arc, Condvar, Mutex};
use std::collections::HashMap;
use crate::frontend_types::{FileState, ParseError};

use args::parse_args;

use crate::frontend_types::{CompilerState, DependencyTracker};

const DEFAULT_PATH: &str = "main.cara";
fn main() {
    let args = parse_args();

    let initial_file = match args.input {
        Some(path) => format!("{:?}",path).trim_matches(|c| c == '"' || c == '\'').to_string(),
        _ => DEFAULT_PATH.to_string()
    };
    let mut initial_tracker = DependencyTracker::new();
    initial_tracker.add_dependency(initial_file);
    let shared_state = Arc::new(CompilerState {
        tracker: Mutex::new(initial_tracker),
        cvar: Condvar::new()
    });

    let num_threads = args.threads;
    let mut handles = vec![];

    for _ in 0..num_threads {
        let state_clone = Arc::clone(&shared_state);
        handles.push(std::thread::spawn(move || {
            worker_loop(state_clone);
        }))
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let final_tracker = shared_state.tracker.lock().unwrap();
    println!("{} Processed {} files.","[AST]".bold(), 
        format!("{}",final_tracker.files.len()).bold().yellow());
    if handle_results(&final_tracker.files) {return};
    if check_dependencies(&final_tracker.files) {return};
}
fn worker_loop(state: Arc<CompilerState>) {
    loop {
        let mut tracker = state.tracker.lock().unwrap();

        while tracker.queue.is_empty() && tracker.active_workers > 0 {
            tracker = state.cvar.wait(tracker).unwrap();
        }

        if tracker.queue.is_empty() && tracker.active_workers == 0 {
            break;
        }
        let current_file = tracker.queue.pop_front().unwrap();
        tracker.files.insert(current_file.clone(), FileState::Processing);
        tracker.active_workers += 1;
        drop(tracker);
        let mut failed = false;
        let file_contents = match std::fs::read_to_string(&current_file) {
            Ok(contents) => contents,
            Err(e) => {
                eprintln!("{} reading [{}]: {:?}","[File System Error]".red().bold(), current_file, e);
                failed = true;
                String::new() // Fallback to empty string so the thread doesn't panic
            }
        }; 
        let (program_opt, errors) = parser::parse_file(&file_contents, Arc::clone(&state));
        let mut tracker = state.tracker.lock().unwrap();
        tracker.active_workers -= 1;
        if errors.len() == 0 && !failed {
            tracker.files.insert(current_file, FileState::Done(program_opt.unwrap()));
        } else {
            tracker.files.insert(current_file, FileState::Failed(errors));
        }

        state.cvar.notify_all();
    }
}

fn handle_results(files: &HashMap<String, FileState>) -> bool {
    let mut had_error = false;

    for (filename, state) in files {
        if let FileState::Failed(errors) = state {
            had_error = true;

            // You’ll need a way to get the source string for this file
            let source = std::fs::read_to_string(filename)
                .unwrap_or_else(|_| "<could not read file>".to_string());

            for err in errors {
                err.pretty_print(&source, filename.clone());
            }
        }
    }

    if had_error {
        true
    } else {
        println!("{} {}", "[AST]".bold(),"No AST errors.".bright_green());
        false
    }
}

fn check_dependencies(files: &HashMap<String, FileState>) -> bool {
    let errors = resolve::resolve_dependencies(files);
    if errors.is_empty() {
        println!("{} {}", "[Resolve]".bold(), "No dependency errors.".bright_green());
        return false;
    }

    for err in &errors {
        if err.file.is_empty() {
            eprintln!("{} {}", "[Resolve Error]".red().bold(), err.message);
        } else {
            eprintln!("{} [{}]: {}", "[Resolve Error]".red().bold(), err.file, err.message);
        }
    }
    true
}