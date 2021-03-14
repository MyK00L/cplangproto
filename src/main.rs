mod ast;
//mod typing;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

fn main() {
	let txt = std::fs::read_to_string("./input.cp").unwrap();
	let a = grammar::ExprParser::new().parse(&txt).unwrap();
	println!("{:?}",a);
}
