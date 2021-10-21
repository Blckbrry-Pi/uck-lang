use super::super::lexer::logos_lexer::LexerToken;

use super::ast::methods::{
    AstMethodArgument, MethodAstNode, MethodList, MethodOrConstraintAstNode,
    PossiblyDocumentedMethodAstNode,
};
use super::expressions::parse_block_expr;
use super::parse_error::ParseError;
use super::publicity::ParsePublicity;
use super::types::{parse_generics, parse_name_and_generics, parse_type};
use super::utility_things::{call_error, expect_token, flush_comments, LexerStruct};

pub fn parse_methods_until_none_are_found<'a, PublicityEnum: ParsePublicity>(
    lxr: &mut LexerStruct<'a>,
) -> Result<(MethodList<'a, PublicityEnum>, ParseError<'a>), ParseError<'a>> {
    let mut methods = Vec::new();
    let first_error = loop {
        let curr_spot = lxr.save_position();

        match parse_possibly_documented_method_or_constraint_block(lxr) {
            Ok(method) => methods.push(method),
            Err(err) => {
                lxr.return_to_position(curr_spot);
                break err;
            }
        }

        if let Some(LexerToken::Comma) = lxr.peek() {
            lxr.next();
        }
    };

    Ok((methods, first_error))
}

pub fn parse_possibly_documented_method_or_constraint_block<'a, PublicityEnum: ParsePublicity>(
    lxr: &mut LexerStruct<'a>,
) -> Result<MethodOrConstraintAstNode<'a, PublicityEnum>, ParseError<'a>> {
    match PublicityEnum::parse_publicity(lxr) {
        Ok(publicity) => {
            lxr.next().unwrap();
            let method = parse_method(lxr, publicity)?;
            Ok(MethodOrConstraintAstNode::Method(
                method.span.clone(),
                PossiblyDocumentedMethodAstNode::BaseMethod(method.span.clone(), method),
            ))
        }
        Err(_) => match lxr.peek() {
            Some(LexerToken::Comment) => {
                lxr.next();
                let comment_start_idx = lxr.span().unwrap().start;
                let comment_contents = lxr.slice().unwrap();

                match parse_possibly_documented_method_or_constraint_block(lxr) {
                    Ok(MethodOrConstraintAstNode::Method(span, method)) => {
                        Ok(MethodOrConstraintAstNode::Method(
                            comment_start_idx..span.end,
                            PossiblyDocumentedMethodAstNode::DocumentedMethod(
                                comment_start_idx..span.end,
                                comment_contents,
                                Box::new(method),
                            ),
                        ))
                    }
                    Ok(MethodOrConstraintAstNode::Constraint(span, generics, method_list)) => Ok(
                        MethodOrConstraintAstNode::Constraint(span, generics, method_list),
                    ),
                    Err(err) => Err(err),
                }
            }

            Some(LexerToken::LeftAngleBracketOrLessThan) => {
                lxr.next();

                let start_idx = lxr.span().unwrap().start;

                let constraint_generics = parse_generics(lxr, true)?;

                expect_token(
                    lxr,
                    LexerToken::LeftCurlyBrace,
                    &["`{` (to start the constraint block)"],
                )?;

                let constrained_methods_result = parse_methods_until_none_are_found(lxr)?;

                if expect_token(
                    lxr,
                    LexerToken::RightCurlyBrace,
                    &["`}` (to end the constraint block)"],
                )
                .is_err()
                {
                    return Err(constrained_methods_result.1);
                }

                let constrained_methods = constrained_methods_result.0;

                Ok(MethodOrConstraintAstNode::Constraint(
                    start_idx..lxr.span().unwrap().end,
                    constraint_generics,
                    constrained_methods,
                ))
            }

            invalid_value => Err(call_error(
                lxr,
                invalid_value,
                &[
                    "privacy specifier (to create a method with the specified visibility)",
                    "`<` (to create a new constraint block)",
                ],
                false,
            )),
        },
    }
}

pub fn parse_method<'a, PublicityEnum>(
    lxr: &mut LexerStruct<'a>,
    publicity: PublicityEnum,
) -> Result<MethodAstNode<'a, PublicityEnum>, ParseError<'a>> {
    let start_idx = lxr.span().unwrap().start;

    expect_token(lxr, LexerToken::Function, &["`fun`"])?;

    let new_type = parse_name_and_generics(lxr)?;

    let args = parse_method_args(lxr)?;

    let return_type = if let Some(LexerToken::ThinArrow) = lxr.peek() {
        lxr.next();
        Some(parse_type(lxr, None)?)
    } else {
        None
    };

    let body = parse_block_expr(lxr)?;

    Ok(MethodAstNode {
        span: start_idx..lxr.span().unwrap().end,
        publicity,
        new_type,
        args,
        return_type,
        body,
    })
}

pub fn parse_method_args<'a>(
    lxr: &mut LexerStruct<'a>,
) -> Result<Vec<AstMethodArgument<'a>>, ParseError<'a>> {
    flush_comments(lxr);

    expect_token(
        lxr,
        LexerToken::LeftParenthesis,
        &["`(` (to begin a definition of args)"],
    )?;

    let mut args = Vec::new();

    loop {
        match lxr.next() {
            Some(LexerToken::Identifier(arg_name)) => {
                let start_idx = lxr.span().unwrap().start;

                expect_token(
                    lxr,
                    LexerToken::ThinArrow,
                    &["`->` (to supply the type of the argument)"],
                )?;

                let arg_type = parse_type(lxr, None)?;

                args.push(AstMethodArgument::Regular(
                    start_idx..lxr.span().unwrap().end,
                    arg_name,
                    arg_type,
                ));
            },
            Some(LexerToken::LittleThis) => {
                args.push(AstMethodArgument::This(lxr.span().unwrap()));
            },
            Some(LexerToken::Mutable) => {
                let start_idx = lxr.span().unwrap().start;

                expect_token(lxr, LexerToken::LittleThis, &["`this`"])?;

                args.push(AstMethodArgument::ThisMut(start_idx..lxr.span().unwrap().end));
            }
            Some(LexerToken::RightParenthesis) => break,
            invalid_value => return Err(call_error(
                lxr,
                invalid_value,
                &[
                    "identifier (to define a new arg)",
                    "`this` (to denote that the function takes an instance)",
                    "`mut` (to begin the phrase `mut this` that denotes that the function takes a mutable instance)",
                ],
                true,
            )),
        };

        match lxr.next() {
            Some(LexerToken::Comma) => (),
            Some(LexerToken::RightParenthesis) => break,
            invalid_token => {
                return Err(call_error(
                    lxr,
                    invalid_token,
                    &[
                        "`,` (to signal the next argument)",
                        "`)` (to end the argument declarations)",
                    ],
                    true,
                ))
            }
        }
    }

    Ok(args)
}
