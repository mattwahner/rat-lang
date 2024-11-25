use crate::grammar::lexer::TokenType;

#[derive(Debug)]
pub enum TermOperator {
    Plus,
    Minus
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expression>,
    pub operator: TermOperator,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub struct Integer {
    pub value: i32
}

#[derive(Debug)]
pub enum Expression {
    Binary(Binary),
    Integer(Integer),
}

struct Parser <'a> {
    tokens: &'a Vec<TokenType>,
    current: usize,
}

impl <'a> Parser <'a> {
    pub fn parse(input: &Vec<TokenType>) -> Expression {
        let mut parser = Parser::new(input);
        parser.term()
    }

    fn new(input: &Vec<TokenType>) -> Parser {
        Parser {
            tokens: input,
            current: 0,
        }
    }

    fn term(&mut self) -> Expression {
        let mut number = self.number();

        while self.match_term_operand() {
            let operator = match self.previous() {
                Some(TokenType::Plus(_)) => TermOperator::Plus,
                Some(TokenType::Minus(_)) => TermOperator::Minus,
                _ => panic!("Shouldnt have happened")
            };
            let right = self.number();
            number = Expression::Binary(Binary {
                left: Box::new(number),
                operator,
                right: Box::new(right)
            });
        }

        number
    }

    fn match_term_operand(&mut self) -> bool {
        matches!(self.advance(), Some(TokenType::Plus(_)) | Some(TokenType::Minus(_)))
    }

    fn number(&mut self) -> Expression {
        match self.advance() {
            Some(TokenType::Number(number)) => Expression::Integer(Integer {
                value: number.literal
            }),
            _ => panic!("Shouldnt have happened")
        }
    }

    fn advance(&mut self) -> Option<&TokenType> {
        if self.is_at_end() { return None; }

        let result = &self.tokens[self.current];
        self.current += 1;
        Some(result)
    }

    fn peek(&self) -> Option<&TokenType> {
        if self.is_at_end() { return None; }
        Some(&self.tokens[self.current])
    }

    fn previous(&self) -> Option<&TokenType> {
        if self.current == 0 || self.is_at_end() { return None; }
        Some(&self.tokens[self.current - 1])
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}

pub fn get_ast(tokens: &Vec<TokenType>) -> Expression {
    Parser::new(tokens).term()
}
