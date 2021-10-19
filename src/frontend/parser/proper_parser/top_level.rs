use crate::frontend::parser::proper_parser::enums::parse_enum_dec;

use super::super::lexer::logos_lexer::LexerToken;

use super::imports_exports::{parse_export_statement, parse_import_statement};
use super::types::parse_type_alias;
use super::utility_things::{LexerStruct, TopLevelAstResult};

use super::ast::top_level::TopLevelAstNode;
use super::parse_error::ParseError;

pub fn parse_top_level<'a: 'b, 'b>(lxr: &'b mut LexerStruct<'a>) -> TopLevelAstResult<'a> {
    const EXPECTED_ARR: &[&str] = &[
        "Import statement",
        "Enum Declaration",
        "Struct Declaration",
        "Class Declaration",
        "Type Alias",
    ];

    match lxr.next() {
        // To deal with stray semicolons. Allows for anything - a class/struct/enum/protocol declaration,
        Some(LexerToken::Semicolon) => parse_top_level(lxr),

        // To deal with documentation comments and attaching them to the correct top level statement.
        Some(LexerToken::Comment) => parse_top_level_documentation_comment(lxr),
        // To deal with parsing export statements.
        Some(LexerToken::Export) => parse_export_statement(lxr),

        // To deal with parsing import statements. Wraps import statement struct in the TopLevelAstNode enum.
        Some(LexerToken::Import) => {
            let import_statement_struct = parse_import_statement(lxr)?;
            Ok(TopLevelAstNode::ImportFrom(
                import_statement_struct.span.clone(),
                import_statement_struct,
            ))
        }

        // To deal with parsing type aliases. Wraps type alias struct in the TopLevelAstNode enum.
        Some(LexerToken::Type) => {
            let type_alias_struct = parse_type_alias(lxr)?;
            Ok(TopLevelAstNode::TypeAlias(
                type_alias_struct.span.clone(),
                type_alias_struct,
            ))
        }

        // To deal with parsing enum declarations. Wraps enum declaration struct in the TopLevelAstNode enum.
        #[allow(unreachable_code, unused_variables, clippy::diverging_sub_expression)]
        Some(LexerToken::Enum) => {
            let enum_declaration_struct: super::ast::enums::EnumDecAstNode = parse_enum_dec(lxr)?;
            Ok(TopLevelAstNode::EnumDec(
                enum_declaration_struct.span.clone(),
                enum_declaration_struct,
            ))
        }

        // To deal with parsing struct declarations. Wraps struct declaration struct in the TopLevelAstNode enum.
        #[allow(unreachable_code, unused_variables, clippy::diverging_sub_expression)]
        Some(LexerToken::Struct) => {
            let struct_declaration_struct: super::ast::structs::StructDecAstNode =
                unimplemented!("Parsing of struct declarations is not yet supported");
            Ok(TopLevelAstNode::StructDec(
                struct_declaration_struct.span.clone(),
                struct_declaration_struct,
            ))
        }

        // To deal with parsing class declarations. Wraps class declaration struct in the TopLevelAstNode enum.
        #[allow(unreachable_code, unused_variables, clippy::diverging_sub_expression)]
        Some(LexerToken::Class) => {
            let class_declaration_struct: super::ast::classes::ClassDecAstNode =
                unimplemented!("Parsing of class declarations is not yet supported");
            Ok(TopLevelAstNode::ClassDec(
                class_declaration_struct.span.clone(),
                class_declaration_struct,
            ))
        }

        // Error if an disallowed token was found at the top level.
        Some(_) => Err(ParseError::unexpected_token_error(
            lxr.slice().unwrap(),
            lxr.span().unwrap(),
            EXPECTED_ARR,
            true,
        )),

        // Error if there is no top level statements left to parse. (This error is **NOT** fatal.)
        None => Err(ParseError::end_of_file_error(EXPECTED_ARR, false)),
    }
}

pub fn parse_top_level_documentation_comment<'a: 'b, 'b>(
    lxr: &'b mut LexerStruct<'a>,
) -> TopLevelAstResult<'a> {
    let span = lxr.span().unwrap();
    let slice = lxr.slice().unwrap();
    parse_top_level(lxr).map(|top_level_statement| {
        TopLevelAstNode::CommentedNode(
            span.start..top_level_statement.get_span().end,
            slice,
            Box::new(top_level_statement),
        )
    })
}
