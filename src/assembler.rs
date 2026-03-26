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

pub struct Token {
    kind: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    fn new(kind: TokenType, lexeme: String, literal: String, line: usize) -> Self {
        Token {
            kind,
            lexeme,
            literal: Some(literal),
            line,
        }
    }
}

impl fmt::Display for Token {
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

pub struct Lexer {
    stream: Vec<char>,
    pub tokens: Vec<Token>,
    pub position: usize,
    pub ch: char,
    pub error: Option<LexerError>,
    pub line: usize,
}

impl Lexer {
    fn new() -> Self {
        Lexer {
            stream: vec![],
            tokens: vec![],
            position: 0,
            ch: '\0',
            error: None,
            line: 1,
        }
    }

    fn set_stream(mut self, stream: String) -> Self {
        self.stream = stream.chars().collect();
        return self;
    }

    fn consume(&mut self) -> Option<char> {
        if self.position < self.stream.len() {
            self.ch = self.stream[self.position];
            self.position += 1;
            return Some(self.ch);
        } else {
            return None;
        }
    }

    fn peek(&self) -> Option<char> {
        if self.position < self.stream.len() {
            Some(self.stream[self.position])
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
                    self.tokens.push(Token::new(
                        TokenType::TokenNewLine,
                        '\n'.to_string(),
                        '\n'.to_string(),
                        self.line,
                    ));
                    self.line += 1;
                }
                '@' => {
                    let mut lexeme = String::new();
                    lexeme.push(c);

                    if self.position < self.stream.len()
                        && self.stream[self.position].is_alphabetic()
                    {
                        lexeme.push(self.stream[self.position]);
                        self.position += 1;

                        while let Some(next_char) = self.peek() {
                            if next_char.is_alphanumeric() || next_char == '_' {
                                lexeme.push(next_char);
                                _ = self.consume();
                            } else {
                                break;
                            }
                        }

                        self.tokens.push(Token::new(
                            TokenType::TokenVariable,
                            lexeme.clone(),
                            lexeme,
                            self.line,
                        ));
                    } else {
                        let error_str =
                            format!("Error: invalid format after {} at line {}", c, self.line);
                        let error = LexerError::new(error_str);
                        self.error = Some(error);
                    }
                }
                ':' => {
                    self.tokens.push(Token::new(
                        TokenType::TokenColon,
                        ':'.to_string(),
                        ':'.to_string(),
                        self.line,
                    ));
                }
                '!' => {
                    self.tokens.push(Token::new(
                        TokenType::TokenBang,
                        '!'.to_string(),
                        '!'.to_string(),
                        self.line,
                    ));
                }
                ';' => {
                    self.tokens.push(Token::new(
                        TokenType::TokenSemicolon,
                        ';'.to_string(),
                        ';'.to_string(),
                        self.line,
                    ));
                }
                '-' => {
                    if self.position < self.stream.len() && self.stream[self.position] == '>' {
                        let mut lexeme = String::new();
                        lexeme.push(c);
                        lexeme.push(self.stream[self.position]);
                        self.tokens.push(Token::new(
                            TokenType::TokenArrow,
                            lexeme.clone(),
                            lexeme,
                            self.line,
                        ));
                        self.position += 1; // consumindo >
                    } else {
                        self.tokens.push(Token::new(
                            TokenType::TokenMinus,
                            '-'.to_string(),
                            '-'.to_string(),
                            self.line,
                        ));
                    }
                }
                '=' => {
                    self.tokens.push(Token::new(
                        TokenType::TokenEquals,
                        "=".to_string(),
                        "=".to_string(),
                        self.line,
                    ));
                }
                ',' => {
                    self.tokens.push(Token::new(
                        TokenType::TokenComma,
                        ','.to_string(),
                        ','.to_string(),
                        self.line,
                    ));
                }
                _ => {
                    if c.is_alphabetic() {
                        let mut lexeme = String::new();
                        lexeme.push(c);

                        while let Some(ch) = self.peek() {
                            if ch.is_alphanumeric() || ch == '_' {
                                lexeme.push(ch);
                                _ = self.consume(); // consome o char q acabamos de ler
                            } else {
                                break;
                            }
                        }
                        let kind = Lexer::get_reserved_token(&lexeme);
                        self.tokens
                            .push(Token::new(kind, lexeme.clone(), lexeme, self.line));
                    } else if c.is_numeric() {
                        let mut lexeme = String::new();
                        lexeme.push(c);

                        while let Some(ch) = self.peek() {
                            if ch.is_numeric() {
                                lexeme.push(ch);
                                _ = self.consume();
                            } else {
                                break;
                            }
                        }

                        self.tokens.push(Token::new(
                            TokenType::TokenNum,
                            lexeme.clone(),
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
    let mut lexer = Lexer::new().set_stream(data);
    lexer.run();

    for token in lexer.tokens {
        println!("{}", token);
    }

    if let Some(e) = lexer.error {
        println!("{}", e);
    }

    Ok(())
}
