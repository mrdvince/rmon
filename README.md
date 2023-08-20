# rmon

An interpreter in Rust 
> Following Writing an Interpreter in Go Book by Thorsten Bal

## Prerequisite

Install Rust https://www.rust-lang.org/tools/install

## Getting started

Clone the repo and cd into the repo

```bash
  git clone https://github.com/mrdvince/rmon.git
  cd rmon
```

## Run
```bash
cargo -q run

Hello vince! This is the..I have no idea what this is tbh!!
Feel to start typing
>>let add = fn(x, y) { x + y; };
Token { type: "LET", literal: "let" }
Token { type: "IDENT", literal: "add" }
Token { type: "=", literal: "=" }
Token { type: "FUNCTION", literal: "fn" }
Token { type: "(", literal: "(" }
Token { type: "IDENT", literal: "x" }
Token { type: ",", literal: "," }
Token { type: "IDENT", literal: "y" }
Token { type: ")", literal: ")" }
Token { type: "{", literal: "{" }
Token { type: "IDENT", literal: "x" }
Token { type: "+", literal: "+" }
Token { type: "IDENT", literal: "y" }
Token { type: ";", literal: ";" }
Token { type: "}", literal: "}" }
Token { type: ";", literal: ";" }
```
...
