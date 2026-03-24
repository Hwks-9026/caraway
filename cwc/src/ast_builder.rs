use crate::ast::*;
use crate::frontend_types::*;
use crate::parser::*;

use std::sync::Arc;
use pest::iterators::Pair;
use pest::pratt_parser::{Assoc::*, Op, PrattParser};
use std::sync::LazyLock;

static PRATT: LazyLock<PrattParser<Rule>> = LazyLock::new(|| {
    PrattParser::new()
        .op(Op::infix(Rule::comp_op, Left))
        .op(Op::infix(Rule::add_op, Left))
        .op(Op::infix(Rule::mul_op, Left))
        .op(Op::infix(Rule::power, Right)) 
});

pub struct AstBuilder {
    pub errors: Vec<ParseError>,
    pub state: Arc<CompilerState>
}

impl AstBuilder {
    pub fn new(state: Arc<CompilerState>) -> Self {
        Self {
            errors: Vec::new(),
            state
        }
    }

    // TODO: Add 'Colored' crate and color error messages properly
    fn push_error(&mut self, span: pest::Span, message: impl Into<String>) {
        self.errors.push(ParseError {
            span: span.into(),
            message: message.into(),
        });
    }

    pub fn build_program(&mut self, pair: Pair<Rule>) -> Program {
        let span = pair.as_span().into();
        let mut statements = Vec::new();

        for inner in pair.into_inner() {
            if inner.as_rule() == Rule::EOI {
                break;
            }
            if let Some(stmt) = self.build_statement(inner) {
                statements.push(stmt);
            }
        }

        Program { statements, span }
    }

    fn build_statement(&mut self, pair: Pair<Rule>) -> Option<Statement> {
        let span = pair.as_span();
        match pair.as_rule() {
            Rule::import_stmt => self.build_import(pair).map(Statement::Import),
            Rule::export_stmt => self.build_export(pair).map(Statement::Export),
            Rule::module_def => self.build_module(pair).map(Statement::ModuleDef),
            Rule::declare => self.build_declare(pair).map(Statement::Declare),
            Rule::update => self.build_update(pair).map(Statement::Update),
            Rule::exported_comment => self.build_exported_comment(pair).map(Statement::ExportedComment),
            Rule::expression => self.build_expression(pair).map(Statement::Expression),
            _ => {
                self.push_error(span, format!("Unexpected rule in statement: {:?}", pair.as_rule()));
                None
            }
        }
    }
    
    fn build_import(&mut self, pair: Pair<Rule>) -> Option<ImportStmt> {
        println!("Building import statement");
        let span = pair.as_span().into();
        let mut inner = pair.into_inner();
        
        // Target *should* be a macro_call or import_path
        let target = inner.next()?; 
        let path = match target.as_rule() {
            Rule::macro_call => {
                let name = target.into_inner().next()?.as_str().to_string();
                ImportPath::Macro(name)
            },
            Rule::import_path => {
                let mut segments = Vec::new();
                let mut has_asterix = false;
                for p in target.into_inner() {
                    if p.as_rule() == Rule::asterix {
                        has_asterix = true;
                    } else {
                        segments.push(p.as_str().to_string());
                    }
                }
                
                // Add to dependency tracker
                let dep_path = format!("{}.cara",segments.join("/"));
                if let Ok(mut tracker) = self.state.tracker.lock() {
                    let is_new_work = tracker.add_dependency(dep_path);
                    if is_new_work {
                        self.state.cvar.notify_all();
                    }
                }
                ImportPath::Path(segments, has_asterix)
            },
            _ => {
                self.push_error(target.as_span(), "Invalid import target");
                return None;
            }
        };

        let mut alias = None;
        if let Some(alias_node) = inner.next() {
            if alias_node.as_rule() == Rule::identifier {
                alias = Some(alias_node.as_str().to_string());
            }
        }

        Some(ImportStmt { path, alias, span })
    }

    fn build_export(&mut self, pair: Pair<Rule>) -> Option<ExportStmt> {
        let inner = pair.into_inner().next()?; // Get what follows "export"

        match inner.as_rule() {
            Rule::module_def => self.build_module(inner).map(ExportStmt::ModuleDef),
            Rule::declare => self.build_declare(inner).map(ExportStmt::Declare),
            Rule::identifier => Some(ExportStmt::Identifier(inner.as_str().to_string())),
            Rule::import_stmt => Some(ExportStmt::Use(self.build_import(inner).unwrap())),
            _ => {
                self.push_error(inner.as_span(), "Invalid export target");
                None
            }
        }
    }

    fn build_update(&mut self, pair: Pair<Rule>) -> Option<Update> {
        let span = pair.as_span().into();
        let mut inner = pair.into_inner();
        
        let lhs_pair = inner.next()?;
        let lhs = self.build_assignment_lhs(lhs_pair)?;
        
        // Skip the "->" arrow_op
        let _arrow = inner.next()?; 
        
        let expr_pair = inner.next()?;
        let expr = self.build_expression(expr_pair)?;

        Some(Update { lhs, expr, span })
    }

    fn build_exported_comment(&mut self, pair: Pair<Rule>) -> Option<StringLit> {
        let inner = pair.into_inner().next()?; // Get the string_lit
        let span = inner.as_span().into();
        let inner_str = inner.into_inner().next()?.as_str().to_string();
        
        Some(StringLit { value: inner_str, span })
    }


    fn build_module(&mut self, pair: Pair<Rule>) -> Option<ModuleDef> {
        let span = pair.as_span().into();
        let mut inner = pair.into_inner();
        
        let name = inner.next()?.as_str().to_string();
        let mut statements = Vec::new();

        for stmt_pair in inner {
            if let Some(stmt) = self.build_statement(stmt_pair) {
                statements.push(stmt);
            }
        }

        Some(ModuleDef { name, statements, span })
    }

    fn build_declare(&mut self, pair: Pair<Rule>) -> Option<Declare> {
        let span = pair.as_span().into();
        let mut inner = pair.into_inner();
        
        let lhs_pair = inner.next()?;
        let lhs = self.build_assignment_lhs(lhs_pair)?;
        
        // Skip assign_op
        let _op = inner.next()?; 
        
        let expr_pair = inner.next()?;
        let expr = self.build_expression(expr_pair)?;

        Some(Declare { lhs, expr, span })
    }

    pub fn build_expression(&mut self, pair: Pair<Rule>) -> Option<Expression> {
        let pairs = pair.into_inner();

        let expr = PRATT
            .map_primary(|primary| {
                // If the primary is actually a deeper nested rule (like term/factor/power),
                // we just extract the real primary token inside it.
                let mut current = primary;
                while current.as_rule() != Rule::number 
                   && current.as_rule() != Rule::string_lit
                   && current.as_rule() != Rule::path_identifier
                   && current.as_rule() != Rule::func_call
                   && current.as_rule() != Rule::block
                   && current.as_rule() != Rule::abs_val
                   && current.as_rule() != Rule::list
                   && current.as_rule() != Rule::point
                   && current.as_rule() != Rule::expression // Parentheses are for cowards
                {
                    // TODO: There's probably a way to avoid cloning here, but my brain isn't
                    // functional enough to figure it out.
                    if let Some(inner) = current.clone().into_inner().next() {
                        current = inner;
                    } else {
                        break;
                    }
                }
                
                if current.as_rule() == Rule::expression {
                    // Handle parenthesized expressions
                    self.build_expression(current)
                        .unwrap_or_else(|| Expression::Primary(Primary::Number(0.0, Span { start: 0, end: 0 }))) // Fallback on error
                } else if let Some(prim) = self.build_primary(current) {
                    Expression::Primary(prim)
                } else {
                    // Fallback to prevent panic if parsing fails, error is already logged in build_primary
                    Expression::Primary(Primary::Number(0.0, Span { start: 0, end: 0 }))
                }
            })
            .map_infix(|lhs, op, rhs| {
                let span = Span {
                    start: get_expr_span(&lhs).start,
                    end: get_expr_span(&rhs).end,
                };
                Expression::Infix {
                    lhs: Box::new(lhs),
                    op: op.as_str().to_string(),
                    rhs: Box::new(rhs),
                    span,
                }
            })
            .parse(pairs);

        Some(expr)
    }

    fn build_primary(&mut self, pair: Pair<Rule>) -> Option<Primary> {
        let span = pair.as_span();
        match pair.as_rule() {
            Rule::number => {
                // Same as previous
                match pair.as_str().parse::<f64>() {
                    Ok(val) => Some(Primary::Number(val, span.into())),
                    Err(_) => {
                        self.push_error(span, "Invalid number format");
                        None
                    }
                }
            },
            Rule::string_lit => {
                let inner_str = pair.into_inner().next().map(|p| p.as_str().to_string()).unwrap_or_default();
                Some(Primary::StringLit(StringLit { value: inner_str, span: span.into() }))
            },
            Rule::path_identifier => {
                let segments = pair.as_str().split("::").map(|s| s.to_string()).collect();
                Some(Primary::PathIdentifier(segments, span.into()))
            },
            Rule::func_call => {
                let mut inner = pair.into_inner();
                // Target is either path_identifier or macro_call (which we are skipping for now)
                // TODO: make sure macros don't mess this up
                let target_pair = inner.next()?; 
                let target = Box::new(self.build_primary(target_pair)?);
                
                let mut args = Vec::new();
                for expr_pair in inner {
                    if let Some(expr) = self.build_expression(expr_pair) {
                        args.push(expr);
                    }
                }
                Some(Primary::FuncCall { target, args, span: span.into() })
            },
            Rule::block => {
                let mut statements = Vec::new();
                for stmt_pair in pair.into_inner() {
                    if let Some(stmt) = self.build_statement(stmt_pair) {
                        statements.push(stmt);
                    }
                }
                Some(Primary::Block(statements, span.into()))
            },
            Rule::abs_val => {
                let expr_pair = pair.into_inner().next()?;
                let expr = self.build_expression(expr_pair)?;
                Some(Primary::AbsVal(Box::new(expr), span.into()))
            },
            Rule::list => {
                let mut elements = Vec::new();
                for prim_pair in pair.into_inner() {
                    if let Some(prim) = self.build_primary(prim_pair) {
                        elements.push(prim);
                    }
                }
                Some(Primary::List(elements, span.into()))
            },
            Rule::point => {
                let mut inner = pair.into_inner();
                let x_pair = inner.next()?;
                let y_pair = inner.next()?;
                
                let x = Box::new(self.build_primary(x_pair)?);
                let y = Box::new(self.build_primary(y_pair)?);
                
                Some(Primary::Point(x, y, span.into()))
            },
            Rule::expression => {
                let expr = self.build_expression(pair)?;
                Some(Primary::Expression(Box::new(expr), span.into()))
            },
            _ => {
                self.push_error(span, format!("Unknown primary token: {:?}", pair.as_rule()));
                None
            }
        }
    }

    fn build_assignment_lhs(&mut self, pair: Pair<Rule>) -> Option<AssignmentLhs> {
        let inner = pair.into_inner().next()?;
        let span = inner.as_span().into();
        
        match inner.as_rule() {
            Rule::func_decl => {
                let mut it = inner.into_inner();
                let path_pair = it.next()?;
                
                let path = path_pair.as_str().split("::").map(|s| s.to_string()).collect();
                let args = it.map(|p| p.as_str().to_string()).collect();
                
                Some(AssignmentLhs::FuncDecl { name: path, args, span })
            },
            Rule::path_identifier => {
                let path = inner.as_str().split("::").map(|s| s.to_string()).collect();
                Some(AssignmentLhs::PathIdentifier { path, span })
            },
            _ => None
        }
    }
    
}


// Helper to get spans out of the new Expression enum
fn get_expr_span(expr: &Expression) -> Span {
    match expr {
        Expression::Primary(p) => get_primary_span(p),
        Expression::Infix { span, .. } => *span,
    }
}

// Do I look like I want to copy and paste this 10 times
fn get_primary_span(prim: &Primary) -> Span {
    match prim {
        Primary::Number(_, span) => *span,
        Primary::StringLit(lit) => lit.span,
        // This is my first time discovering that you can
        // use this '..' syntax for unused struct elements
        // (I've tried _ in the past and it errors)
        Primary::FuncCall { span, .. } => *span, 
        Primary::MacroCall { span, .. } => *span,
        Primary::PathIdentifier(_, span) => *span,
        Primary::Expression(_, span) => *span,
        Primary::Block(_, span) => *span,
        Primary::AbsVal(_, span) => *span,
        Primary::List(_, span) => *span,
        Primary::Point(_, _, span) => *span,
    }
}
