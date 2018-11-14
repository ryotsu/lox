use super::utils;
use crate::ast::{
    Assignment, Binary, Call, Expression, Literal, Logical, LogicalOp, Primary, Unary,
};
use crate::tokens::{Token, TokenType::*};
use std::iter::Peekable;

macro_rules! make_function {
    ($name:ident, $fun:path, $($x:expr),+) => {
        fn $name<T>(tokens: &mut Peekable<T>) -> Result<Expression, String>
        where
            T: Iterator<Item=Token>
        {
            let mut expr = $fun(tokens)?;

            while Some(true) == check_next_token!(tokens, $($x),+) {
                let op = utils::map_binary_op(tokens.next().unwrap().token_type);
                let right = $fun(tokens)?;
                expr = Expression::Binary(Binary{
                    op: op,
                    left: Box::new(expr),
                    right: Box::new(right),
                });
            }

            Ok(expr)
        }
    };
}

impl Expression {
    pub fn parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Self, String> {
        Assignment::parse(tokens)
    }
}

impl Assignment {
    fn parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Expression, String> {
        let expr = Logical::parse(tokens)?;

        if match_next_token!(tokens, EQUAL) {
            let value = Assignment::parse(tokens)?;

            return Ok(match expr {
                Expression::Literal(Literal::Variable(variable)) => {
                    Expression::Assignment(Assignment {
                        variable: variable,
                        value: Box::new(value),
                    })
                }
                _ => return Err(format!("Invalid assignment target")),
            });
        }

        Ok(expr)
    }
}

impl Logical {
    fn parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Expression, String> {
        let mut expr = Self::parse_logical_and(tokens)?;

        while match_next_token!(tokens, OR) {
            let right = Self::parse_logical_and(tokens)?;
            expr = Expression::Logical(Logical {
                op: LogicalOp::Or,
                left: Box::new(expr),
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn parse_logical_and<T>(tokens: &mut Peekable<T>) -> Result<Expression, String>
    where
        T: Iterator<Item = Token>,
    {
        let mut expr = Binary::parse(tokens)?;

        while match_next_token!(tokens, AND) {
            let right = Binary::parse(tokens)?;
            expr = Expression::Logical(Logical {
                op: LogicalOp::And,
                left: Box::new(expr),
                right: Box::new(right),
            })
        }

        Ok(expr)
    }
}

impl Binary {
    fn parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Expression, String> {
        Self::equality(tokens)
    }

    make_function!(equality, Self::comparison, EQUAL_EQUAL, BANG_EQUAL);
    make_function!(
        comparison,
        Self::addition,
        GREATER,
        GREATER_EQUAL,
        LESS,
        LESS_EQUAL
    );
    make_function!(addition, Self::multiplication, PLUS, HYPHEN);
    make_function!(multiplication, Unary::parse, SLASH, ASTERICS);
}

impl Unary {
    fn parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Expression, String> {
        use crate::ast::UnaryOp::*;

        if Some(true) == check_next_token!(tokens, BANG, HYPHEN) {
            let op = if tokens.next().unwrap().token_type == BANG {
                Not
            } else {
                Minus
            };
            let expression = Self::parse(tokens)?;
            return Ok(Expression::Unary(Unary {
                op: op,
                expression: Box::new(expression),
            }));
        }

        Ok(Call::parse(tokens)?)
    }
}

impl Call {
    fn parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Expression, String> {
        let mut expr = Literal::parse(tokens)?;

        while match_next_token!(tokens, LEFT_PAREN) {
            expr = Self::finish(tokens, expr)?;
        }

        Ok(expr)
    }

    fn finish<T>(tokens: &mut Peekable<T>, expr: Expression) -> Result<Expression, String>
    where
        T: Iterator<Item = Token>,
    {
        let mut args = vec![];
        if Some(false) == check_next_token!(tokens, RIGHT_PAREN) {
            args.push(Expression::parse(tokens)?);
            while match_next_token!(tokens, COMMA) {
                args.push(Expression::parse(tokens)?);
            }
        }

        utils::consume(tokens, RIGHT_PAREN, "Expect ) after function arguments")?;

        Ok(Expression::Call(Call {
            callee: Box::new(expr),
            arguments: args,
        }))
    }
}

impl Literal {
    fn parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Expression, String> {
        use self::Literal::*;
        use self::Primary::*;

        let literal = match tokens.next().unwrap().token_type {
            IDENTIFIER(identifier) => Variable(identifier),
            NUMBER(num) => Primary(Number(num)),
            STRING(s) => Primary(String(s)),
            TRUE => Primary(Boolean(true)),
            FALSE => Primary(Boolean(false)),
            NIL => Primary(Nil),
            LEFT_PAREN => {
                let expr = Expression::parse(tokens)?;
                utils::consume(tokens, RIGHT_PAREN, "Expect ) after expression")?;
                return Ok(Expression::Grouping(Box::new(expr)));
            }
            _ => return Err(format!("Unexpected character")),
        };

        Ok(Expression::Literal(literal))
    }
}
