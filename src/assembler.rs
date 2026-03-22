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
    stream: Vec<u8>,
    pub tokens: Vec<Token>,
    pub position: usize,
    pub ch: char,
    pub error: Option<String>,
}

impl Lexer {
    fn new() -> Self {
        Lexer {
            stream: vec![],
            tokens: vec![],
            position: 0,
            ch: '\0',
            error: None,
        }
    }

    fn set_stream(mut self, stream: Vec<u8>) -> Self {
        self.stream = stream;
        return self;
    }

    fn is_char(ch: u8) -> bool {
        return (ch >= 97 && ch <= 122) || (ch >= 65 && ch <= 90);
    }

    fn is_num(ch: u8) -> bool {
        return ch >= 48 && ch <= 57;
    }

    fn run(&mut self) {
        while self.position < self.stream.len() && self.error == None {
            let crr_char = self.stream[self.position];
            match crr_char {
                // ignorar espaços
                // pular comentários
                _ => {}
            }
        }
    }
}

fn main() {}
