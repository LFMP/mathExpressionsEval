#[derive(Debug, PartialEq)]
pub enum Operators {
  Soma,
  Sub,
  Mul,
  Div,
}

#[derive(Debug, PartialEq)]
pub enum Token {
  Num(String),
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

#[derive(Debug, PartialEq)]
pub enum Tree {
  Num,
  Operacao(Operators,Box<Tree>,Box<Tree>)
}

pub fn lexer(mut _entrada : &String ) -> Vec<Token> {
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
      '0'...'9' => {
        _is_number = true;
        let mut number: Vec<char> = Vec::new();
        while _is_number {
          if let Some(ch) = aux.peek(){
            match ch {
              '0'...'9' => {
                number.push(*ch)
              },
              _ => _is_number = false
            }
            if _is_number {
              aux.next();
            }
          }
        }
        tokens.push(Token::Num(number.iter().cloned().collect()))
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

pub fn parser(_tokens : Vec<Token> ) -> Vec<Token> {
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
            _ => fila.push(op)
          }
        }
        assert!(_is_close);
      }
    }
  }
  while let Some(token) = pilha.pop() {
    fila.push(token);
  }
  fila
}

pub fn eval_step(_expressao : &[Vec<Tree>]) -> Vec<Tree> {
  let avaliado:Vec<Tree> = Vec::new();
  avaliado
}

pub fn to_string(_expressao : &[Vec<Tree>]) -> String {
  let converted:String = String::new();
  converted
}

#[cfg(test)]
mod tests {
  
  #[test]
  fn test_lexer(){
    //assert_eq!(vec!["31", "*", "(", "4", "+", "10", ")"], lexer("31  * (4 + 10)"));
  }
}