pub mod lexer;
pub mod proper_parser;


use proper_parser::{ ast::TopLevelAstNode, parse_error::ParseError };

pub fn parse_str<'a: 'a>(str_to_parse: &'a str) -> Result<TopLevelAstNode<'a>, ParseError<'a>> {
    // Set up lexer
    let mut lxr = lexer::get_custom_lexer_from_string(str_to_parse);


    // Parse
    proper_parser::get_ast_from_custom_lexer(&mut lxr)
}
