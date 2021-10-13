pub mod logos_lexer;
pub mod custom_lexer_struct;

use self::{logos_lexer::LexerToken, custom_lexer_struct::CustomLexerStruct};

use logos::Logos;

pub fn get_custom_lexer_from_string(
    str_to_lex: &str
) -> CustomLexerStruct<LexerToken> {
    LexerToken::lexer(str_to_lex).into()
}