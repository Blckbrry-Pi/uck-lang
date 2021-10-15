use logos::Logos;
#[derive(Clone, Debug, Logos, PartialEq)]
pub enum LexerToken<'a> {

    /*

        KEYWORDS SECTION

    */

    // Module keywords

    #[regex(r"import")]
    Import,
    #[regex(r"from")]
    From,

    #[regex(r"default")]
    Default,
    #[regex(r"export")]
    Export,


    // Type stuff

    #[regex(r"enum")]
    Enum,
    #[regex(r"struct")]
    Struct,
    #[regex(r"class")]
    Class,
    #[regex(r"type")]
    Type,

    #[regex(r"fun")]
    Function,

    #[regex(r"as")]
    As,


    // Class/Struct constraints.
    
    #[regex(r"extends")]
    Extends,
    #[regex(r"implements")]
    Implements,


    // Conditional Branching

    #[regex(r"if")]
    If,
    #[regex(r"else")]
    Else,

    #[regex(r"match")]
    Match,


    // Loops

    #[regex(r"for")]
    For,
    #[regex(r"while")]
    While,
    #[regex(r"loop")]
    Loop,

    #[regex(r"break")]
    Break,
    #[regex(r"continue")]
    Continue,


    // Variables

    #[regex(r"let")]
    Let,
    #[regex(r"const")]
    Const,

    #[regex(r"mut")]
    Mutable,


    // Evaluation

    #[regex(r"return")]
    Return,
    #[regex(r"yield")]
    Yield,



    /*

        PUNCTUATION

    */

    #[regex(r",")]
    Comma,

    #[regex(r":")]
    Colon,
    #[regex(r";")]
    Semicolon,

    #[regex(r"\(")]
    LeftParenthesis,
    #[regex(r"\)")]
    RightParenthesis,

    #[regex(r"\{")]
    LeftCurlyBrace,
    #[regex(r"\}")]
    RightCurlyBrace,

    #[regex(r"\[")]
    LeftSquareBracket,
    #[regex(r"\]")]
    RightSquareBracket,

    #[regex(r"<")]
    LeftAngleBracketOrLessThan,
    #[regex(r">")]
    RightAngleBracketOrGreaterThan,



    /*

        SYMBOLS SECTION

    */

    // Operators

    #[regex(r"\*\*")]
    DoubleAsterisk,
    #[regex(r"\*")]
    Asterisk,
    #[regex(r"/")]
    ForwardSlash,
    #[regex(r"\+")]
    Plus,
    #[regex(r"\-")]
    Dash,

    #[regex(r"%")]
    Percent,
    
    #[regex(r"\.\.")]
    DoubleDot,
    
    #[regex(r"&")]
    Ampersand,
    #[regex(r"&&")]
    DoubleAmpersand,

    #[regex(r"\|")]
    VerticalBar,
    #[regex(r"\|\|")]
    DoubleVerticalBar,

    #[regex(r"\^")]
    Caret,

    #[regex(r"<<")]
    DoubleLeftAngleBracket,
    #[regex(r">>")]
    DoubleRightAngleBracket,

    #[regex(r"==")]
    EqualTo,
    #[regex(r"!=")]
    NotEqualTo,
    
    #[regex(r">=")]
    GreaterThanOrEqualTo,
    #[regex(r"<=")]
    LessThanOrEqualTo,

    #[regex(r"=")]
    Assign,

    #[regex(r"\.")]
    MemberAccess,

    #[regex(r"\?")]
    Optional,
    #[regex(r"!")]
    Bang,

    #[regex(r"(~)|(\$)|(@)|(#)")]
    Reserved,


    // Arrows

    #[regex(r"->")]
    ThinArrow,
    #[regex(r"=>")]
    ThiccArrow,
    


    /*
    
        IDENTIFIERS SECTION (with literally just identifiers)

    */

    #[regex(r"[\p{L}_][\p{L}\p{N}_]*")]
    Identifier(&'a str),



    /*

        LITERALS SECTION

    */

    // String literals

    #[regex(r#""([^\\"]|(\\[\S\s]))*""#, |lex| lex.slice().parse())]
    #[regex(r#"l"[^"]*""#, |lex| lex.slice().parse())]
    StriLiteral(String),
    #[regex(r#"'([^\\']|(\\[\S\s]))'"#, |lex| lex.slice().chars().next().unwrap())]
    #[regex(r#"l'[^']'"#, |lex| lex.slice().chars().next().unwrap())]
    CharLiteral(char),


    // Number literals

    #[regex(r#"[0-9]+"#, |lex| lex.slice().parse())]
    InteLiteral(i64),
    #[regex(r#"[0-9]+u"#, |lex| lex.slice().trim_end_matches('u').parse())]
    #[regex(r#"0x[0-9A-F]+"#, |lex| u64::from_str_radix(lex.slice().trim_start_matches("0x"), 16))]
    #[regex(r#"0b[01]+"#, |lex| u64::from_str_radix(lex.slice().trim_start_matches("0b"), 2))]
    WordLiteral(u64),
    #[regex(r"[0-9]+(\.[0-9]*([eE][+-]?[0-9]+)?|[eE][+-]?[0-9]+)", |lex| lex.slice().parse())]
    FloatLiteral(f64),


    // Boolean literals

    #[regex(r#"true|false"#, |lex| "true" == lex.slice())]
    BoolLiteral(bool),


    #[regex(r#"//.*"#)]
    #[regex(r#"/\*([^*]*\*+)((([^*/]?)|([^*/][^*]*))\*+)*/"#)]
    Comment,

    /*

        ERROR SECTION
    
    */

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}