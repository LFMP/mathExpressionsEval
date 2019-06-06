#![feature(box_patterns)]
#![feature(box_syntax)]

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operators {
  Soma,
  Sub,
  Mul,
  Div,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tree {
  Num(i64),
  Operacao(Operators, Box<Tree>, Box<Tree>)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
  Num(i64),
  Abre,
  Fecha,
  Operador(Operators)
}

impl Token {
  fn precedencia(&self) -> usize {
    match self {
      Token::Operador(Operators::Div) | Token::Operador(Operators::Mul) => 2,
      Token::Operador(Operators::Sub) | Token::Operador(Operators::Soma) => 1,
      _ => 0,
    }
  }
}

fn to_tree(mut _rpn : Vec<Token>) -> Tree {
  
  let mut saida: Vec<Tree> = Vec::new();

  for tokens in _rpn {
    match tokens {
      Token::Num(x)=>{
        saida.push(Tree::Num(x));
      },
      Token::Operador(x)=>{
        let _right = Box::new(saida.pop().unwrap());
        let _left = Box::new(saida.pop().unwrap());
        saida.push(Tree::Operacao(x,_left,_right));
      },
      _ => {
        break;
      }
    }
  }
  saida.pop().unwrap()
}

pub fn lexer(mut _entrada : String ) -> Vec<Token> {
  let mut tokens: Vec<Token> = Vec::new();
  let mut _is_number = false;
  let mut aux = _entrada.trim_start().chars().peekable();
  while let Some(it) = aux.peek() {
    match it {
      '*' => {
          tokens.push(Token::Operador(Operators::Mul));
          aux.next();
        },
      '/' => {
          tokens.push(Token::Operador(Operators::Div));
          aux.next();
        },
      '+' => {
          tokens.push(Token::Operador(Operators::Soma));
          aux.next();
        },
      '-' => {
          tokens.push(Token::Operador(Operators::Sub));
          aux.next();
        },
      '(' => {
          tokens.push(Token::Abre);
          aux.next();
        },
      ')' => {
          tokens.push(Token::Fecha);
          aux.next();
        },
      ' ' => {
          aux.next();
        },
      '0'..='9' => {
        _is_number = true;
        let mut number = String::new();
        while _is_number {
          if let Some(ch) = aux.peek(){
            match ch {
              '0'..='9' => {
                number.push(*ch)
              },
              _ => {
                _is_number = false;
              }
            }
            if _is_number {
              aux.next();
            }
          }else{
            break;
          }
        }
        tokens.push(Token::Num(number.parse().unwrap()));
      },
      '\n'=> {
        break;
      },
      _ => {
        panic!("Caracter não permitido");
      }
    }
  }
  tokens
}

pub fn parser(_tokens : Vec<Token> ) -> Tree {
  let mut fila: Vec<Token> = vec![];
  let mut pilha: Vec<Token> = vec![];

  for token in _tokens {
    match token {
      Token::Num(_) => {
        fila.push(token);
      },
      Token::Operador(_) => {
        while let Some(op) = pilha.pop() {
          if token.precedencia() <= op.precedencia() {
            fila.push(op);
          }else {
            pilha.push(op);
            break;
          }
        }
        pilha.push(token);
      },
      Token::Abre => {
        pilha.push(token);
      },
      Token::Fecha => {
        let mut _is_close = false;
        while let Some(op) = pilha.pop() {
          match op {
            Token::Fecha => {
              _is_close = true;
              break;
            },
            Token::Abre => {
              continue;
            }
            _ => fila.push(op)
          }
        }
      }
    }
  }
  while let Some(token) = pilha.pop() {
    fila.push(token);
  }
  let saida = to_tree(fila);
  saida
}

impl Tree {
  pub fn eval_step(self) -> Tree{
    match self{
      Tree::Num(_) => self,
      Tree::Operacao(op, box Tree::Num(a), box Tree::Num(b)) => {
        match &op {
          Operators::Div => {
            Tree::Num(a/b)
          },
          Operators::Mul => {
            Tree::Num(a*b)
          },
          Operators::Sub => {
            Tree::Num(a-b)
          },
          Operators::Soma => {
            Tree::Num(a+b)
          }
        }
      },
      Tree::Operacao(op, box l, box r) => {
        match (&l,&r) {
          (Tree::Operacao(_,_,_),_) => {
            Tree::Operacao(op,Box::new(l.eval_step()),Box::new(r))
          },
          (_,Tree::Operacao(_,_,_)) => {
            Tree::Operacao(op,Box::new(l),Box::new(r.eval_step()))
          },
          _=> panic!("aaaaaa")
        }
      }
    }
  }
}

pub fn to_string(_expressao : Vec<Tree>) -> String {
  let converted:String = String::new();
  converted
}

#[cfg(test)]
mod test{
  use super::*;

  #[test]
  fn lexer_assert() {
    let mut entrada = String::from("4 / 2 + 7");
    let mut saida = vec![
      Token::Num(4),Token::Operador(Operators::Div),Token::Num(2),
      Token::Operador(Operators::Soma),Token::Num(7)
      ];
    
    assert_eq!(lexer(entrada),saida);

    entrada = String::from("(10 / 3 + 23) * (1 - 4)");
    saida = vec![
      Token::Abre, Token::Num(10), Token::Operador(Operators::Div), Token::Num(3),
      Token::Operador(Operators::Soma), Token::Num(23), Token::Fecha,
      Token::Operador(Operators::Mul), Token::Abre, Token::Num(1),
      Token::Operador(Operators::Sub), Token::Num(4), Token::Fecha
      ];
    
    assert_eq!(lexer(entrada),saida);
  }
  #[test]
  fn parser_assert(){
    let mut entrada = vec![
      Token::Num(4), Token::Operador(Operators::Div), Token::Num(2),
      Token::Operador(Operators::Soma), Token::Num(7)
    ];
    let mut saida = Tree::Operacao(
      Operators::Soma,
      box Tree::Operacao(Operators::Div, box Tree::Num(4), box Tree::Num(2)),
      box Tree::Num(7)
    );

    assert_eq!(parser(entrada), saida);

    entrada = vec![
      Token::Abre, Token::Num(10), Token::Operador(Operators::Div),
      Token::Num(3), Token::Operador(Operators::Soma), Token::Num(23),
      Token::Fecha, Token::Operador(Operators::Mul), Token::Abre, Token::Num(1),
      Token::Operador(Operators::Sub), Token::Num(4), Token::Fecha
    ];
    saida = Tree::Operacao(
      Operators::Mul,
      box Tree::Operacao(
        Operators::Soma,
        box Tree::Operacao(Operators::Div, box Tree::Num(10), box Tree::Num(3)),
        box Tree::Num(23)
      ),
      box Tree::Operacao(
        Operators::Sub,
        box Tree::Num(1),
        box Tree::Num(4)
      )
    );

    assert_eq!(parser(entrada), saida)
  }
}

