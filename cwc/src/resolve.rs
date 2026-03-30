use std::collections::{HashMap, HashSet};

use crate::ast::*;
use crate::frontend_types::FileState;
use crate::graph::Graph;

pub struct ResolveError {
    pub file: String,
    pub message: String,
}

fn qualified_name(prefix: &str, name: &str) -> String {
    if prefix.is_empty() { name.to_string() } else { format!("{}::{}", prefix, name) }
}

// Tries progressively shorter prefixes until we get a match.
fn resolve_import_path(segments: &[String], known_files: &HashSet<String>) -> Option<(String, Vec<String>)> {
    for split in (1..=segments.len()).rev() {
        let mut search_path = std::path::PathBuf::new();
        for segment in &segments[..split] {
            search_path.push(segment); // first try a/b/c.cara, then a/b.cara for a function called c, then a.cara for b::c
        }
        search_path.set_extension("cara");
        let candidate = search_path.to_string_lossy().to_string();

        if known_files.contains(&candidate) {
            return Some((candidate, segments[split..].to_vec()));
        }
    }
    None
}

fn lhs_name(lhs: &AssignmentLhs) -> Option<String> {
    match lhs {
        AssignmentLhs::FuncDecl       { name, .. } => Some(name.join("::")),
        AssignmentLhs::PathIdentifier { path, .. } => Some(path.join("::")),
        AssignmentLhs::MacroCall      {       .. } => None,
    }
}

// Collects all declared symbol names
fn collect_declarations(statements: &[Statement], prefix: &str, out: &mut HashSet<String>) {
    for stmt in statements {
        match stmt {
            Statement::Declare(decl) => {
                if let Some(name) = lhs_name(&decl.lhs) {
                    out.insert(qualified_name(prefix, &name));
                }
            },
            Statement::ModuleDef(module) => {
                let qname = qualified_name(prefix, &module.name);
                out.insert(qname.clone());
                collect_declarations(&module.statements, &qname, out);
            },
            Statement::Export(ExportStmt::Declare(decl)) => {
                if let Some(name) = lhs_name(&decl.lhs) {
                    out.insert(qualified_name(prefix, &name));
                }
            },
            Statement::Export(ExportStmt::ModuleDef(module)) => {
                let qname = qualified_name(prefix, &module.name);
                out.insert(qname.clone());
                collect_declarations(&module.statements, &qname, out);
            },
            _ => {},
        }
    }
}

// Collects all import statements
fn collect_imports(
    statements: &[Statement],
    known_files: &HashSet<String>,
) -> Vec<(String, Vec<String>, bool)> {
    let mut imports = Vec::new();
    for stmt in statements {
        let import = match stmt {
            Statement::Import(i) => i,
            Statement::Export(ExportStmt::Use(i)) => i,
            _ => continue,
        };
        if let ImportPath::Path(segments, wildcard) = &import.path {
            if let Some((file_path, symbol_path)) = resolve_import_path(segments, known_files) {
                imports.push((file_path, symbol_path, *wildcard));
            }
        }
    }
    imports
}

pub fn resolve_dependencies(files: &HashMap<String, FileState>) -> Vec<ResolveError> {
    let mut errors = Vec::new();
    let known_files: HashSet<String> = files.keys().cloned().collect();

    // Collect every declaration in every file
    let mut declarations: HashMap<String, HashSet<String>> = HashMap::new();
    for (path, state) in files {
        if let FileState::Done(program) = state {
            let mut syms = HashSet::new();
            collect_declarations(&program.statements, "", &mut syms);
            declarations.insert(path.clone(), syms);
        }
    }

    let mut graph: Graph<String> = Graph::new();
    for path in declarations.keys() {
        graph.add_node(path.clone());
    }

    for (path, state) in files {
        if let FileState::Done(program) = state {
            for (source_file, symbol_path, wildcard) in collect_imports(&program.statements, &known_files) {
                graph.add_connection(path.clone(), source_file.clone());

                if wildcard || symbol_path.is_empty() {
                    continue;
                }
                let symbol = symbol_path.join("::");
                if let Some(target_decls) = declarations.get(&source_file) {
                    if !target_decls.contains(&symbol) {
                        errors.push(ResolveError {
                            file: path.clone(),
                            message: format!("Unresolved import: '{}' not found in '{}'", symbol, source_file),
                        });
                    }
                }
            }
        }
    }

    if graph.topological_order().is_none() {
        errors.push(ResolveError {
            file: String::new(),
            message: "Circular import detected between files".to_string(),
        });
    }

    errors
}
