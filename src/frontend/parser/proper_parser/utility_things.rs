use super::super::lexer::{custom_lexer_struct::CustomLexerStruct, logos_lexer::LexerToken};

use super::{ast::TopLevelAstNode, parse_error::ParseError};

pub type TopLevelAstResult<'a> = Result<TopLevelAstNode<'a>, ParseError<'a>>;

pub type LexerStruct<'a> = CustomLexerStruct<'a, LexerToken<'a>>;

pub fn flush_comments(lxr: &mut LexerStruct) {
    while let Some(LexerToken::Comment) = lxr.peek() {
        lxr.next();
    }
}
