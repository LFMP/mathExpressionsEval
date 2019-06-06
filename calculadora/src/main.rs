use calculadora::lexer;
use calculadora::parser;
//use calculadora::to_string;
use std::io;


fn main() {
	let mut saida = &mut String::new();
	let mut input = String::new();
	io::stdin().read_line(&mut input).ok().expect("Couldn't read line");
	/////////////////////////////////////////////////////
	let buff = lexer(input);
	println!("{:?}",buff);
	///////////////////////////////////////////////////////
	let mut buff2 = parser(buff);
	println!("{:?}",buff2);
	///////////////////////////////////////////////////////
	buff2.to_string(&mut saida);
	println!("{:?}", saida);
	saida.clear();
	////////////////////////////////////////////////////////
	buff2 = buff2.eval_step();
	println!("{:?}", buff2);
	///////////////////////////////////////////////////////
	buff2.to_string(&mut saida);
	println!("{:?}", saida);
}
