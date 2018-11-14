use super::utils;
use crate::ast::{Block, Declaration, Expression, Function, If, Statement, While};
use crate::tokens::{Token, TokenType::*};
use std::iter::Peekable;

impl Statement {
    pub fn parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Self, String> {
        if match_next_token!(tokens, VAR) {
            Declaration::parse(tokens)
        } else if match_next_token!(tokens, FUN) {
            Function::parse(tokens)
        } else if match_next_token!(tokens, PRINT) {
            Self::print(tokens)
        } else if match_next_token!(tokens, LEFT_BRACE) {
            Block::parse(tokens)
        } else if match_next_token!(tokens, IF) {
            If::parse(tokens)
        } else if match_next_token!(tokens, WHILE) {
            While::parse(tokens)
        } else if match_next_token!(tokens, FOR) {
            While::parse_for(tokens)
        } else {
            Self::expression(tokens)
        }
    }
}

impl Statement {
    fn print<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Self, String> {
        let expr = Expression::parse(tokens)?;
        utils::consume(tokens, SEMICOLON, "Expect ';' after print statement")?;
        Ok(Statement::Print(expr))
    }

    fn expression<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Self, String> {
        println!("{:?}", tokens.peek());
        let expr = Expression::parse(tokens)?;
        utils::consume(tokens, SEMICOLON, "Expect ';' after statement")?;
        Ok(Statement::Expression(expr))
    }
}

impl Declaration {
    fn parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Statement, String> {
        let name = utils::get_identifier(tokens)?;
        let mut value = None;

        if match_next_token!(tokens, EQUAL) {
            value = Some(Expression::parse(tokens)?);
        }

        utils::consume(tokens, SEMICOLON, "Expect ';' after variable declaration")?;
        Ok(Statement::Declaration(Self {
            name: name,
            value: value,
        }))
    }
}

impl Function {
    fn parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Statement, String> {
        let name = utils::get_identifier(tokens)?;
        utils::consume(tokens, LEFT_PAREN, "Expect '(' after function name")?;

        let mut params = vec![];
        if Some(false) == check_next_token!(tokens, RIGHT_PAREN) {
            params.push(utils::get_identifier(tokens)?);
            while match_next_token!(tokens, COMMA) {
                params.push(utils::get_identifier(tokens)?);
            }
        }
        utils::consume(tokens, RIGHT_PAREN, "Expect ')' after function params")?;

        utils::consume(tokens, LEFT_BRACE, "Expect '{' after function params")?;
        let body = Block::parse(tokens)?;

        Ok(Statement::Function(Function {
            name: name,
            args: params,
            body: Box::new(body),
        }))
    }
}

impl Block {
    fn parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Statement, String> {
        let mut statements = vec![];

        while Some(false) == check_next_token!(tokens, RIGHT_BRACE) {
            statements.push(Statement::parse(tokens)?);
        }
        utils::consume(tokens, RIGHT_BRACE, "Expect } after block")?;

        Ok(Statement::Block(Block {
            statements: statements,
        }))
    }
}

impl If {
    fn parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Statement, String> {
        utils::consume(tokens, LEFT_PAREN, "Expect '(' after if")?;
        let expr = Expression::parse(tokens)?;
        utils::consume(tokens, RIGHT_PAREN, "Expect ')' after condition")?;

        let then_branch = Statement::parse(tokens)?;
        let mut else_branch = None;
        if match_next_token!(tokens, ELSE) {
            else_branch = Some(Box::new(Statement::parse(tokens)?));
        }

        Ok(Statement::If(If {
            cond: expr,
            success: Box::new(then_branch),
            failure: else_branch,
        }))
    }
}

impl While {
    fn parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Statement, String> {
        utils::consume(tokens, LEFT_PAREN, "Expect '(' after 'while'")?;
        let cond = Expression::parse(tokens)?;
        utils::consume(tokens, RIGHT_PAREN, "Expect ')' after condition")?;
        let body = Statement::parse(tokens)?;

        Ok(Statement::While(While {
            cond: cond,
            body: Box::new(body),
        }))
    }

    fn parse_for<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Statement, String> {
        use crate::ast::{Literal, Primary};

        utils::consume(tokens, LEFT_PAREN, "Expect '(' after 'for'")?;
        let init = if match_next_token!(tokens, SEMICOLON) {
            None
        } else if match_next_token!(tokens, VAR) {
            Some(Declaration::parse(tokens)?)
        } else {
            Some(Statement::expression(tokens)?)
        };

        let cond = if Some(true) == check_next_token!(tokens, SEMICOLON) {
            None
        } else {
            Some(Expression::parse(tokens)?)
        };
        utils::consume(tokens, SEMICOLON, "Exprect ';' after condition")?;

        let increment = if Some(false) == check_next_token!(tokens, RIGHT_PAREN) {
            Some(Expression::parse(tokens)?)
        } else {
            None
        };
        utils::consume(tokens, RIGHT_PAREN, "Exprect ')' after for clauses")?;

        let mut body = Statement::parse(tokens)?;

        if let Some(inc) = increment {
            body = Statement::Block(Block {
                statements: vec![body, Statement::Expression(inc)],
            })
        }

        body = if let Some(c) = cond {
            Statement::While(While {
                cond: c,
                body: Box::new(body),
            })
        } else {
            Statement::While(While {
                cond: Expression::Literal(Literal::Primary(Primary::Boolean(true))),
                body: Box::new(body),
            })
        };

        if let Some(i) = init {
            body = Statement::Block(Block {
                statements: vec![i, body],
            });
        }

        Ok(body)
    }
}
