mod test;
mod types;

use regex_macro::regex;

pub use types::*;

pub struct Lexer<'a> {
    str: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn from_str(s: &'a str) -> Self {
        Lexer { str: s }
    }

    fn emit(&mut self) -> Option<Token<'a>> {
        self.str = self.str.trim_start();

        // trim comments
        if self.str.starts_with('#') {
            while let Some(cmm) = regex!(r"^#.*[\n$]").find(self.str) {
                self.str = &self.str[cmm.len()..]
            }
            self.str = self.str.trim_start();
        }

        if self.str.starts_with(';') {
            self.str = &self.str[1..];
            Some(Token::Semi)
        } else if self.str.starts_with('(') {
            self.str = &self.str[1..];
            Some(Token::OpenPar)
        } else if self.str.starts_with(')') {
            self.str = &self.str[1..];
            Some(Token::ClosePar)
        } else if self.str.starts_with(',') {
            self.str = &self.str[1..];
            Some(Token::Comma)
        } else if self.str.starts_with('+') {
            self.str = &self.str[1..];
            Some(Token::Operation(Opcode::Add))
        } else if self.str.starts_with('-') {
            self.str = &self.str[1..];
            Some(Token::Operation(Opcode::Sub))
        } else if self.str.starts_with('*') {
            self.str = &self.str[1..];
            Some(Token::Operation(Opcode::Mul))
        } else {
            let re_num = regex!(r"^[0-9]+\.?[0-9]*|\.[0-9]+");
            let re_ide = regex!(r"^[a-z_]+");

            if let Some(ide) = re_ide.find(self.str) {
                self.str = &self.str[ide.len()..];

                match ide.as_str() {
                    "fn" => Some(Token::Fn),
                    str => Some(Token::Identifier(str)),
                }
            } else if let Some(num) = re_num.find(self.str) {
                self.str = &self.str[num.len()..];

                Some(Token::Num(num.as_str().parse().unwrap()))
            } else {
                None
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (usize, Token<'a>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.emit() {
            Some(el) => Some((0, el, 0)),
            None => None
        }
    }
}
