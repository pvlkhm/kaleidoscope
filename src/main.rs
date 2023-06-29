mod lexer;
mod parser;
mod grammar;

use std::io::{self, Write};
use crate::grammar::ASTFileParser;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Lexer mode
    #[arg(short, long)]
    lexer: bool,
    /// Parser mode
    #[arg(short, long)]
    parser: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let stdin       = io::stdin();
    let mut stdout  = io::stdout();
    let mut input   = String::new();

    loop {
        print!("> ");
        stdout.flush()?;
        input.clear();

        stdin.read_line(&mut input)?;

        if args.lexer {
            let lx = lexer::Lexer::from_str(input.as_str());

            let mut v: Vec<lexer::Token> = vec![];

            for token in lx {
                v.push(token.1);
            }

            println!("{:?}", v);
            stdout.flush()?;
        } else {
            loop {
                let input_snapshot = input.clone();
                let ast = ASTFileParser::new().parse(lexer::Lexer::from_str(input_snapshot.as_str()));

                if ast.is_err() {
                    print!("< ");
                    stdout.flush()?;

                    stdin.read_line(&mut input)?;
                } else {
                    if args.parser {
                        println!("{:?}", ast.unwrap());
                        stdout.flush()?;
                    } else {
                        for fun in ast.unwrap().fns {
                            println!("{:?}", fun);
                        }
                    }

                    break;
                }
            }
        }
    }
}
