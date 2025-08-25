use cinnabar_common::address::Address;

#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
#[allow(dead_code)]
pub enum TokenType {
    //math poerators
    Add,        // +
    Sub,        // -
    Mul,        // *
    Div,        // /
    Pow,        // **
    Mod,        // %
    //syntax sugar
    Assign,     // :=
    ReAssign,   // =
    AssignAdd,  // +=
    AssignSub,  // -=
    AssignMul,  // *=
    AssignDiv,  // /=
    AssignPow,  // **=
    AssignMod,  // %=
    //delimaters
    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }
    LBracket,   // [
    RBracket,   // ]
    Colon,      // :
    Semi,       // ;
    Comma,      // ,
    Dot,        // .
    Pipe,       // |
    //etc
    Bang,       // !
    Question,   // ?
    At,         // @
    Dollar,     // $
    Ampersand,  // &
    Arrow,      // =>
    Under,      // _
    //condition operators
    Greater,    // >
    GreaterEq,  // >=
    Equal,      // ==
    NotEq,      // !=
    LessEq,     // <=
    Less,       // <
    Or,         // ||
    And,        // &&
    //keywords
    Package,    
    Plug,
    Let,
    Set,
    Const,
    As,
    Match,
    Loop,
    Continue,
    Break,
    Void,
    Take,
    Lambda,
    Scatch,
    Impl,
    Iface,
    New,
    Try,
    Catch,
    Throw,
    Macro,
    Quote,
    Unquote,
    Comptime,       //macros comptime!
    //modifiers
    Private,
    Public,
    Static,
    Async, //not use, while will support
    //types
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,

    Uint,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,

    Float,
    Float32,
    Float64,

    Binary,
    Hexonal,

    String,
    Char,
    Bool,
    List,
    Tuple,
    Array,
    Hash,
    Hashlist,
    //literals,
    Text,       // "text"
    Symbol,     // 'c'
    Number,     // 12345.12345
    True,       // true
    False,      // false
    Nil,        // Nil
    Hex,        // 0xFFFFFF
    Bin,        // 0b101010
    //var id
    Id,
    //
    EOF, //end of file
    Comment,

}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Token {
    pub token_type:TokenType,
    pub value: String,
    pub address: Address,
}

impl Token {
    pub fn new(token_type:TokenType, value: String, address:Address) -> Token {
        Token {token_type, value, address,}
    }
}

