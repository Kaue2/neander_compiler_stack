use std::error::Error;
use std::fmt;
use std::fmt::Display;

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
    TokenColon,
    TokenArrow,
    TokenComma,
    TokenSpace,
    TokenTab,
    TokenSlashR,
    TokenNewLine,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TokenType::TokenLabel => "Label",
            TokenType::TokenIdentfier => "Identfier",
            TokenType::TokenVariable => "Variable",
            TokenType::TokenInstructionSetUp => "Instruction Setup",
            TokenType::TokenInstruction => "Instruction",
            TokenType::TokenNum => "Number",
            TokenType::TokenEquals => "Equals",
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

    fn create_token(kind: TokenType, lexeme: String, literal: String, line: usize) -> Token {
        Token {
            kind,
            lexeme,
            literal: Some(literal),
            line,
        }
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
                ' ' | '\t' | '\r' => match c {
                    ' ' => {
                        self.tokens.push(Lexer::create_token(
                            TokenType::TokenSpace,
                            ' '.to_string(),
                            ' '.to_string(),
                            self.line,
                        ));
                    }
                    '\t' => {
                        self.tokens.push(Lexer::create_token(
                            TokenType::TokenTab,
                            '\t'.to_string(),
                            '\t'.to_string(),
                            self.line,
                        ));
                    }
                    '\r' => {
                        self.tokens.push(Lexer::create_token(
                            TokenType::TokenSlashR,
                            '\r'.to_string(),
                            '\r'.to_string(),
                            self.line,
                        ));
                    }
                    _ => {
                        let error_str = format!("Error: invalid char {} at {}", c, self.line);
                        let error = LexerError::new(error_str);
                        self.error = Some(error);
                    }
                },
                '\n' => {
                    self.tokens.push(Lexer::create_token(
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

                        while self.position < self.stream.len() {
                            let next_char = self.stream[self.position];

                            if next_char.is_alphanumeric() || next_char == '_' {
                                lexeme.push(next_char);
                                self.position += 1;
                            } else {
                                break;
                            }
                        }

                        self.tokens.push(Lexer::create_token(
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
                ':' => {}
                '!' => {}
                ';' => {}
                '-' => {
                    if self.stream[self.position] == '>' {
                        let mut lexeme = String::new();
                        lexeme.push(c);
                        lexeme.push(self.stream[self.position]);
                        let token = Lexer::create_token(
                            TokenType::TokenArrow,
                            lexeme.clone(),
                            lexeme,
                            self.line,
                        );
                        self.tokens.push(token);
                        self.position += 1;
                    }
                }
                '=' => {}
                ',' => {}
                _ => {
                    if c.is_alphabetic() {
                    } else if c.is_numeric() {
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
