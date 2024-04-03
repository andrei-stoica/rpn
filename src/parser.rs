use crate::tokenizer::{Op, Token};

#[derive(Debug, PartialEq)]
enum Expr {
    Calc(Op, Box<Expr>, Box<Expr>),
    Literal(Token),
}

#[derive(Debug, PartialEq)]
enum ParserError {
    UnrecognizedToken,
    OperatorMissingOpperand(Op),
    UnbalancedEquation,
    NoExpression,
}

fn parse(tokens: Vec<Token>) -> Result<Expr, ParserError> {
    let mut stack: Vec<Expr> = vec![];
    for token in tokens.into_iter() {
        match token {
            Token::Unrecognized => return Err(ParserError::UnrecognizedToken),
            Token::Operation(op) => {
                let operand2 = stack
                    .pop()
                    .ok_or(ParserError::OperatorMissingOpperand(op.clone()))?;
                let operand1 = stack
                    .pop()
                    .ok_or(ParserError::OperatorMissingOpperand(op.clone()))?;

                stack.push(Expr::Calc(op, Box::new(operand1), Box::new(operand2)));
            }
            token => stack.push(Expr::Literal(token)),
        }
    }
    match stack.len() {
        1 => Ok(stack.pop().unwrap()),
        0 => Err(ParserError::NoExpression),
        _ => Err(ParserError::UnbalancedEquation),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse(vec![]), Err(ParserError::NoExpression));
        assert_eq!(
            parse(vec![Token::Int(1)]).unwrap(),
            Expr::Literal(Token::Int(1))
        );
        // TODO This error does not make sense in this case, change later
        assert_eq!(
            parse(vec![Token::Int(1), Token::Int(2)]),
            Err(ParserError::UnbalancedEquation)
        );
        assert_eq!(
            parse(vec![Token::Int(1), Token::Operation(Op::Add)]),
            Err(ParserError::OperatorMissingOpperand(Op::Add))
        );
        assert_eq!(
            parse(vec![
                Token::Int(1),
                Token::Int(2),
                Token::Operation(Op::Add),
                Token::Operation(Op::Sub),
            ]),
            Err(ParserError::OperatorMissingOpperand(Op::Sub))
        );
        assert_eq!(
            parse(vec![
                Token::Int(1),
                Token::Int(2),
                Token::Operation(Op::Add),
                Token::Float(2.0),
            ]),
            Err(ParserError::UnbalancedEquation)
        );

        assert_eq!(
            parse(vec![
                Token::Int(1),
                Token::Int(2),
                Token::Operation(Op::Add)
            ])
            .unwrap(),
            Expr::Calc(
                Op::Add,
                Box::new(Expr::Literal(Token::Int(1))),
                Box::new(Expr::Literal(Token::Int(2)))
            )
        );

        assert_eq!(
            parse(vec![
                Token::Int(1),
                Token::Float(2.1),
                Token::Operation(Op::Add)
            ])
            .unwrap(),
            Expr::Calc(
                Op::Add,
                Box::new(Expr::Literal(Token::Int(1))),
                Box::new(Expr::Literal(Token::Float(2.1)))
            )
        );
        assert_eq!(
            parse(vec![
                Token::Float(1.5),
                Token::Float(2.5),
                Token::Operation(Op::Add)
            ])
            .unwrap(),
            Expr::Calc(
                Op::Add,
                Box::new(Expr::Literal(Token::Float(1.5))),
                Box::new(Expr::Literal(Token::Float(2.5)))
            )
        );

        assert_eq!(
            parse(vec![
                Token::Int(3),
                Token::Int(1),
                Token::Int(2),
                Token::Operation(Op::Add),
                Token::Operation(Op::Sub)
            ])
            .unwrap(),
            Expr::Calc(
                Op::Sub,
                Box::new(Expr::Literal(Token::Int(3))),
                Box::new(Expr::Calc(
                    Op::Add,
                    Box::new(Expr::Literal(Token::Int(1))),
                    Box::new(Expr::Literal(Token::Int(2)))
                ))
            )
        );

        assert_eq!(
            parse(vec![
                Token::Int(3),
                Token::Int(1),
                Token::Operation(Op::Mult),
                Token::Int(4),
                Token::Int(2),
                Token::Operation(Op::Div),
                Token::Operation(Op::Add)
            ])
            .unwrap(),
            Expr::Calc(
                Op::Add,
                Box::new(Expr::Calc(
                    Op::Mult,
                    Box::new(Expr::Literal(Token::Int(3))),
                    Box::new(Expr::Literal(Token::Int(1)))
                )),
                Box::new(Expr::Calc(
                    Op::Div,
                    Box::new(Expr::Literal(Token::Int(4))),
                    Box::new(Expr::Literal(Token::Int(2)))
                ))
            )
        );
    }
}
