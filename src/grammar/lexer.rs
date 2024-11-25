use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Debug)]
pub struct LiteralToken <T> {
    pub literal: T,
    pub lexeme: Lexeme,
    pub line: u32,
    pub character: u32,
}

#[derive(Debug)]
pub struct NonLiteralToken {
    pub lexeme: Lexeme,
    pub line: u32,
    pub character: u32,
}

#[derive(Debug)]
pub struct UnexpectedToken {
    pub lexeme: Lexeme,
    pub line: u32,
    pub character: u32,
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedToken(UnexpectedToken),
}

pub struct Lexeme {
    input: Rc<Vec<char>>,
    start: usize,
    end: usize
}

impl Debug for Lexeme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lexeme")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl Lexeme {
    pub fn get_value(&self) -> &[char] {
        &(*self.input)[self.start..self.end]
    }
}

#[derive(Debug)]
pub enum TokenType {
    Number(LiteralToken<i32>),
    Plus(NonLiteralToken),
    Minus(NonLiteralToken),
    EOF(),
}

struct Scanner {
    input: Rc<Vec<char>>,
    start: usize,
    current: usize,
    line: u32,
    character: u32,
    tokens: Vec<TokenType>
}

impl Scanner {

    fn scan(input: &str) -> Result<Vec<TokenType>, LexerError> {
        let mut scanner = Scanner::new(input);
        while !scanner.is_at_end() {
            scanner.start = scanner.current;
            scanner.scan_token()?;
            scanner.character += (scanner.current - scanner.start) as u32;
        }

        scanner.tokens.push(TokenType::EOF());
        Ok(scanner.tokens)
    }

    fn new(input: &str) -> Scanner {
        Scanner {
            input: Rc::new(input.chars().collect()),
            start: 0,
            current: 0,
            line: 1,
            character: 1,
            tokens: Vec::new()
        }
    }

    fn scan_token(&mut self) -> Result<(), LexerError> {
        let c = self.advance().unwrap();
        match c {
            // Handle whitespace
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.newline(),

            // Simple tokens
            '+' => self.add_plus_token(),
            '-' => self.add_minus_token(),

            // Longer tokens
            c if c.is_digit(10) => self.number(),

            // identifiers
            'a'..='z' | 'A'..='Z' | '_' => {}

            _ => return Err(self.unexpected_token_error()),
        }

        Ok(())
    }

    fn newline(&mut self) {
        self.line += 1;
        self.character = 1;
        self.start = self.current;
    }

    fn number(&mut self) {
        while self.peek().map_or(false, |c| c.is_digit(10)) {
            self.advance();
        }

        self.add_number_token()
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() { return None; }

        let result = self.input[self.current];
        self.current += 1;
        Some(result)
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() { return None; }
        Some(self.input[self.current])
    }

    fn add_plus_token(&mut self) {
        self.tokens.push(TokenType::Plus(NonLiteralToken {
            lexeme: self.get_current_lexeme(),
            line: self.line,
            character: self.character,
        }))
    }

    fn add_minus_token(&mut self) {
        self.tokens.push(TokenType::Minus(NonLiteralToken {
            lexeme: self.get_current_lexeme(),
            line: self.line,
            character: self.character,
        }))
    }

    fn add_number_token(&mut self) {
        let s = self.input[self.start..self.current].iter().collect::<String>();
        let value = s.parse::<i32>().unwrap();
        self.tokens.push(TokenType::Number(LiteralToken {
            literal: value,
            lexeme: self.get_current_lexeme(),
            line: self.line,
            character: self.character,
        }));
    }

    fn unexpected_token_error(&self) -> LexerError {
        LexerError::UnexpectedToken(UnexpectedToken {
            lexeme: self.get_current_lexeme(),
            line: self.line,
            character: self.character,
        })
    }

    fn get_current_lexeme(&self) -> Lexeme {
        Lexeme {
            input: self.input.clone(),
            start: self.start,
            end: self.current
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
}

pub fn get_tokens(input: &str) -> Result<Vec<TokenType>, LexerError> {
    Ok(Scanner::scan(input)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOKEN_WRONG_LITERAL: &str = "Token literal value is not correct";
    const TOKEN_WRONG_LINE: &str = "Token is on wrong line";
    const TOKEN_WRONG_CHARACTER: &str = "Token is marked by wrong character";
    const TOKEN_WRONG_LEXEME: &str = "Token lexeme does not match";
    const UNEXPECTED_TOKEN_MATCH: &str = "The expected token did not match";

    #[test]
    fn test_simple_addition() -> Result<(), String> {
        let result = get_tokens("2+3").unwrap();

        assert_number_token(&result[0], 2, 1, 1, "2");
        assert_plus_token(&result[1], 1, 2);
        assert_number_token(&result[2], 3, 1, 3, "3");
        assert!(matches!(&result[3], TokenType::EOF()));
        assert_eq!(result.len(), 4);

        Ok(())
    }

    #[test]
    fn test_with_whitespace() -> Result<(), String> {
        let result = get_tokens("2 + \t\r\n3").unwrap();

        assert_number_token(&result[0], 2, 1, 1, "2");
        assert_plus_token(&result[1], 1, 3);
        assert_number_token(&result[2], 3, 2, 1, "3");
        assert!(matches!(&result[3], TokenType::EOF()));
        assert_eq!(result.len(), 4);

        Ok(())
    }

    #[test]
    fn test_subtraction() -> Result<(), String> {
        let result = get_tokens("-").unwrap();

        match &result[0] {
            TokenType::Minus(token) => {
                assert_eq!(token.line, 1, "{}", TOKEN_WRONG_LINE);
                assert_eq!(token.character, 1, "{}", TOKEN_WRONG_CHARACTER);
                assert_eq!(token.lexeme.get_value(), str_to_char_slice("-"), "{}", TOKEN_WRONG_LEXEME);
            }
            _ => assert!(false, "{}", UNEXPECTED_TOKEN_MATCH)
        }

        assert!(matches!(&result[1], TokenType::EOF()));
        assert_eq!(result.len(), 2);

        Ok(())
    }

    #[test]
    fn test_unexpected_token() -> Result<(), String> {
        let result = get_tokens("`");
        match result {
            Err(LexerError::UnexpectedToken(token)) => {
                assert_eq!(token.line, 1, "{}", TOKEN_WRONG_LINE);
                assert_eq!(token.character, 1, "{}", TOKEN_WRONG_CHARACTER);
                assert_eq!(token.lexeme.get_value(), str_to_char_slice("`"), "{}", TOKEN_WRONG_LEXEME)
            },
            _ => assert!(false, "{}", UNEXPECTED_TOKEN_MATCH)
        }

        Ok(())
    }

    #[test]
    fn test_big_numbers() -> Result<(), String> {
        let result = get_tokens("123 + 456").unwrap();

        assert_number_token(&result[0], 123, 1, 1, "123");
        assert_plus_token(&result[1], 1, 5);
        assert_number_token(&result[2], 456, 1, 7, "456");
        assert!(matches!(&result[3], TokenType::EOF()));
        assert_eq!(result.len(), 4);

        Ok(())
    }

    fn assert_number_token(
        token_type: &TokenType,
        literal: i32,
        line: u32,
        character: u32,
        lexeme: &str
    ) {
        match token_type {
            TokenType::Number(token) => {
                assert_eq!(token.literal, literal, "{}", TOKEN_WRONG_LITERAL);
                assert_eq!(token.line, line, "{}", TOKEN_WRONG_LINE);
                assert_eq!(token.character, character, "{}", TOKEN_WRONG_CHARACTER);
                assert_eq!(token.lexeme.get_value(), str_to_char_slice(lexeme), "{}", TOKEN_WRONG_LEXEME);
            }
            _ => assert!(false, "{}", UNEXPECTED_TOKEN_MATCH)
        }
    }

    fn assert_plus_token(token_type: &TokenType, line: u32, character: u32) {
        match token_type {
            TokenType::Plus(token) => {
                assert_eq!(token.line, line, "{}", TOKEN_WRONG_LINE);
                assert_eq!(token.character, character, "{}", TOKEN_WRONG_CHARACTER);
                assert_eq!(token.lexeme.get_value(), str_to_char_slice("+"), "{}", TOKEN_WRONG_LEXEME);
            }
            _ => assert!(false, "{}", UNEXPECTED_TOKEN_MATCH)
        }
    }

    fn str_to_char_slice(s: &str) -> Vec<char> {
        s.chars().collect()
    }

}
