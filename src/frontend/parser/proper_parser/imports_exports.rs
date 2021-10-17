use super::super::lexer::logos_lexer::LexerToken;

use super::patterns::parse_destructuring_pattern;
use super::top_level::parse_top_level;

use super::utility_things::{flush_comments, LexerStruct, TopLevelAstResult};

use super::ast::imports_exports::{AstModuleLocation, ImportStatementAstNode};
use super::ast::top_level::TopLevelAstNode;
use super::parse_error::ParseError;

pub fn parse_import_statement<'a: 'b, 'b>(
    lxr: &'b mut LexerStruct<'a>,
) -> Result<ImportStatementAstNode<'a>, ParseError<'a>> {
    flush_comments(lxr);

    match parse_destructuring_pattern(lxr) {
        Ok(destructuring_pattern) => {
            flush_comments(lxr);

            let module_location = if let Some(LexerToken::From) = lxr.next() {
                let mut module_path: AstModuleLocation;

                flush_comments(lxr);

                if let Some(LexerToken::Identifier(name)) = lxr.next() {
                    module_path = AstModuleLocation::Root(lxr.span().unwrap(), name);
                } else {
                    const EXPECTED_ARR: &[&str] = &["identifier (as part of module path)"];

                    return Err(if let Some(span) = lxr.span() {
                        ParseError::unexpected_token_error(
                            lxr.slice().unwrap(),
                            span,
                            EXPECTED_ARR,
                            true,
                        )
                    } else {
                        ParseError::end_of_file_error(EXPECTED_ARR, true)
                    });
                }

                loop {
                    flush_comments(lxr);

                    if let Some(LexerToken::MemberAccess) = lxr.peek() {
                        lxr.next();
                    } else {
                        break;
                    }

                    flush_comments(lxr);

                    if let Some(LexerToken::Identifier(next_ident)) = lxr.next() {
                        let new_span = module_path.get_span().start..lxr.span().unwrap().end;

                        module_path =
                            AstModuleLocation::MemberOf(new_span, Box::new(module_path), next_ident)
                    } else {
                        const EXPECTED_ARR: &[&str] = &["identifier (as part of module path)"];

                        return Err(if let Some(span) = lxr.span() {
                            ParseError::unexpected_token_error(
                                lxr.slice().unwrap(),
                                span,
                                EXPECTED_ARR,
                                true,
                            )
                        } else {
                            ParseError::end_of_file_error(EXPECTED_ARR, true)
                        });
                    }
                }

                module_path
            } else {
                const EXPECTED_ARR: &[&str] = &["`from`"];
                return if let Some(span) = lxr.span() {
                    Err(ParseError::unexpected_token_error(
                        lxr.slice().unwrap(),
                        span,
                        EXPECTED_ARR,
                        true,
                    ))
                } else {
                    Err(ParseError::end_of_file_error(EXPECTED_ARR, true))
                };
            };
            let saved_span = lxr.span().unwrap();
            if let Some(LexerToken::Semicolon) = lxr.next() {
                Ok(ImportStatementAstNode {
                    span: destructuring_pattern.get_span().start..lxr.span().unwrap().end,
                    destructuring_pattern,
                    module_location,
                })
            } else {
                const EXPECTED_ARR: &[&str] = &["`;`"];
                if let Some(slice) = lxr.slice() {
                    Err(ParseError::unexpected_token_error(
                        slice,
                        saved_span,
                        EXPECTED_ARR,
                        true,
                    ))
                } else {
                    Err(ParseError::end_of_file_error(EXPECTED_ARR, true))
                }
            }
        }
        Err(mut parse_err) => {
            parse_err.fatal = true;
            Err(parse_err)
        }
    }
}

pub fn parse_export_statement<'a: 'b, 'b>(lxr: &'b mut LexerStruct<'a>) -> TopLevelAstResult<'a> {
    let start = lxr.span().unwrap().start;

    flush_comments(lxr);

    if let Some(LexerToken::Default) = lxr.peek() {
        lxr.next();
        parse_top_level(lxr)
            .map(|top_level_statement| {
                TopLevelAstNode::ExportDefault(
                    start..top_level_statement.get_span().end,
                    Box::new(top_level_statement),
                )
            })
            .map_err(|mut error| {
                error.fatal = true;
                error
            })
    } else {
        parse_top_level(lxr)
            .map(|top_level_statement| {
                TopLevelAstNode::Export(
                    start..top_level_statement.get_span().end,
                    Box::new(top_level_statement),
                )
            })
            .map_err(|mut error| {
                error.fatal = true;
                error
            })
    }
}
