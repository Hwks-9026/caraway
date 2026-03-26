// TODO: Remove before first release, and make sure no warnings persist
#![allow(dead_code)]

use crate::frontend_types::Span;

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub span: Span,
}

// I find it really funny that this enum has to exist but it kind of does
#[derive(Debug, Clone)]
pub enum Statement {
    Import(ImportStmt),
    Export(ExportStmt),
    ModuleDef(ModuleDef),
    Declare(Declare),
    Update(Update),
    ExportedComment(StringLit),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub struct ImportStmt {
    pub path: ImportPath,
    pub alias: Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ImportPath {
    Macro(String),
    Path(Vec<String>, bool), // bool is true if it ends with a *.this is genuinely the best way I
                             // could think of to do that I don't feel like writing another fucking
                             // enum
}

#[derive(Debug, Clone)]
pub enum ExportStmt {
    Use(ImportStmt),
    ModuleDef(ModuleDef),
    Declare(Declare),
    Identifier(String),
    Span(Span),
}

#[derive(Debug, Clone)]
pub struct ModuleDef {
    pub name: String,
    pub statements: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum AssignmentLhs {
    FuncDecl { name: Vec<String>, args: Vec<String>, span: Span },
    PathIdentifier { path: Vec<String>, span: Span },
    MacroCall { name: String, content: String, span: Span },
}

#[derive(Debug, Clone)]
pub struct Declare {
    pub lhs: AssignmentLhs,
    pub expr: Expression,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Update {
    pub lhs: AssignmentLhs,
    pub expr: Expression,
    pub span: Span,
}

// Math trees
#[derive(Debug, Clone)]
pub enum Expression {
    Primary(Primary),
    Infix {
        lhs: Box<Expression>,
        op: String,
        rhs: Box<Expression>,
        span: Span,
    },
}

#[derive(Debug, Clone)]
pub enum Primary {
    Number(f64, Span),
    StringLit(StringLit),
    FuncCall { target: Box<Primary>, args: Vec<Expression>, span: Span },
    MacroCall { name: String, content: String, span: Span },
    PathIdentifier(Vec<String>, Span),
    Expression(Box<Expression>, Span),
    Block(Vec<Statement>, Span),
    AbsVal(Box<Expression>, Span),
    List(Vec<Primary>, Span),
    Point(Box<Primary>, Box<Primary>, Span),
    Update(Box<Update>, Span)
}

#[derive(Debug, Clone)]
pub struct StringLit {
    pub value: String,
    pub span: Span,
}
