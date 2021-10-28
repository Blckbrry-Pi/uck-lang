pub mod ast;

pub mod enums;
pub mod expressions;
pub mod extends_implements;
pub mod fields;
pub mod imports_exports;
pub mod methods;
pub mod patterns;
pub mod publicity;
pub mod structs;
pub mod top_level;
pub mod types;

pub mod parse_error;

pub mod utility_things;

use utility_things::{LexerStruct, TopLevelAstResult};

pub fn get_ast_from_custom_lexer<'a>(lxr: &mut LexerStruct<'a>) -> TopLevelAstResult<'a> {
    top_level::parse_top_level(lxr)
}
