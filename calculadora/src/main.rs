use calculadora::lexer;
use calculadora::parser;
//use calculadora::to_string;
use std::io;


fn main() {
	let mut saida = &mut String::new();
	let mut input = String::new();
	io::stdin().read_line(&mut input).ok().expect("Couldn't read line");

	let mut expressao = parser(lexer(input));

	expressao.to_string(&mut saida);
	println!("{}", saida);
	saida.clear();

	expressao = expressao.eval_step();
	
	expressao.to_string(&mut saida);
	println!("{}", saida);
	saida.clear();
}
