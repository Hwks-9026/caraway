mod parser;
mod frontend_types;
mod ast;
mod ast_builder;
mod args;

use std::sync::{Arc, Condvar, Mutex};
use crate::ast::Program;
use crate::frontend_types::{FileState, ParseError};


use args::parse_args;

use crate::frontend_types::{CompilerState, DependencyTracker};

const DEFAULT_PATH: &str = "src/main.cara";
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
    println!("Compilation finished! Processed {} files.", final_tracker.files.len());
    
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
        // Print where the OS thinks we currently are
        println!("CWD: {:?}", std::env::current_dir().unwrap());
        println!("Trying to read: [{}]", current_file);

        let file_contents = match std::fs::read_to_string(&current_file) {
            Ok(contents) => contents,
            Err(e) => {
                // This will tell us if it's NotFound, PermissionDenied, etc.
                eprintln!("CRITICAL FS ERROR reading [{}]: {:?}", current_file, e);
                String::new() // Fallback to empty string so the thread doesn't panic
            }
        };
        let (program_opt, errors) = parser::parse_file(&file_contents, Arc::clone(&state));

        let mut tracker = state.tracker.lock().unwrap();
        tracker.active_workers -= 1;

        if errors.len() > 0 {
            tracker.files.insert(current_file, FileState::Done(program_opt.unwrap()));
        } else {
            tracker.files.insert(current_file, FileState::Failed(errors));
        }

        state.cvar.notify_all();
    }
}
