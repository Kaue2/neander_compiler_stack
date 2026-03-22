use std::error::Error;
use std::fmt;

enum TokenType {
    // Delimitadores Iniciadores
    TokenSetup,
    TokenData,
    TokenBeginning,

    // Delimitadores Fim
    TokemEnd,

    // Variaveis
    TokenVar,

    // Instrucoes
    TokenInstructionSetUp,
    TokenInstruction,

    // Enderecos
    TokenAddress,

    // Literais
    TokenNum,

    // Simbolos unicos
    TokenEquals,
    TokenColon,
    TokenArrow,
    TokenComma,
}

pub struct Token {
    kind: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: u16,
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
            line: 0,
        }
    }

    fn set_stream(mut self, stream: String) -> Self {
        self.stream = stream.chars().collect();
        return self;
    }

    fn run(&mut self) -> Result<(), LexerError> {
        while self.position < self.stream.len() {
            let crr_char = self.stream[self.position];
            match crr_char {
                // ignorar espaços
                // pular comentários
                _ => {
                    let error_str =
                        format!("Unexpected symbol \"{}\" at line {}", crr_char, self.line);
                    let error = LexerError::new(error_str);
                    self.error = Some(error.clone());
                    return Err(error);
                }
            }
        }

        Ok(())
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

    if let Some(e) = lexer.error {
        println!("{}", e);
    }

    Ok(())
}
