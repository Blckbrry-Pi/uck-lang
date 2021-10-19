use std::borrow::Cow;
use std::collections::HashMap;

use super::ast::types::{AstType, Generics, TypeAliasAstNode};
use super::parse_error::ParseError;
use super::utility_things::{
    call_error, expect_semicolon, expect_token, flush_comments, LexerStruct,
};

use super::super::lexer::logos_lexer::LexerToken;

pub fn parse_generics<'a: 'b, 'b>(
    lxr: &'b mut LexerStruct<'a>,
    allow_constraints: bool,
) -> Result<Generics<'a>, ParseError<'a>> {
    let starting_span = lxr.span().unwrap();

    let mut generics = HashMap::new();
    loop {
        flush_comments(lxr);

        let generic_name: Cow<'a, str> = if allow_constraints {
            match lxr.next() {
                Some(LexerToken::RightAngleBracketOrGreaterThan) => break,
                Some(LexerToken::Identifier(name)) => name,
                invalid_value => {
                    return Err(call_error(
                        lxr,
                        invalid_value,
                        &[
                            "`>` (to close the generics)",
                            "identifier (to create a new generic)",
                        ],
                        true,
                    ))
                }
            }
            .into()
        } else {
            generics.len().to_string().into()
        };

        let span = lxr.span().unwrap();
        let slice = lxr.slice().unwrap();

        const EXPECTED_ARR_2: &[&str] = &["a unique generic name"];

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
                Some(LexerToken::ThinArrow) => (),
                invalid_value => {
                    return Err(call_error(
                        lxr,
                        invalid_value,
                        &[
                            "`>` (to close the generics)",
                            "`,` (to signal the next generic)",
                            "`:` (to define a constraint on the current generic)",
                        ],
                        true,
                    ))
                }
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

        flush_comments(lxr);

        match lxr.next() {
            Some(LexerToken::Comma) => (),
            Some(LexerToken::RightAngleBracketOrGreaterThan) => break,
            invalid_value => {
                return Err(call_error(
                    lxr,
                    invalid_value,
                    &[
                        "`>` (to close the generics)",
                        "`,` (to signal the next generic)",
                    ],
                    true,
                ))
            }
        }
    }

    Ok((starting_span.start..lxr.span().unwrap().end, generics))
}

pub fn parse_type<'a: 'b, 'b>(
    lxr: &'b mut LexerStruct<'a>,
    curr_type: Option<AstType<'a>>,
) -> Result<AstType<'a>, ParseError<'a>> {
    expect_token(
        lxr,
        LexerToken::Identifier(""),
        &["identifier (as part of a type)"],
    )?;
    let name = lxr.slice().unwrap();

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
}

pub fn parse_name_and_generics<'a: 'b, 'b>(
    lxr: &'b mut LexerStruct<'a>,
) -> Result<AstType<'a>, ParseError<'a>> {
    flush_comments(lxr);

    expect_token(
        lxr,
        LexerToken::Identifier(""),
        &["identifier (as part of a type)"],
    )?;
    let name = lxr.slice().unwrap();

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

pub fn parse_type_alias<'a: 'b, 'b>(
    lxr: &'b mut LexerStruct<'a>,
) -> Result<TypeAliasAstNode<'a>, ParseError<'a>> {
    let start_idx = lxr.span().unwrap_or(usize::MAX..usize::MAX).start;

    let aliased_type = parse_name_and_generics(lxr)?;

    expect_token(
        lxr,
        LexerToken::Assign,
        &["`=` (to seperate the type alias and the type it refers to)"],
    )?;

    let orig_type = parse_type(lxr, None).map_err(|mut err| {
        err.fatal = true;
        err
    })?;

    expect_semicolon(lxr)?;

    Ok(TypeAliasAstNode {
        span: start_idx..lxr.span().unwrap().end,
        aliased_type,
        orig_type,
    })
}
