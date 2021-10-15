use std::collections::HashMap;
use logos::Span;

#[derive(Debug)]
pub enum TopLevelAstNode<'a> {
    ImportFrom(Span, AstDestructuringPattern<'a>, AstModuleLocation<'a>),

    Export(Span, Box<TopLevelAstNode<'a>>),
    ExportDefault(Span, Box<TopLevelAstNode<'a>>),

    EnumDec(Span, String, EnumDecAstNode),

    StructDec(Span, String, StructDecAstNode),


    CommentedNode(Span, Span, Box<TopLevelAstNode<'a>>),

    Empty,
}

impl<'comment_contents> TopLevelAstNode<'comment_contents> {
    pub fn get_span(&self) -> Span {
        match self {
            Self::CommentedNode(span, _, _) |
            Self::EnumDec(span, _, _) |
            Self::Export(span, _) |
            Self::ExportDefault(span, _) |
            Self::ImportFrom(span, _, _) |
            Self::StructDec(span, _, _) => span.clone(),
            Self::Empty => unimplemented!("Can't get the span of an empty AST node.")
        }
    }
}

#[derive(Debug)]
pub struct EnumDecAstNode {
    generics: HashMap<String, (Span, AstType)>,
    values: HashMap<String, (Span, Vec<AstType>)>,
}

#[derive(Debug)]
pub struct StructDecAstNode {
    generics: HashMap<String, (Span, AstType)>,

    properties: HashMap<String, AstType>,
}

#[derive(Debug)]
pub enum AstType {

}

#[derive(Debug)]
pub enum AstModuleLocation<'a> {
    Root(Span, &'a str),
    MemberOf(Span, Box<AstModuleLocation<'a>>, &'a str),
}

impl<'a> AstModuleLocation<'a> {
    pub fn get_span(&self) -> Span {
        match self {
            Self::Root(span, _) |
            Self::MemberOf(span, _, _) => span.clone(),
        }
    }
}

#[derive(Debug)]
pub enum AstDestructuringPattern<'a> {
    Name(Span, &'a str),
    AliasedName(Span, &'a str, &'a str),
    Destructured(Span, &'a str, Box<Vec<AstDestructuringPattern<'a>>>),
}

impl<'a> AstDestructuringPattern<'a> {
    pub fn get_span(&self) -> Span {
        match self {
            Self::Name(span, _) |
            Self::AliasedName(span, _, _) |
            Self::Destructured(span, _, _) => span.clone(),
        }
    }
}

#[derive(Debug)]
pub enum AstPattern {
}
