pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

trait LiteralTrait {
    fn manage_the_literal(&self);
}

pub struct Token {
    lexeme: &str,
    token_definition: TokenType,
    literal: Box<dyn LiteralTrait>, // this will dynamically handle the definitions of the language
    line: usize,
}

impl LiteralTrait for i32 {
    // this do something still planing
    fn manage_the_literal(&self) {
        //do something here
        println!("{}", self);
    }
}

impl LiteralTrait for String {
    // this is another overwrite for Literal Traint that willchange depending what this recive
}

// is this necessary ?
// no, this is not necessary, just force og habit i guess XD, will correct soon
impl Token {
    pub fn set_lexeme(&mut self, lexeme: &str) {
        self.lexeme = lexeme;
    }

    pub fn get_lexeme(&self) -> &str {
        self.lexeme
    }

    pub fn set_literal(&mut self, literal: Box<dyn LiteralTrait>) {
        self.literal = literal;
    }

    pub fn get_literal(&self) -> Box<dyn LiteralTrait> {
        self.literal
    }

    pub fn set_token_definition(&mut self, token_definition: TokenType) {
        self.token_definition = token_definition;
    }

    pub fn get_token_definition(&self) -> TokenType {
        self.token_definition
    }

    pub fn set_line(&mut self, line: &usize) {
        self.line = *line;
    }

    pub fn get_line(&self) -> &usize {
        *self.line
    }
}

/*
 *this is a java example to take into account:
 *interface pastryVisitor{
*
 *}

*/
