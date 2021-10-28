use std::mem;

use super::super::lexer::{custom_lexer_struct::CustomLexerStruct, logos_lexer::LexerToken};

use super::ast::top_level::TopLevelAstNode;
use super::parse_error::ParseError;

pub type TopLevelAstResult<'a> = Result<TopLevelAstNode<'a>, ParseError<'a>>;

pub type LexerStruct<'a> = CustomLexerStruct<'a, LexerToken<'a>>;

pub fn flush_comments(lxr: &mut LexerStruct) {
    while let Some(LexerToken::Comment) = lxr.peek() {
        lxr.next();
    }
}

pub fn expect_token<'a>(
    lxr: &mut LexerStruct<'a>,
    expected_token: LexerToken,
    expected_arr: &'a [&'a str],
) -> Result<LexerToken<'a>, ParseError<'a>> {
    expect_token_with_optional_span(lxr, expected_token, expected_arr, None)
}

pub fn expect_semicolon<'a>(lxr: &mut LexerStruct<'a>) -> Result<LexerToken<'a>, ParseError<'a>> {
    expect_token_with_optional_span(
        lxr,
        LexerToken::Semicolon,
        &["`;`"],
        Some(lxr.span().unwrap()),
    )
}

pub fn expect_token_with_optional_span<'a>(
    lxr: &mut LexerStruct<'a>,
    expected_token: LexerToken,
    expected_arr: &'a [&'a str],
    optional_span: Option<logos::Span>,
) -> Result<LexerToken<'a>, ParseError<'a>> {
    let optional_span = if optional_span.is_some() {
        optional_span
    } else {
        lxr.peek_span()
    };

    if let Some(token) = lxr.next() {
        if mem::discriminant(&token) == mem::discriminant(&expected_token) {
            Ok(token)
        } else {
            Err(ParseError::unexpected_token_error(
                lxr.slice().unwrap(),
                optional_span.unwrap(),
                expected_arr,
                true,
            ))
        }
    } else {
        Err(ParseError::end_of_file_error(expected_arr, true))
    }
}

pub fn call_error<'a>(
    lxr: &mut LexerStruct<'a>,
    token: Option<LexerToken<'a>>,
    expected_arr: &'static [&'static str],
    fatality: bool,
) -> ParseError<'a> {
    match token {
        Some(_) => ParseError::unexpected_token_error(
            lxr.slice().unwrap(),
            lxr.span().unwrap(),
            expected_arr,
            fatality,
        ),
        None => ParseError::end_of_file_error(expected_arr, fatality),
    }
}
