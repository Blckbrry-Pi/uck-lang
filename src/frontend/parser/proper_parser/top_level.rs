use super::super::lexer::{ custom_lexer_struct::CustomLexerStruct, logos_lexer::LexerToken };

use super::patterns::parse_destructuring_pattern;

use super::ast::{ TopLevelAstNode, AstModuleLocation };
use super::parse_error::ParseError;

type LexerStruct<'a> = CustomLexerStruct<'a, LexerToken<'a>>;

type TopLevelAstResult<'a> = Result<TopLevelAstNode<'a>, ParseError<'a>>;

pub fn parse_top_level<'a: 'b, 'b>(
    lxr: &'b mut LexerStruct<'a>,
) -> TopLevelAstResult<'a> {
    const EXPECTED_ARR: &'static [&'static str] = &[
        "Import statement",
        "Enum Declaration",
        "Struct Declaration",
        "Class Declaration",
        "Type Alias",
    ];

    match lxr.next() {
        Some(LexerToken::Import) => parse_import_statement(lxr),
        Some(LexerToken::Export) => {
            let start = lxr.span().unwrap().start;
            if let Some(LexerToken::Default) = lxr.peek() {
                lxr.next();
                parse_import_statement(lxr)
                    .map(
                        |top_level_statement| TopLevelAstNode::ExportDefault(
                            start..top_level_statement.get_span().end,
                            Box::new(top_level_statement),
                        )
                    )
                    .map_err(
                        |mut error| { error.fatal = true; error } 
                    )
            } else {
                parse_import_statement(lxr)
                    .map(
                        |top_level_statement| TopLevelAstNode::Export(
                            start..top_level_statement.get_span().end,
                            Box::new(top_level_statement),
                        )
                    )
                    .map_err(
                        |mut error| { error.fatal = true; error }
                    )
            }
        },
        Some(LexerToken::Enum) => parse_enum_dec(lxr),
        Some(LexerToken::Struct) => parse_struct_dec(lxr),
        Some(LexerToken::Class) => parse_class_dec(lxr),
        Some(LexerToken::Type) => parse_type_alias(lxr),
        Some(LexerToken::Comment) => {
            let span = lxr.span().unwrap();
            parse_top_level(lxr)
                .map(
                    |top_level_statement| TopLevelAstNode::CommentedNode(
                        span.start..top_level_statement.get_span().end,
                        span,
                        Box::new(top_level_statement),
                    )
                )
        }
        Some(_) => {
            let span = lxr.span().unwrap();
            Err(
                ParseError::unexpected_token_error(
                    lxr.slice().unwrap(),
                    span,
                    EXPECTED_ARR,
                    true,
                )
            )
        },
        None => Err(
            ParseError::end_of_file_error(
                EXPECTED_ARR,
                false,
            )
        ),
    }
}

pub fn parse_import_statement<'a: 'b, 'b>(
    lxr: &'b mut LexerStruct<'a>,
) -> TopLevelAstResult<'a> {
    match parse_destructuring_pattern(lxr) {
        Ok(destructuring_pattern) => {
            let module_path = if let Some(LexerToken::From) = lxr.next() {
                let mut module_path: AstModuleLocation;
            
                if let Some(LexerToken::Identifier(name)) = lxr.next() {
                    module_path = AstModuleLocation::Root(lxr.span().unwrap(), name);
                } else {
                    const EXPECTED_ARR: &'static [&'static str] = &[
                        "identifier (as part of module path)",
                    ];

                    return Err(
                        if let Some(span) = lxr.span() {
                            ParseError::unexpected_token_error(lxr.slice().unwrap(), span, EXPECTED_ARR, true)
                        } else {
                            ParseError::end_of_file_error(EXPECTED_ARR, true)
                        }
                    )
                }

                loop {
                    if let Some(LexerToken::MemberAccess) = lxr.peek() { lxr.next(); } else { break; }

                    if let Some(LexerToken::Identifier(next_ident)) = lxr.next() {
                        let new_span = module_path.get_span().start..lxr.span().unwrap().end;

                        module_path = AstModuleLocation::MemberOf(
                            new_span,
                            Box::new(module_path),
                            next_ident,
                        )
                    } else {
                        const EXPECTED_ARR: &'static [&'static str] = &[
                            "identifier (as part of module path)",
                        ];

                        return Err(
                            if let Some(span) = lxr.span() {
                                ParseError::unexpected_token_error(lxr.slice().unwrap(), span, EXPECTED_ARR, true)
                            } else {
                                ParseError::end_of_file_error(EXPECTED_ARR, true)
                            }
                        )
                    }
                }

                module_path

            } else {
                const EXPECTED_ARR: &'static [&'static str] = &[
                    "`from`",
                ];
                return if let Some(span) = lxr.span() {
                    Err(
                        ParseError::unexpected_token_error(
                            lxr.slice().unwrap(),
                            span,
                            EXPECTED_ARR,
                            true,
                        )
                    )
                } else { Err(ParseError::end_of_file_error(EXPECTED_ARR, true)) };
            };
            let saved_span = lxr.span().unwrap();
            if let Some(LexerToken::Semicolon) = lxr.next() {
                Ok(
                    TopLevelAstNode::ImportFrom(
                        destructuring_pattern.get_span().start..lxr.span().unwrap().end,
                        destructuring_pattern,
                        module_path,
                    )
                )
            } else {
                const EXPECTED_ARR: &'static [&'static str] = &[
                    "`;`",
                ];
                if let Some(slice) = lxr.slice() {
                    Err(
                        ParseError::unexpected_token_error(slice, saved_span, EXPECTED_ARR, true)
                    )
                } else {
                    Err(
                        ParseError::end_of_file_error(EXPECTED_ARR, true)
                    )
                }
            }
        },
        Err(mut parse_err) => {
            parse_err.fatal = true;
            Err(parse_err)
        }
    }
}

pub fn parse_enum_dec<'a: 'b, 'b>(
    lxr: &'b mut LexerStruct,
) -> TopLevelAstResult<'a> {
    Ok(TopLevelAstNode::Empty)
}

pub fn parse_struct_dec<'a: 'b, 'b>(
    lxr: &'b mut LexerStruct,
) -> TopLevelAstResult<'a> {
    Ok(TopLevelAstNode::Empty)
}

pub fn parse_class_dec<'a: 'b, 'b>(
    lxr: &'b mut LexerStruct,
) -> TopLevelAstResult<'a> {
    Ok(TopLevelAstNode::Empty)
}

pub fn parse_type_alias<'a: 'b, 'b>(
    lxr: &'b mut LexerStruct,
) -> TopLevelAstResult<'a> {
    Ok(TopLevelAstNode::Empty)
}
