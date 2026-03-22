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
}

fn main() {}
