use std::borrow::Cow;
use std::collections::HashMap;

use super::ast::{AstType, Generics, TopLevelAstNode};
use super::parse_error::ParseError;
use super::utility_things::{flush_comments, LexerStruct, TopLevelAstResult};

use super::super::lexer::logos_lexer::LexerToken;

pub fn parse_generics<'a: 'b, 'b>(
    lxr: &'b mut LexerStruct<'a>,
    allow_constraints: bool,
) -> Result<Generics<'a>, ParseError<'a>> {
    let starting_span = lxr.span().unwrap();

    let mut generics = HashMap::new();
    loop {
        const EXPECTED_ARR: &[&str] = &[
            "`>` (to close the generics)",
            "identifier (to create a new generic)",
        ];

        flush_comments(lxr);

        let generic_name: Cow<'a, str> = if allow_constraints {
            match lxr.next() {
                Some(LexerToken::RightAngleBracketOrGreaterThan) => break,
                Some(LexerToken::Identifier(name)) => name,
                Some(_) => {
                    return Err(ParseError::unexpected_token_error(
                        lxr.slice().unwrap(),
                        lxr.span().unwrap(),
                        EXPECTED_ARR,
                        true,
                    ))
                }
                None => return Err(ParseError::end_of_file_error(EXPECTED_ARR, true)),
            }
            .into()
        } else {
            generics.len().to_string().into()
        };

        let span = lxr.span().unwrap();
        let slice = lxr.slice().unwrap();

        const EXPECTED_ARR_2: &[&str] = &["a unique generic name"];

        const EXPECTED_ARR_3: &[&str] = &[
            "`>` (to close the generics)",
            "`,` (to signal the next generic)",
            "`:` (to define a constraint on the current generic)",
        ];

        flush_comments(lxr);

        if allow_constraints {
            match lxr.next() {
                Some(LexerToken::Comma) => {
                    if generics
                        .insert(generic_name, (span.clone(), None))
                        .is_some()
                    {
                        return Err(ParseError::unexpected_token_error(
                            slice,
                            span,
                            EXPECTED_ARR_2,
                            true,
                        ));
                    }
                    continue;
                }
                Some(LexerToken::RightAngleBracketOrGreaterThan) => {
                    if generics
                        .insert(generic_name, (span.clone(), None))
                        .is_some()
                    {
                        return Err(ParseError::unexpected_token_error(
                            slice,
                            span,
                            EXPECTED_ARR_2,
                            true,
                        ));
                    }
                    break;
                }
                Some(LexerToken::Colon) => (),
                Some(_) => {
                    return Err(ParseError::unexpected_token_error(
                        lxr.slice().unwrap(),
                        lxr.span().unwrap(),
                        EXPECTED_ARR_3,
                        true,
                    ))
                }
                None => return Err(ParseError::end_of_file_error(EXPECTED_ARR_3, true)),
            }
        }

        flush_comments(lxr);

        let generic_type = parse_type(lxr, None).map_err(|mut err| {
            err.fatal = true;
            err
        })?;

        let result_of_insert = generics.insert(
            generic_name,
            (
                if allow_constraints {
                    span.start..generic_type.get_span().end
                } else {
                    generic_type.get_span()
                },
                Some(generic_type),
            ),
        );

        if result_of_insert.is_some() {
            return Err(ParseError::unexpected_token_error(
                slice,
                span,
                EXPECTED_ARR_2,
                true,
            ));
        }

        const EXPECTED_ARR_4: &[&str] = &[
            "`>` (to close the generics)",
            "`,` (to signal the next generic)",
        ];

        flush_comments(lxr);

        match lxr.next() {
            Some(LexerToken::Comma) => (),
            Some(LexerToken::RightAngleBracketOrGreaterThan) => break,
            Some(_) => {
                return Err(ParseError::unexpected_token_error(
                    lxr.slice().unwrap(),
                    lxr.span().unwrap(),
                    EXPECTED_ARR_4,
                    true,
                ))
            }
            None => return Err(ParseError::end_of_file_error(EXPECTED_ARR_4, true)),
        }
    }

    Ok((starting_span.start..lxr.span().unwrap().end, generics))
}

pub fn parse_type<'a: 'b, 'b>(
    lxr: &'b mut LexerStruct<'a>,
    curr_type: Option<AstType<'a>>,
) -> Result<AstType<'a>, ParseError<'a>> {
    if let Some(LexerToken::Identifier(name)) = lxr.next() {
        let mut curr_type = match curr_type {
            Some(starting_type) => AstType::MemberOf(
                starting_type.get_span().start..lxr.span().unwrap().end,
                Box::new(starting_type),
                name,
            ),
            None => AstType::RootName(lxr.span().unwrap(), name),
        };

        match lxr.peek() {
            Some(LexerToken::MemberAccess) => {
                lxr.next();
                match parse_type(lxr, Some(curr_type)) {
                    Ok(parsed_type) => curr_type = parsed_type,
                    Err(error) => return Err(error),
                }
            }
            Some(LexerToken::LeftAngleBracketOrLessThan) => {
                lxr.next();
                match parse_generics(lxr, false) {
                    Ok(parsed_generic) => {
                        curr_type = AstType::GenericOf(
                            curr_type.get_span().start..parsed_generic.0.end,
                            Box::new(curr_type),
                            parsed_generic,
                        )
                    }
                    Err(error) => return Err(error),
                }
                match lxr.peek() {
                    Some(LexerToken::MemberAccess) => {
                        lxr.next();
                        match parse_type(lxr, Some(curr_type)) {
                            Ok(parsed_type) => curr_type = parsed_type,
                            Err(error) => return Err(error),
                        }
                    }
                    Some(LexerToken::LeftAngleBracketOrLessThan) => {
                        return Err(ParseError::unexpected_token_error(
                            lxr.slice().unwrap(),
                            lxr.span().unwrap(),
                            &["`.` (to continue type)", "end of type"],
                            true,
                        ))
                    }
                    _ => (),
                }
            }
            _ => (),
        }
        Ok(curr_type)
    } else {
        const EXPECTED_ARR: &[&str] = &["identifier (as part of a type)"];

        Err(if let Some(slice) = lxr.slice() {
            ParseError::unexpected_token_error(slice, lxr.span().unwrap(), EXPECTED_ARR, false)
        } else {
            ParseError::end_of_file_error(EXPECTED_ARR, false)
        })
    }
}

pub fn parse_name_and_generics<'a: 'b, 'b>(
    lxr: &'b mut LexerStruct<'a>,
) -> Result<AstType<'a>, ParseError<'a>> {
    flush_comments(lxr);

    let name = if let Some(LexerToken::Identifier(name)) = lxr.next() {
        name
    } else {
        const EXPECTED_ARR: &[&str] = &["identifier (as part of a type)"];

        return Err(if let Some(slice) = lxr.slice() {
            ParseError::unexpected_token_error(slice, lxr.span().unwrap(), EXPECTED_ARR, true)
        } else {
            ParseError::end_of_file_error(EXPECTED_ARR, true)
        });
    };
    let base_span = lxr.span().unwrap();
    let base_name = AstType::RootName(base_span, name);

    flush_comments(lxr);

    let generic_declarations = if let Some(LexerToken::LeftAngleBracketOrLessThan) = lxr.peek() {
        lxr.next();
        parse_generics(lxr, true)?
    } else {
        let full_span = lxr.span().unwrap_or(usize::MAX..usize::MAX);
        (full_span.start..full_span.start, HashMap::new())
    };

    Ok(AstType::GenericOf(
        base_name.get_span().start..lxr.span().unwrap().end,
        Box::new(base_name),
        generic_declarations,
    ))
}

pub fn parse_type_alias<'a: 'b, 'b>(lxr: &'b mut LexerStruct<'a>) -> TopLevelAstResult<'a> {
    let start_idx = lxr.span().unwrap_or(usize::MAX..usize::MAX).start;

    let aliased_type = parse_name_and_generics(lxr)?;

    if let Some(LexerToken::Assign) = lxr.next() {
    } else {
        const EXPECTED_ARR: &[&str] =
            &["`=` (to seperate the type alias and the type it refers to)"];

        return Err(if let Some(slice) = lxr.slice() {
            ParseError::unexpected_token_error(slice, lxr.span().unwrap(), EXPECTED_ARR, true)
        } else {
            ParseError::end_of_file_error(EXPECTED_ARR, true)
        });
    }

    let unaliased_type = parse_type(lxr, None).map_err(|mut err| {
        err.fatal = true;
        err
    })?;

    if let Some(LexerToken::Semicolon) = lxr.next() {
    } else {
        const EXPECTED_ARR: &[&str] = &["`;` (to signal the end of the expression)"];

        return Err(if let Some(slice) = lxr.slice() {
            ParseError::unexpected_token_error(slice, lxr.span().unwrap(), EXPECTED_ARR, true)
        } else {
            ParseError::end_of_file_error(EXPECTED_ARR, true)
        });
    }

    Ok(TopLevelAstNode::TypeAlias(
        start_idx..lxr.span().unwrap().end,
        aliased_type,
        unaliased_type,
    ))
}
