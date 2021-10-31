use super::super::lexer::logos_lexer::LexerToken;

use super::ast::classes::ClassDecAstNode;
use super::extends_implements::{parse_extends, parse_implements};
use super::fields::parse_fields_until_none_are_left;
use super::methods::parse_methods_until_none_are_found;
use super::parse_error::{combine_parse_errors, ParseError};
use super::types::parse_name_and_generics;
use super::utility_things::{expect_token, LexerStruct};

pub fn parse_class<'a>(lxr: &mut LexerStruct<'a>) -> Result<ClassDecAstNode<'a>, ParseError<'a>> {
    let start_span = lxr.span().unwrap().start;

    let class_type = parse_name_and_generics(lxr)?;

    let extends = parse_extends(lxr)?;
    let implements = parse_implements(lxr)?;

    expect_token(
        lxr,
        LexerToken::LeftCurlyBrace,
        &["`{` (to open the body of the class)"],
    )?;

    let fields_and_final_error = parse_fields_until_none_are_left(lxr)?;
    let fields = fields_and_final_error.0;

    let methods_and_final_error = match parse_methods_until_none_are_found(lxr) {
        Ok(thing) => thing,
        Err(err) => {
            return Err(ParseError {
                fatal: true,
                ..combine_parse_errors(fields_and_final_error.1, err)
            })
        }
    };
    let methods = methods_and_final_error.0;

    if let Err(err) = expect_token(
        lxr,
        LexerToken::RightCurlyBrace,
        &["`}` (to close the body of the class)"],
    ) {
        return Err(combine_parse_errors(methods_and_final_error.1, err));
    }

    Ok(ClassDecAstNode {
        span: start_span..lxr.span().unwrap().end,
        class_type,
        extends,
        implements,
        fields,
        methods,
    })
}
