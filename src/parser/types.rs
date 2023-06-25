use crate::lexer::Opcode;

#[derive(Debug, PartialEq)]
pub struct ASTFile {
    pub fns: Vec<ASTFn>
}

#[derive(Debug, PartialEq)]
pub struct ASTFnDecl {
    pub name: ASTIdentifier,
    pub args: Vec<ASTIdentifier>
}

#[derive(Debug, PartialEq)]
pub struct  ASTFn {
    pub decl: ASTFnDecl,
    pub body: ASTExpr
}

#[derive(Debug, PartialEq)]
pub struct ASTFnCall {
    pub name: ASTIdentifier,
    pub args: Vec<ASTExpr>
}

#[derive(Debug, PartialEq)]
pub struct ASTIdentifier {
    pub name: String
}

#[derive(Debug, PartialEq)]
pub enum ASTExpr {
    Literal(f64),
    Var(ASTIdentifier),
    Op(Opcode, Box<ASTExpr>, Box<ASTExpr>),
    FnCall(ASTFnCall)
}
