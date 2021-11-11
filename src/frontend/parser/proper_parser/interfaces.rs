use super::ast::interfaces::InterfaceDecAstNode;
use super::extends_implements::parse_extends;
use super::parse_error::ParseError;
use super::types::parse_name_and_generics;
use super::utility_things::{LexerStruct};

pub fn parse_interface<'a>(lxr: &mut LexerStruct<'a>) -> Result<InterfaceDecAstNode<'a>, ParseError<'a>> {
    let start_span = lxr.span().unwrap().start;

    let interface_type = parse_name_and_generics(lxr)?;
    let extends = parse_extends(lxr)?;
    
    // TODO: Parse interface body

    Ok(InterfaceDecAstNode {
        span: start_span..lxr.span().unwrap().end,
        interface_type,
        extends,
        methods: Vec::new(),
    })
}