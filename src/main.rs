mod lexer;
mod parser;
mod grammar;

use std::io::{self, Write};
use crate::grammar::ASTFileParser;
use crate::parser::*;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue};
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

struct Ctx<'a> {
    context: &'a inkwell::context::Context,
    module: inkwell::module::Module<'a>,
    builder: inkwell::builder::Builder<'a>,
    ty: inkwell::types::FloatType<'a>
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let stdin       = io::stdin();
    let mut stdout  = io::stdout();
    let mut input   = String::new();

    let ctx = inkwell::context::Context::create();
    let ctx = Ctx {
        context: &ctx,
        module: ctx.create_module("repl"),
        builder: ctx.create_builder(),
        ty: ctx.f64_type()
    };

    loop {
        print!("> ");
        stdout.flush()?;
        input.clear();

        stdin.read_line(&mut input)?;

        if args.lexer { // Lexer mode
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
                    if args.parser { // Parser mode
                        println!("{:?}", ast.unwrap());
                        stdout.flush()?;
                    } else { // LLVM IR mode
                        for fn_raw in ast.unwrap().fns {
                            fn_raw.codegen(&ctx);
                        }

                        ctx.module.print_to_stderr();
                    }

                    break;
                }
            }
        }
    }
}

impl parser::ASTFn {
    fn codegen(&self, ctx: &Ctx) {
        let args: Vec<BasicMetadataTypeEnum> = self.decl.args.iter().map(|_| ctx.ty.into()).collect();
        let fn_type = ctx.ty.fn_type(&args, false);
        let fn_new = ctx.module.add_function(self.decl.name.name.as_str(), fn_type, None);
        let entry_bb = ctx.context.append_basic_block(fn_new, "entry");

        ctx.builder.position_at_end(entry_bb);

        ctx.builder.build_return(Some(&self.body.codegen(&ctx, &self.decl.args, &fn_new)));
    }
}

impl<'a> parser::ASTExpr {
    fn codegen(&'a self, ctx: &'a Ctx, args: &Vec<ASTIdentifier>, fun: &FunctionValue<'a>) -> BasicValueEnum {
        match self {
            Self::Literal(num)        => BasicValueEnum::FloatValue(ctx.ty.const_float(*num)),
            Self::Var(ident)          => {
                let mut param_num = 0;
                for (ii, arg) in args.into_iter().enumerate() {
                    if arg == ident { param_num = ii; break }
                };

                fun.get_nth_param(param_num as u32).unwrap()
            },
            Self::Op(op, num1, num2)  => match op {
                lexer::Opcode::Add => BasicValueEnum::FloatValue(ctx.builder.build_float_add(num1.codegen(&ctx, args, fun).into_float_value(), num2.codegen(&ctx, args, fun).into_float_value(), "tmp")),
                lexer::Opcode::Sub => BasicValueEnum::FloatValue(ctx.builder.build_float_sub(num1.codegen(&ctx, args, fun).into_float_value(), num2.codegen(&ctx, args, fun).into_float_value(), "tmp")),
                lexer::Opcode::Mul => BasicValueEnum::FloatValue(ctx.builder.build_float_mul(num1.codegen(&ctx, args, fun).into_float_value(), num2.codegen(&ctx, args, fun).into_float_value(), "tmp")),
            },
            Self::FnCall(fn_name)     => panic!("call")
        }
    }
}
