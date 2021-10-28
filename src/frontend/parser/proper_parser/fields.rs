use super::super::lexer::logos_lexer::LexerToken;

use super::ast::fields::{FieldAstNode, FieldList};
use super::publicity::ParsePublicity;
use super::types::parse_type;

use super::parse_error::ParseError;
use super::utility_things::{expect_token, flush_comments, LexerStruct};

pub fn parse_fields_until_none_are_left<'a, PublicityEnum: ParsePublicity>(
    lxr: &mut LexerStruct<'a>,
) -> Result<(FieldList<'a, PublicityEnum>, ParseError<'a>), ParseError<'a>> {
    let mut fields = Vec::new();

    let first_error = loop {
        let saved_position = lxr.save_position();

        flush_comments(lxr);
        match parse_field(lxr) {
            Ok(field) => fields.push(field),
            Err(err) if !err.fatal => {
                lxr.return_to_position(saved_position);
                break err;
            }
            Err(err) => return Err(err),
        }

        let saved_position = lxr.save_position();

        flush_comments(lxr);
        match expect_token(
            lxr,
            LexerToken::Comma,
            &["`,` (to begin to declare the next field)"],
        ) {
            Ok(_) => (),
            Err(e) => {
                lxr.return_to_position(saved_position);
                break ParseError { fatal: false, ..e };
            }
        }
    };

    Ok((fields, first_error))
}

pub fn parse_field<'a, PublicityEnum: ParsePublicity>(
    lxr: &mut LexerStruct<'a>,
) -> Result<FieldAstNode<'a, PublicityEnum>, ParseError<'a>> {
    flush_comments(lxr);

    let publicity = PublicityEnum::parse_publicity(lxr)?;
    lxr.next().unwrap();

    let start_idx = lxr.span().unwrap().start;

    flush_comments(lxr);

    expect_token(
        lxr,
        LexerToken::Identifier(""),
        &["identifier (to set the name of the field)"],
    )
    .map_err(|err| ParseError {
        fatal: false,
        ..err
    })?;
    let name = lxr.slice().unwrap();

    flush_comments(lxr);

    expect_token(
        lxr,
        LexerToken::ThinArrow,
        &["`->` (to denote the type of the field)"],
    )
    .map_err(|err| ParseError {
        fatal: false,
        ..err
    })?;

    flush_comments(lxr);

    let field_type = parse_type(lxr, None)?;

    Ok(FieldAstNode {
        span: start_idx..lxr.span().unwrap().end,
        publicity,
        name,
        field_type,
    })
}
