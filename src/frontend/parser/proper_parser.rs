pub mod ast;

pub mod classes;
pub mod enums;
pub mod expressions;
pub mod extends_implements;
pub mod fields;
pub mod imports_exports;
pub mod interfaces;
pub mod methods;
pub mod patterns;
pub mod publicity;
pub mod structs;
pub mod top_level;
pub mod types;

pub mod parse_error;

pub mod utility_things;

use super::lexer::{custom_lexer_struct::CustomLexerStruct, logos_lexer::LexerToken};
use {ast::top_level::TopLevelAstNode, parse_error::ParseError};


pub type TopLevelAstNodeListResult<'a> = Result<Vec<TopLevelAstNode<'a>>, ParseError<'a>>;

pub fn get_ast_from_custom_lexer<'a>(lxr: &mut CustomLexerStruct<'a, LexerToken<'a>>) -> TopLevelAstNodeListResult<'a> {
    let mut statements = vec![];

    loop {
        match top_level::parse_top_level(lxr) {
            Ok(node) => statements.push(node),
            Err(err) if err.fatal => return Err(err),
            _ => break,
        }
    }

    Ok(statements)
}
