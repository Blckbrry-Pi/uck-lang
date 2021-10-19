use super::ast::expressions::ExpressionBlockAstNode;
use super::parse_error::ParseError;
use super::utility_things::LexerStruct;

pub fn parse_block_expr<'a>(
    _lxr: &mut LexerStruct<'a>,
) -> Result<ExpressionBlockAstNode, ParseError<'a>> {
    Ok(ExpressionBlockAstNode {})
}
