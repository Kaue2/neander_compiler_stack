use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy)]
enum TokenType {
    // Delimitadores Iniciadores
    TokenLabel,

    // Variaveis
    TokenIdentfier,
    TokenVariable,

    // Instrucoes
    TokenInstructionSetUp,
    TokenInstruction,

    // Literais
    TokenNum,

    // Simbolos unicos
    TokenEquals,
    TokenBang,
    TokenColon,
    TokenSemicolon,
    TokenArrow,
    TokenComma,
    TokenSpace,
    TokenTab,
    TokenSlashR,
    TokenNewLine,
    TokenMinus,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TokenType::TokenLabel => "Label",
            TokenType::TokenIdentfier => "Identfier",
            TokenType::TokenVariable => "Variable",
            TokenType::TokenSemicolon => "Semi",
            TokenType::TokenInstructionSetUp => "Instruction Setup",
            TokenType::TokenInstruction => "Instruction",
            TokenType::TokenMinus => "Minus",
            TokenType::TokenNum => "Number",
            TokenType::TokenEquals => "Equals",
            TokenType::TokenBang => "Bang",
            TokenType::TokenColon => "Colon",
            TokenType::TokenArrow => "Arrow",
            TokenType::TokenComma => "Comma",
            TokenType::TokenSpace => "Space",
            TokenType::TokenTab => "Tab",
            TokenType::TokenSlashR => "Return of line",
            TokenType::TokenNewLine => "New Line",
        };

        write!(f, "{}", name)
    }
}

pub struct Token<'a> {
    kind: TokenType,
    lexeme: &'a str,
    literal: Option<&'a str>,
    line: usize,
}

impl<'a> Token<'a> {
    fn new(kind: TokenType, lexeme: &'a str, literal: &'a str, line: usize) -> Self {
        Token {
            kind,
            lexeme,
            literal: Some(literal),
            line,
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token -> \n\tTipo: {} \n\tLexeme: {} \n\tLine: {}",
            self.kind,
            self.lexeme.escape_debug(),
            self.line
        )
    }
}

pub struct Lexer<'a> {
    stream: &'a str,
    pub tokens: Vec<Token<'a>>,
    pub position: usize,
    pub ch: char,
    pub error: Option<LexerError>,
    pub line: usize,
}

impl<'a> Lexer<'a> {
    fn new(stream: &'a str) -> Self {
        Lexer {
            stream,
            tokens: vec![],
            position: 0,
            ch: '\0',
            error: None,
            line: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.stream[self.position..].chars().next()
    }

    fn consume(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            self.position += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    fn get_reserved_token(lexeme: &str) -> TokenType {
        let kind = match lexeme {
            "data" | "program" | "end" => TokenType::TokenLabel,
            _ => TokenType::TokenIdentfier,
        };
        return kind;
    }

    fn run(&mut self) {
        // enquanto funcao avancar funcionar continua
        while let Some(c) = self.consume() {
            if self.error.is_some() {
                break;
            }

            match c {
                // ignorar espaços
                // pular comentários
                ' ' | '\t' | '\r' => {
                    continue;
                }
                '\n' => {
                    let lexeme = &self.stream[self.position..self.position];
                    self.tokens.push(Token::new(
                        TokenType::TokenNewLine,
                        lexeme,
                        lexeme,
                        self.line,
                    ));
                    self.line += 1;
                }
                '@' => {
                    let start_pos = self.position - 1;

                    if let Some(next_char) = self.peek() {
                        if next_char.is_alphabetic() {
                            self.consume();

                            while let Some(c) = self.peek() {
                                if c.is_alphabetic() {
                                    self.consume();
                                } else {
                                    break;
                                }
                            }

                            let lexeme = &self.stream[start_pos..self.position];
                            self.tokens.push(Token::new(
                                TokenType::TokenVariable,
                                lexeme,
                                lexeme,
                                self.line,
                            ));
                        }
                    } else {
                        let error_str =
                            format!("Error: invalid format after {} at line {}", c, self.line);
                        let error = LexerError::new(error_str);
                        self.error = Some(error);
                    }
                }
                ':' => {
                    let lexeme = &self.stream[self.position..self.position];
                    self.tokens
                        .push(Token::new(TokenType::TokenColon, lexeme, lexeme, self.line));
                }
                '!' => {
                    let lexeme = &self.stream[self.position..self.position];
                    self.tokens
                        .push(Token::new(TokenType::TokenBang, lexeme, lexeme, self.line));
                }
                ';' => {
                    let lexeme = &self.stream[self.position..self.position];
                    self.tokens.push(Token::new(
                        TokenType::TokenSemicolon,
                        lexeme,
                        lexeme,
                        self.line,
                    ));
                }
                '-' => {
                    let start_pos = self.position;
                    if let Some(next_char) = self.peek() {
                        if next_char == '>' {
                            self.consume();

                            let lexeme = &self.stream[start_pos..self.position];
                            self.tokens.push(Token::new(
                                TokenType::TokenArrow,
                                lexeme,
                                lexeme,
                                self.line,
                            ));
                        }
                    } else {
                        let lexeme = &self.stream[start_pos..self.position];
                        self.tokens.push(Token::new(
                            TokenType::TokenMinus,
                            lexeme,
                            lexeme,
                            self.line,
                        ));
                    }
                }
                '=' => {
                    let lexeme = &self.stream[self.position..self.position];
                    self.tokens.push(Token::new(
                        TokenType::TokenEquals,
                        lexeme,
                        lexeme,
                        self.line,
                    ));
                }
                ',' => {
                    let lexeme = &self.stream[self.position..self.position];
                    self.tokens
                        .push(Token::new(TokenType::TokenComma, lexeme, lexeme, self.line));
                }
                _ => {
                    if c.is_alphabetic() {
                        let start_pos = self.position - 1;

                        while let Some(ch) = self.peek() {
                            if ch.is_alphanumeric() || ch == '_' {
                                self.consume();
                            } else {
                                break;
                            }
                        }
                        let lexeme = &self.stream[start_pos..self.position];
                        let kind = Lexer::get_reserved_token(lexeme);
                        self.tokens
                            .push(Token::new(kind, lexeme, lexeme, self.line));
                    } else if c.is_numeric() {
                        let start_pos = self.position - 1;

                        while let Some(ch) = self.peek() {
                            if ch.is_numeric() {
                                self.consume();
                            } else {
                                break;
                            }
                        }

                        let lexeme = &self.stream[start_pos..self.position];
                        self.tokens.push(Token::new(
                            TokenType::TokenNum,
                            lexeme,
                            lexeme,
                            self.line,
                        ));
                    } else {
                        let error_str =
                            format!("Unexpected symbol \"{}\" at line {}", c, self.line);
                        let error = LexerError::new(error_str);
                        self.error = Some(error);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct LexerError {
    message: String,
}

impl LexerError {
    fn new(error: String) -> Self {
        LexerError { message: error }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ERROR: {}", self.message)
    }
}

impl Error for LexerError {}

fn main() -> Result<(), Box<dyn Error>> {
    let data = std::fs::read_to_string("utils/asm.txt").expect("ERROR: unable to read file.");
    let mut lexer = Lexer::new(&data);
    lexer.run();

    for token in lexer.tokens {
        println!("{}", token);
    }

    if let Some(e) = lexer.error {
        println!("{}", e);
    }

    Ok(())
}
