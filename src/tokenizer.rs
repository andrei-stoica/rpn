#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Int(i32),
    Float(f32),
    Operation(Op),
    Unrecognized,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mult,
    Div,
}

pub fn tokenize(text: String) -> Vec<Token> {
    let mut tokens = vec![];
    let mut stream = text.chars().into_iter().peekable();

    let mut buffer = vec![];
    let mut skip_to_whitespace = false;
    while let Some(ch) = stream.next() {
        if skip_to_whitespace {
            if ch.is_whitespace() {
                skip_to_whitespace = false;
            }
            continue;
        }
        let next = stream.peek();
        match ch {
            ch if ch.is_whitespace() => {
                buffer.clear();
                continue;
            }
            '+' if next.is_none() || next.unwrap().is_whitespace() => {
                tokens.push(Token::Operation(Op::Add))
            }
            '-' if next.is_none() || next.unwrap().is_whitespace() => {
                tokens.push(Token::Operation(Op::Sub))
            }
            '*' if next.is_none() || next.unwrap().is_whitespace() => {
                tokens.push(Token::Operation(Op::Mult))
            }
            '/' if next.is_none() || next.unwrap().is_whitespace() => {
                tokens.push(Token::Operation(Op::Div))
            }
            '0'..='9' | '.' => match next {
                None => {
                    buffer.push(ch);
                    tokens.push(parse(&buffer));
                }
                Some(look_ahead) => match look_ahead {
                    '0'..='9' | '.' => buffer.push(ch),
                    _ if look_ahead.is_whitespace() => {
                        buffer.push(ch);
                        tokens.push(parse(&buffer));
                    }
                    _ => (),
                },
            },
            _ => {
                tokens.push(Token::Unrecognized);
                skip_to_whitespace = true;
            }
        }
    }

    tokens
}

fn parse(chars: &Vec<char>) -> Token {
    if chars.contains(&'.') {
        Token::Float(String::from_iter(chars).parse().unwrap())
    } else {
        Token::Int(String::from_iter(chars).parse().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("2".into()), vec![Token::Int(2)]);
        assert_eq!(tokenize("22".into()), vec![Token::Int(22)]);

        assert_eq!(tokenize("f".into()), vec![Token::Unrecognized]);
        assert_eq!(
            tokenize("22 asdf *(".into()),
            vec![Token::Int(22), Token::Unrecognized, Token::Unrecognized]
        );
        assert_eq!(
            tokenize("2f f32 3f65".into()),
            vec![
                Token::Unrecognized,
                Token::Unrecognized,
                Token::Unrecognized
            ]
        );

        assert_eq!(tokenize("1.4".into()), vec![Token::Float(1.4)]);
        assert_eq!(tokenize("10.".into()), vec![Token::Float(10.0)]);
        assert_eq!(tokenize("10.5".into()), vec![Token::Float(10.5)]);

        assert_eq!(
            tokenize("10.5 4".into()),
            vec![Token::Float(10.5), Token::Int(4)]
        );
        assert_eq!(
            tokenize("  10.5   4    ".into()),
            vec![Token::Float(10.5), Token::Int(4)]
        );

        assert_eq!(
            tokenize("10 4 +".into()),
            vec![Token::Int(10), Token::Int(4), Token::Operation(Op::Add)]
        );
        assert_eq!(
            tokenize("10 4.5 -".into()),
            vec![Token::Int(10), Token::Float(4.5), Token::Operation(Op::Sub)]
        );
        assert_eq!(
            tokenize("10. 4 /".into()),
            vec![Token::Float(10.0), Token::Int(4), Token::Operation(Op::Div)]
        );
        assert_eq!(
            tokenize("10 4 *".into()),
            vec![Token::Int(10), Token::Int(4), Token::Operation(Op::Mult)]
        );

        assert_eq!(
            tokenize("10 4 * 2 +".into()),
            vec![
                Token::Int(10),
                Token::Int(4),
                Token::Operation(Op::Mult),
                Token::Int(2),
                Token::Operation(Op::Add)
            ]
        );
        assert_eq!(
            tokenize("2 10 4 * +".into()),
            vec![
                Token::Int(2),
                Token::Int(10),
                Token::Int(4),
                Token::Operation(Op::Mult),
                Token::Operation(Op::Add)
            ]
        );
    }
}
