use super::super::lexer::logos_lexer::LexerToken;

use super::ast::types::AstType;
use super::parse_error::ParseError;
use super::types::parse_type;
use super::utility_things::{expect_token, LexerStruct};

pub fn parse_implements<'a>(
    lxr: &mut LexerStruct<'a>,
) -> Result<Option<AstType<'a>>, ParseError<'a>> {
    let position = lxr.save_position();
    if expect_token(lxr, LexerToken::Implements, &[]).is_err() {
        lxr.return_to_position(position);
        return Ok(None);
    }

    parse_type(lxr, None).map(Some)
}
