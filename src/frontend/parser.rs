pub mod lexer;
pub mod proper_parser;

use proper_parser::utility_things::TopLevelAstResult;

pub fn parse_str(str_to_parse: &str) -> TopLevelAstResult {
    // Set up lexer
    let mut lxr = lexer::get_custom_lexer_from_string(str_to_parse);

    // Parse
    proper_parser::get_ast_from_custom_lexer(&mut lxr)
}
