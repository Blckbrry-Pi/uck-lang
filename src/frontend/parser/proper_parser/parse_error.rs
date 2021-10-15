#[derive(Debug)]
pub struct ParseError<'a> {

    pub expected: &'a [&'a str],
    pub span: Option<logos::Span>,

    pub got: Option<String>,
    pub is_eof: bool,

    pub fatal: bool,
}

impl<'a> ParseError<'a> {
    pub fn end_of_file_error(
        expected: &'a [&'a str],
        is_fatal: bool,
    ) -> Self {
        ParseError {
            expected,
            span: Some(usize::MAX..usize::MAX),
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
            expected,
            span: Some(span),
            got: Some(got.to_string()),
            is_eof: false,
            fatal: is_fatal,
        }
    }
}
