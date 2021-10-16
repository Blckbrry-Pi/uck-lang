pub mod custom_lexer_struct;
pub mod logos_lexer;

use self::{custom_lexer_struct::CustomLexerStruct, logos_lexer::LexerToken};

use logos::Logos;

pub fn get_custom_lexer_from_string(str_to_lex: &str) -> CustomLexerStruct<LexerToken> {
    LexerToken::lexer(str_to_lex).into()
}
