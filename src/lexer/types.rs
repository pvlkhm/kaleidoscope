#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    Fn,
    Semi,
    Num(f64),
    OpenPar,
    ClosePar,
    Comma,
    Identifier(&'a str),
    Operation(Opcode),
}

impl<'a> Token<'a> {
    pub fn as_f64(self) -> f64 {
        match self {
            Token::Num(n) => n,
            _ => panic!("panic")
        }
    }

    pub fn as_string(self) -> String {
        match self {
            Token::Identifier(str) => String::from(str),
            _ => panic!("panic")
        }
    }
}
