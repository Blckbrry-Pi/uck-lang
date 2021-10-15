pub mod ast;

pub mod patterns;
pub mod top_level;

pub mod parse_error;

use ast::TopLevelAstNode;
use super::lexer::{ custom_lexer_struct::CustomLexerStruct, logos_lexer::LexerToken };
use parse_error::ParseError;

pub fn get_ast_from_custom_lexer<'a: 'b, 'b>(
    lxr: &'b mut CustomLexerStruct<'a, LexerToken<'a>>, 
) -> Result<TopLevelAstNode<'a>, ParseError<'a>> {
    top_level::parse_top_level(lxr)
}
