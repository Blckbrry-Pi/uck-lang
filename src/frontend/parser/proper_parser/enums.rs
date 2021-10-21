use super::ast::enums::{EnumCaseAstNode, EnumDecAstNode};

use super::methods::parse_methods_until_none_are_found;
use super::parse_error::ParseError;
use super::types::{parse_name_and_generics, parse_type};
use super::utility_things::{call_error, expect_token, flush_comments, LexerStruct};

use super::super::lexer::logos_lexer::LexerToken;

pub fn parse_enum_case<'a>(
    lxr: &mut LexerStruct<'a>,
) -> Result<EnumCaseAstNode<'a>, ParseError<'a>> {
    flush_comments(lxr);

    expect_token(
        lxr,
        LexerToken::Identifier(""),
        &["identifier (to name the enum case)"],
    )?;
    let case_name = lxr.slice().unwrap();
    let start_idx = lxr.span().unwrap().start;

    flush_comments(lxr);

    let case_args = if let Some(LexerToken::LeftParenthesis) = lxr.peek() {
        expect_token(
            lxr,
            LexerToken::LeftParenthesis,
            &["`(` (to begin description of the fields of the enum case)"],
        )?;

        let mut case_args = Vec::new();

        const CLOSING_PAREN_STRING: &str =
            "`)` (to end the description of the fields of the enum case)";

        loop {
            match lxr.peek() {
                Some(LexerToken::Identifier(_)) => case_args.push(parse_type(lxr, None)?),
                Some(LexerToken::RightParenthesis) => break,
                invalid_value => {
                    lxr.next();
                    return Err(call_error(
                        lxr,
                        invalid_value,
                        &["identifier (to start a type)", CLOSING_PAREN_STRING],
                        true,
                    ));
                }
            }

            match lxr.peek() {
                Some(LexerToken::Comma) => {
                    lxr.next().unwrap();
                }
                Some(LexerToken::RightParenthesis) => break,
                invalid_value => {
                    lxr.next().unwrap();
                    return Err(call_error(
                        lxr,
                        invalid_value,
                        &["`,` (to signal the next type)", CLOSING_PAREN_STRING],
                        true,
                    ));
                }
            }
        }

        expect_token(lxr, LexerToken::RightParenthesis, &[CLOSING_PAREN_STRING])?;

        case_args
    } else {
        Vec::new()
    };

    Ok(EnumCaseAstNode {
        span: start_idx..lxr.span().unwrap().end,
        case_name,
        case_args,
    })
}

pub fn parse_enum_dec<'a>(lxr: &mut LexerStruct<'a>) -> Result<EnumDecAstNode<'a>, ParseError<'a>> {
    let start_idx = lxr.span().unwrap().start;

    flush_comments(lxr);

    let enum_type = parse_name_and_generics(lxr)?;

    flush_comments(lxr);

    let implements = match lxr.next() {
        Some(LexerToken::Implements) => {
            let implement_constraint = parse_type(lxr, None)?;

            expect_token(
                lxr,
                LexerToken::LeftCurlyBrace,
                &["`{` (to begin the body of the enum)"],
            )?;

            Some(implement_constraint)
        }
        Some(LexerToken::LeftCurlyBrace) => None,
        invalid_value => {
            return Err(call_error(
                lxr,
                invalid_value,
                &[
                    "`implements` (to signal which traits the enum implements)",
                    "`{` (to begin the body of the enum)",
                ],
                true,
            ))
        }
    };

    let mut cases = Vec::new();

    loop {
        flush_comments(lxr);

        match lxr.peek() {
            Some(
                LexerToken::Public
                | LexerToken::Private
                | LexerToken::LeftAngleBracketOrLessThan
                | LexerToken::RightCurlyBrace,
            ) => break,
            Some(LexerToken::Identifier(_)) => cases.push(parse_enum_case(lxr)?),
            invalid_value => {
                return Err(call_error(
                    lxr,
                    invalid_value,
                    &[
                        "identifier (to define a new enum case)",
                        "`pub` or `priv` (to define a new method and its publicity)",
                        "`<` (to start a new constraint block)",
                    ],
                    true,
                ))
            }
        }

        flush_comments(lxr);

        match lxr.peek() {
            Some(LexerToken::Comma) => {
                lxr.next().unwrap();
            }
            Some(LexerToken::RightCurlyBrace) => break,
            invalid_value => {
                return Err(call_error(
                    lxr,
                    invalid_value,
                    &[
                        "`,` (to signal the next case or first method of the enum)",
                        "`}` (to close the struct)",
                    ],
                    true,
                ))
            }
        }
    }

    let methods_tuple = parse_methods_until_none_are_found(lxr)?;
    if methods_tuple.1.fatal {
        Err(methods_tuple.1)
    } else {
        let methods = methods_tuple.0;
        Ok(EnumDecAstNode {
            span: start_idx..lxr.span().unwrap().end,
            enum_type,
            implements,
            cases,
            methods,
        })
    }
}
