use std::fmt::Display;

use crate::parser::Expr;
use crate::tokenizer::{Op, Token};

#[derive(Debug, PartialEq)]
pub enum EvalError {
    UnexpectedLiteral(Token),
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Float(f32),
    Int(i32),
}

impl Number {
    pub fn add(&self, other: &Number) -> Number {
        match self {
            Number::Int(x) => match other {
                Number::Int(y) => Number::Int(x + y),
                Number::Float(y) => Number::Float(*x as f32 + y),
            },
            Number::Float(x) => match other {
                Number::Int(y) => Number::Float(x + *y as f32),
                Number::Float(y) => Number::Float(x + y),
            },
        }
    }

    pub fn sub(&self, other: &Number) -> Number {
        match self {
            Number::Int(x) => match other {
                Number::Int(y) => Number::Int(x - y),
                Number::Float(y) => Number::Float(*x as f32 - y),
            },
            Number::Float(x) => match other {
                Number::Int(y) => Number::Float(x - *y as f32),
                Number::Float(y) => Number::Float(x - y),
            },
        }
    }

    pub fn div(&self, other: &Number) -> Number {
        match self {
            Number::Int(x) => match other {
                Number::Int(y) => Number::Int(x / y),
                Number::Float(y) => Number::Float(*x as f32 / y),
            },
            Number::Float(x) => match other {
                Number::Int(y) => Number::Float(x / *y as f32),
                Number::Float(y) => Number::Float(x / y),
            },
        }
    }

    pub fn mult(&self, other: &Number) -> Number {
        match self {
            Number::Int(x) => match other {
                Number::Int(y) => Number::Int(x * y),
                Number::Float(y) => Number::Float(*x as f32 * y),
            },
            Number::Float(x) => match other {
                Number::Int(y) => Number::Float(x * *y as f32),
                Number::Float(y) => Number::Float(x * y),
            },
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Float(n) => write!(f, "{}", n),
            Number::Int(n) => write!(f, "{}", n),
        }
    }
}

pub fn eval(expr: &Expr) -> Result<Number, EvalError> {
    match expr {
        Expr::Literal(Token::Int(n)) => Ok(Number::Int(*n)),
        Expr::Literal(Token::Float(n)) => Ok(Number::Float(*n)),
        Expr::Calc(op, expr1, expr2) => {
            let val1 = eval(expr1)?;
            let val2 = eval(expr2)?;

            match op {
                Op::Add => Ok(val1.add(&val2)),
                Op::Sub => Ok(val1.sub(&val2)),
                Op::Div => Ok(val1.div(&val2)),
                Op::Mult => Ok(val1.mult(&val2)),
            }
        }
        Expr::Literal(token) => Err(EvalError::UnexpectedLiteral(*token)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(
            eval(&Expr::Literal(Token::Operation(Op::Add))),
            Err(EvalError::UnexpectedLiteral(Token::Operation(Op::Add)))
        );
        assert_eq!(
            eval(&Expr::Literal(Token::Operation(Op::Sub))),
            Err(EvalError::UnexpectedLiteral(Token::Operation(Op::Sub)))
        );
        assert_eq!(
            eval(&Expr::Literal(Token::Operation(Op::Div))),
            Err(EvalError::UnexpectedLiteral(Token::Operation(Op::Div)))
        );
        assert_eq!(
            eval(&Expr::Literal(Token::Operation(Op::Mult))),
            Err(EvalError::UnexpectedLiteral(Token::Operation(Op::Mult)))
        );
        assert_eq!(
            eval(&Expr::Literal(Token::Unrecognized)),
            Err(EvalError::UnexpectedLiteral(Token::Unrecognized)),
        );

        assert_eq!(eval(&Expr::Literal(Token::Int(1))), Ok(Number::Int(1)));
        assert_eq!(
            eval(&Expr::Literal(Token::Float(1.2))),
            Ok(Number::Float(1.2))
        );

        assert_eq!(
            eval(&Expr::Calc(
                Op::Add,
                Box::new(Expr::Literal(Token::Int(1))),
                Box::new(Expr::Literal(Token::Int(1))),
            )),
            Ok(Number::Int(2))
        );
        assert_eq!(
            eval(&Expr::Calc(
                Op::Add,
                Box::new(Expr::Literal(Token::Int(-1))),
                Box::new(Expr::Literal(Token::Int(-11))),
            )),
            Ok(Number::Int(-12))
        );
        assert_eq!(
            eval(&Expr::Calc(
                Op::Add,
                Box::new(Expr::Literal(Token::Int(-1))),
                Box::new(Expr::Literal(Token::Int(11))),
            )),
            Ok(Number::Int(10))
        );
        assert_eq!(
            eval(&Expr::Calc(
                Op::Sub,
                Box::new(Expr::Literal(Token::Int(1))),
                Box::new(Expr::Literal(Token::Int(1))),
            )),
            Ok(Number::Int(0))
        );
        assert_eq!(
            eval(&Expr::Calc(
                Op::Div,
                Box::new(Expr::Literal(Token::Int(2))),
                Box::new(Expr::Literal(Token::Int(1))),
            )),
            Ok(Number::Int(2))
        );
        assert_eq!(
            eval(&Expr::Calc(
                Op::Mult,
                Box::new(Expr::Literal(Token::Int(2))),
                Box::new(Expr::Literal(Token::Int(3))),
            )),
            Ok(Number::Int(6))
        );

        assert_eq!(
            eval(&Expr::Calc(
                Op::Add,
                Box::new(Expr::Literal(Token::Float(1.4))),
                Box::new(Expr::Literal(Token::Int(1))),
            )),
            Ok(Number::Float(2.4))
        );
        assert_eq!(
            eval(&Expr::Calc(
                Op::Add,
                Box::new(Expr::Literal(Token::Int(1))),
                Box::new(Expr::Literal(Token::Float(1.4))),
            )),
            Ok(Number::Float(2.4))
        );
        assert_eq!(
            eval(&Expr::Calc(
                Op::Add,
                Box::new(Expr::Literal(Token::Float(1.4))),
                Box::new(Expr::Literal(Token::Float(1.4))),
            )),
            Ok(Number::Float(2.8))
        );

        assert_eq!(
            eval(&Expr::Calc(
                Op::Add,
                Box::new(Expr::Literal(Token::Float(1.4))),
                Box::new(Expr::Calc(
                    Op::Mult,
                    Box::new(Expr::Literal(Token::Float(1.4))),
                    Box::new(Expr::Literal(Token::Int(2)))
                )),
            )),
            Ok(Number::Float(4.2))
        );
        assert_eq!(
            eval(&Expr::Calc(
                Op::Add,
                Box::new(Expr::Calc(
                    Op::Mult,
                    Box::new(Expr::Literal(Token::Float(1.4))),
                    Box::new(Expr::Literal(Token::Int(1)))
                )),
                Box::new(Expr::Literal(Token::Float(1.4))),
            )),
            Ok(Number::Float(2.8))
        );
    }
}
