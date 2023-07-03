# kaleidoscope

Dirty-written mini-kaleidoscope LLVM flow at Rust

Run:
```
cargo run -- -l  # lexer only mode
cargo run -- -p  # parser only mode
cargo run        # full LLVM IR mode
```

Usage (REPL):
```
> exit  # close REPL
> print # print (full) LLVM module
> exec  # execute main function
> ...   # type any function to lex+pars+codegen
```

