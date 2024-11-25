extern crate core;

use crate::grammar::evaluate::evaluate;
use crate::grammar::lexer::get_tokens;
use crate::grammar::parser::get_ast;

mod grammar;

fn main() {
    let input = "2+3-1+456-1-3-2";
    let tokens = get_tokens(input).unwrap();
    let ast = get_ast(&tokens);
    println!("{:#?}", ast);
    let value = evaluate(&ast);
    println!("{}", value);
}

