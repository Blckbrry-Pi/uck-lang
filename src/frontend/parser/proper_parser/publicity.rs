use super::super::lexer::logos_lexer::LexerToken;
use super::ast::publicity::{AstClassItemPublicity, AstPublicity};
use super::parse_error::ParseError;
use super::utility_things::{call_error, LexerStruct};

pub trait ParsePublicity {
    fn parse_publicity<'a>(lxr: &mut LexerStruct<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized;
}

impl ParsePublicity for AstPublicity {
    fn parse_publicity<'a>(lxr: &mut LexerStruct<'a>) -> Result<Self, ParseError<'a>> {
        match lxr.peek() {
            Some(LexerToken::Public) => Ok(AstPublicity::Public),
            Some(LexerToken::ModulePrivate) => Ok(AstPublicity::ModulePrivate),
            Some(LexerToken::Private) => Ok(AstPublicity::Private),
            invalid_token => Err(call_error(
                lxr,
                invalid_token,
                &["[pub, mpriv, or priv] (to declare the publicity of a struct item)"],
                false,
            )),
        }
    }
}

impl ParsePublicity for AstClassItemPublicity {
    fn parse_publicity<'a>(lxr: &mut LexerStruct<'a>) -> Result<Self, ParseError<'a>> {
        match lxr.peek() {
            Some(LexerToken::Public) => Ok(AstClassItemPublicity::Public),
            Some(LexerToken::ModulePrivate) => Ok(AstClassItemPublicity::ModulePrivate),
            Some(LexerToken::Private) => Ok(AstClassItemPublicity::Private),
            invalid_token => Err(call_error(
                lxr,
                invalid_token,
                &["[pub, mprot, prot, mpriv, or priv] (to declare the publicity of a class item)"],
                false,
            )),
        }
    }
}
