use std::io::{BufRead, Write};

use crate::lexer::Lexer;

const PROMPT: &str = ">>";
pub fn start<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut lines = input.lines();
    loop {
        write!(output, "{}", PROMPT).unwrap();
        output.flush().unwrap();

        if let Some(Ok(line)) = lines.next() {
            let mut lexer = Lexer::new(&line);
            loop {
                let token = lexer.next_token();
                if token.r#type == "EOF" {
                    break;
                }
                writeln!(output, "{:?}", token).unwrap()
            }
        } else {
            break;
        }
    }
}
