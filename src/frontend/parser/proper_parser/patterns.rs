use super::super::lexer::logos_lexer::LexerToken;

use super::ast::patterns::AstDestructuringPattern;
use super::parse_error::ParseError;
use super::utility_things::{call_error, expect_token, flush_comments, LexerStruct};

type AstDestructuringPatternResult<'a> = Result<AstDestructuringPattern<'a>, ParseError<'a>>;

pub fn parse_destructuring_pattern<'a>(
    lxr: &mut LexerStruct<'a>,
) -> AstDestructuringPatternResult<'a> {
    flush_comments(lxr);

    expect_token(
        lxr,
        LexerToken::Identifier(""),
        &["identifier (as part of a destructuring pattern)"],
    )?;
    let base_span = lxr.span().unwrap();
    let name = lxr.slice().unwrap();

    flush_comments(lxr);

    // If this matches, it has parsed... `[name]: `
    if let Some(LexerToken::Colon) = lxr.peek() {
        lxr.next();

        const EXPECTED_ARR: &[&str] = &["identifier or `{` (as part of a destructuring pattern)"];

        flush_comments(lxr);

        match lxr.next() {
            // If this matches, it has parsed... `[name]: [aliased_name]`
            Some(LexerToken::Identifier(aliased_name)) => Ok(AstDestructuringPattern::AliasedName(
                base_span.start..lxr.span().unwrap().end,
                name,
                aliased_name,
            )),

            // If this matches, it has parsed... `[name]: {`
            Some(LexerToken::LeftCurlyBrace) => {
                let mut child_destructuring_patterns = Vec::new();
                loop {
                    flush_comments(lxr);

                    match lxr.peek() {
                        Some(LexerToken::Identifier(_)) => {
                            child_destructuring_patterns.push(parse_destructuring_pattern(lxr)?)
                        }
                        Some(LexerToken::RightCurlyBrace) => {
                            lxr.next();
                            break;
                        }
                        invalid_value => {
                            lxr.next();
                            return Err(call_error(
                                lxr,
                                invalid_value,
                                &["identifier or `}` (as part of a destructuring pattern)"],
                                true,
                            ));
                        }
                    }

                    flush_comments(lxr);

                    match lxr.next() {
                        Some(LexerToken::Comma) => (),
                        Some(LexerToken::RightCurlyBrace) => break,
                        invalid_value => {
                            return Err(call_error(
                                lxr,
                                invalid_value,
                                &["`,` or `}` (as part of a destructuring pattern)"],
                                true,
                            ))
                        }
                    }
                }

                Ok(AstDestructuringPattern::Destructured(
                    base_span.start..lxr.span().unwrap().end,
                    name,
                    child_destructuring_patterns,
                ))
            }

            // If this matches the end of the file matches `[name]: [EOF]`
            Some(_) => {
                let span = lxr.span().unwrap();
                Err(ParseError::unexpected_token_error(
                    lxr.slice().unwrap(),
                    span,
                    EXPECTED_ARR,
                    true,
                ))
            }

            // If this matches the end of the file matches `[name]: [EOF]`
            None => Err(ParseError::end_of_file_error(
                &["identifier or `{` (as part of a destructuring pattern)"],
                true,
            )),
        }
    }
    // Otherwise, this matches
    else {
        Ok(AstDestructuringPattern::Name(base_span, &name))
    }
}
