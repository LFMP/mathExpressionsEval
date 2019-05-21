use calculadora::lexer;
use calculadora::parser;
// use calculadora::eval_step;
// use calculadora::to_string;
use std::io;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Couldn't read line");
    let _buff = lexer(&input);
    println!("{:?}",_buff);
    let _buff2 = parser(_buff);
    println!("{:?}",_buff2);
}
