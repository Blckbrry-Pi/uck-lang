use std::borrow::Cow;

#[derive(Debug)]
pub struct ParseError<'a> {
    pub expected: Cow<'a, [&'a str]>,
    pub span: logos::Span,

    pub got: Option<String>,
    pub is_eof: bool,

    pub fatal: bool,
}

impl<'a> ParseError<'a> {
    pub fn end_of_file_error(expected: &'a [&'a str], is_fatal: bool) -> Self {
        ParseError {
            expected: expected.into(),
            span: usize::MAX..usize::MAX,
            got: None,
            is_eof: true,
            fatal: is_fatal,
        }
    }

    pub fn unexpected_token_error(
        got: &str,
        span: logos::Span,
        expected: &'a [&'a str],
        is_fatal: bool,
    ) -> Self {
        ParseError {
            expected: expected.into(),
            span,
            got: Some(got.to_string()),
            is_eof: false,
            fatal: is_fatal,
        }
    }
}

pub fn combine_parse_errors<'a>(err_1: ParseError<'a>, err_2: ParseError<'a>) -> ParseError<'a> {
    ParseError {
        expected: err_1
            .expected
            .iter()
            .chain(err_2.expected.iter())
            .copied()
            .collect::<Vec<&str>>()
            .into(),
        span: if err_1.is_eof || err_2.is_eof {
            usize::MAX..usize::MAX
        } else {
            usize::min(err_1.span.start, err_2.span.start)
                ..usize::min(err_1.span.end, err_2.span.end)
        },
        got: err_1.got,
        is_eof: err_1.is_eof || err_2.is_eof,
        fatal: err_1.fatal || err_2.fatal,
    }
}
