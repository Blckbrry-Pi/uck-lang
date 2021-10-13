use logos::Logos;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NewTypeKeyword {
    Enum,
    Struct,
    Class,
    Type,
}

impl NewTypeKeyword {
    fn try_new(string_in: &str) -> Result<Self, ()> {
        match string_in {
            "enum"   => Ok(NewTypeKeyword::Enum),
            "struct" => Ok(NewTypeKeyword::Struct),
            "class"  => Ok(NewTypeKeyword::Class),
            "type"   => Ok(NewTypeKeyword::Type),
            _        => Err(()),
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LoopKeyword {
    For,
    While,
    Loop,
}

impl LoopKeyword {
    fn try_new(string_in: &str) -> Result<Self, ()> {
        match string_in {
            "for"   => Ok(LoopKeyword::For),
            "while" => Ok(LoopKeyword::While),
            "loop"  => Ok(LoopKeyword::Loop),
            _       => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LoopFlowKeyword {
    Break,
    Continue,
}

impl LoopFlowKeyword {
    fn try_new(string_in: &str) -> Result<Self, ()> {
        match string_in {
            "break"    => Ok(LoopFlowKeyword::Break),
            "continue" => Ok(LoopFlowKeyword::Continue),
            _          => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VarDecKeyword {
    Let,
    Const,
}

impl VarDecKeyword {
    fn try_new(string_in: &str) -> Result<Self, ()> {
        match string_in {
            "let"   => Ok(VarDecKeyword::Let),
            "const" => Ok(VarDecKeyword::Const),
            _       => Err(()),
        }
    }
}

#[derive(Clone, Debug, Logos, PartialEq)]
pub enum LexerToken {

    /*

        KEYWORDS SECTION

    */

    #[regex(r"(enum)|(struct)|(class)|(type)", |lex| NewTypeKeyword::try_new(lex.slice()))]
    Type(NewTypeKeyword),

    #[regex(r"fun")]
    Function,


    // Conditional Branching

    #[regex(r"if")]
    If,
    #[regex(r"else")]
    Else,
    #[regex(r"match")]
    Match,


    // Loops

    #[regex(r"(for)|(while)|(loop)", |lex| LoopKeyword::try_new(lex.slice()))]
    Loop(LoopKeyword),

    #[regex(r"(break)|(continue)", |lex| LoopFlowKeyword::try_new(lex.slice()))]
    LoopFlow(LoopFlowKeyword),


    // Variables

    #[regex(r"(let)|(const)", |lex| VarDecKeyword::try_new(lex.slice()))]
    VariableDeclaration(VarDecKeyword),

    #[regex(r"mut")]
    Mutable,




    /*
    
        IDENTIFIERS SECTION (with literally just identifiers)

    */

    #[regex(r"[\p{L}_][\p{L}\p{N}_]*", |lex| lex.slice().to_string())]
    Identifier(String),






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




    /*

        ERROR SECTION
    
    */

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}