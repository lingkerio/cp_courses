#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    As, Break, Const, Continue, Crate, Else, Enum, Extern, False, Fn,
    For, If, Impl, In, Let, Loop, Match, Mod, Move, Mut, Pub, Ref,
    Return, SELFVALUE, SELFTYPE, Static, Struct, Super, Trait, True, Type, Unsafe, Use,
    Where, While, Async, Await, Dyn,
    // Reserved keywords
    Abstract, Become, Box, Do, Final, Macro, Override, Priv, Typeof,
    Unsized, Virtual, Yield, Try,
    // Weak keywords
    MacroRules, Union, StaticLifetime,

    // Identifiers
    Identifier(String),

    // Literals
    CharLiteral(char),
    StringLiteral(String),
    IntegerLiteral(String),
    FloatLiteral(String),

    // Lifetimes and Labels
    LifetimeOrLabel(String),

    // Comments
    Comment(String),

    // Whitespace
    // Whitespace,

    // Punctuation
    Plus, Minus, Star, Slash, Percent, Caret, Not, And, Or, AndAnd, OrOr,
    Shl, Shr, PlusEq, MinusEq, StarEq, SlashEq, PercentEq, CaretEq, AndEq, OrEq,
    ShlEq, ShrEq, Eq, EqEq, Ne, Gt, Lt, Ge, Le, At, Underscore, Dot, DotDot,
    DotDotDot, DotDotEq, Comma, Semi, Colon, PathSep, RArrow, FatArrow, LArrow,
    Pound, Dollar, Question, Tilde,

    // Delimiters
    OpenBrace, CloseBrace, OpenBracket, CloseBracket, OpenParen, CloseParen,

    // Unknown (fallback case)
    Unknown(char),
}
