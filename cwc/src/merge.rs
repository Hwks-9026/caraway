use std::collections::HashMap;
use colored::Colorize;

use crate::frontend_types::*;
use crate::ast::*;
pub struct AstFlattener {
    flattened_statements: Vec<Statement>,
}

impl AstFlattener {
    pub fn new() -> Self {
        Self {
            flattened_statements: Vec::new(),
        }
    }

    pub fn flatten(
        &mut self,
        files: HashMap<String, FileState>,
    ) -> Program {
        for (file_path, file_state) in files {
            match file_state {
                FileState::Done(program) => {
                    let base_prefix = vec![file_path.replace(".cara", "")]; 
                    
                    let mut stmts = self.process_statements(program.statements, &base_prefix);
                    self.flattened_statements.append(&mut stmts);
                }
                FileState::Pending | FileState::Processing => {
                    panic!("{} {}", "[Flattener]".to_string().bold(), "Pending/Processing file was passed to flattener.".to_string().red())
                }
                FileState::Failed(errors) => panic!("{} {}", "[Flattener]".to_string().bold(), "Failed file was passed to flattener.".to_string().red())
            }
        }

        Program {
            statements: self.flattened_statements.clone(),
            span: Span::default(), 
        }
    }

    fn process_statements(
        &mut self,
        statements: Vec<Statement>,
        prefix: &[String],
    ) -> Vec<Statement> {
        let mut flat = Vec::new();

        for stmt in statements {
            match stmt {
                Statement::ModuleDef(mod_def) => {
                    let mut new_prefix = prefix.to_vec();
                    new_prefix.push(mod_def.name);
                    let mut inner_stmts = self.process_statements(mod_def.statements, &new_prefix);
                    flat.append(&mut inner_stmts);
                }
                
                Statement::Declare(mut decl) => {
                    decl.lhs = self.prefix_lhs(decl.lhs, prefix);
                    decl.expr = self.prefix_expr(decl.expr, prefix);
                    flat.push(Statement::Declare(decl));
                }

                Statement::Update(mut update) => {
                    update.lhs = self.prefix_lhs(update.lhs, prefix);
                    update.expr = self.prefix_expr(update.expr, prefix);
                    flat.push(Statement::Update(update));
                }

                Statement::Expression(expr) => {
                    flat.push(Statement::Expression(self.prefix_expr(expr, prefix)));
                }

                Statement::ExportedComment(c) => flat.push(Statement::ExportedComment(c)),

                Statement::Import(_) | Statement::Export(_) => {
                }
            }
        }
        flat
    }

    fn prefix_lhs(&mut self, lhs: AssignmentLhs, prefix: &[String]) -> AssignmentLhs {
        match lhs {
            AssignmentLhs::FuncDecl { mut name, args, span } => {
                name.splice(0..0, prefix.to_vec());
                // Note: we do NOT prefix `args` because they are local variables.
                AssignmentLhs::FuncDecl { name, args, span }
            }
            AssignmentLhs::PathIdentifier { mut path, span } => {
                path.splice(0..0, prefix.to_vec());
                AssignmentLhs::PathIdentifier { path, span }
            }
            AssignmentLhs::MacroCall { name, content, span } => {
                // Decide if macros are scoped or global
                AssignmentLhs::MacroCall { name, content, span }
            }
        }
    }

    fn prefix_expr(&mut self, expr: Expression, prefix: &[String]) -> Expression {
        match expr {
            Expression::Primary(primary) => Expression::Primary(self.prefix_primary(primary, prefix)),
            Expression::Infix { lhs, op, rhs, span } => Expression::Infix {
                lhs: Box::new(self.prefix_expr(*lhs, prefix)),
                op,
                rhs: Box::new(self.prefix_expr(*rhs, prefix)),
                span,
            },
        }
    }

    fn prefix_primary(&mut self, primary: Primary, prefix: &[String]) -> Primary {
        match primary {
            Primary::PathIdentifier(mut path, span) => {
                path.splice(0..0, prefix.to_vec());
                Primary::PathIdentifier(path, span)
            }
            Primary::FuncCall { target, args, span } => {
                let prefixed_target = Box::new(self.prefix_primary(*target, prefix));
                let prefixed_args = args
                    .into_iter()
                    .map(|arg| self.prefix_expr(arg, prefix))
                    .collect();
                Primary::FuncCall {
                    target: prefixed_target,
                    args: prefixed_args,
                    span,
                }
            }
            Primary::Expression(expr, span) => {
                Primary::Expression(Box::new(self.prefix_expr(*expr, prefix)), span)
            }
            Primary::Block(stmts, span) => {
                Primary::Block(self.process_statements(stmts, prefix), span)
            }
            Primary::AbsVal(expr, span) => {
                Primary::AbsVal(Box::new(self.prefix_expr(*expr, prefix)), span)
            }
            Primary::List(items, span) => {
                let prefixed_items = items
                    .into_iter()
                    .map(|item| self.prefix_primary(item, prefix))
                    .collect();
                Primary::List(prefixed_items, span)
            }
            Primary::Point(x, y, span) => Primary::Point(
                Box::new(self.prefix_primary(*x, prefix)),
                Box::new(self.prefix_primary(*y, prefix)),
                span,
            ),
            Primary::Update(update, span) => {
                let mut new_update = *update;
                new_update.lhs = self.prefix_lhs(new_update.lhs, prefix);
                new_update.expr = self.prefix_expr(new_update.expr, prefix);
                Primary::Update(Box::new(new_update), span)
            }
            Primary::Number(_, _) | Primary::StringLit(_) | Primary::DesmosLiteral(_) => primary,
            Primary::MacroCall { .. } => primary,
        }
    }
}
