use std::collections::HashMap;
use std::borrow::Cow;
use logos::Span;

#[derive(Debug)]
pub enum TopLevelAstNode<'a> {
    ImportFrom(Span, AstDestructuringPattern<'a>, AstModuleLocation<'a>),

    Export(Span, Box<TopLevelAstNode<'a>>),
    ExportDefault(Span, Box<TopLevelAstNode<'a>>),

    EnumDec(Span, String, EnumDecAstNode<'a>),

    StructDec(Span, String, StructDecAstNode<'a>),
    ClassDec(Span, String, ClassDecAstNode<'a>),

    TypeAlias(Span, AstType<'a>, AstType<'a>),

    CommentedNode(Span, &'a str, Box<TopLevelAstNode<'a>>),

    Empty,
}

impl<'comment_contents> TopLevelAstNode<'comment_contents> {
    pub fn get_span(&self) -> Span {
        match self {
            Self::ClassDec(span, _, _) |
            Self::CommentedNode(span, _, _) |
            Self::EnumDec(span, _, _) |
            Self::Export(span, _) |
            Self::ExportDefault(span, _) |
            Self::ImportFrom(span, _, _) |
            Self::StructDec(span, _, _) |
            Self::TypeAlias(span, _, _) => span.clone(),
            Self::Empty => unimplemented!("Can't get the span of an empty AST node.")
        }
    }
}

pub type Generics<'a> = (Span, HashMap<Cow<'a, str>, (Span, Option<AstType<'a>>)>);

#[derive(Debug)]
pub struct EnumDecAstNode<'a> {
    generics: Generics<'a>,
    values: HashMap<String, (Span, Vec<AstType<'a>>)>,
}

#[derive(Debug)]
pub struct StructDecAstNode<'a> {
    generics: Generics<'a>,
    implements: Vec<AstType<'a>>,
    properties: HashMap<String, AstType<'a>>,
}

#[derive(Debug)]
pub struct ClassDecAstNode<'a> {
    generics: Generics<'a>,
    implements: Vec<AstType<'a>>,
    extends: Option<AstType<'a>>,
    properties: HashMap<String, AstType<'a>>,
}

#[derive(Debug)]
pub enum AstType<'a> {
    RootName(Span, &'a str),
    MemberOf(Span, Box<AstType<'a>>, &'a str),
    GenericOf(Span, Box<AstType<'a>>, Generics<'a>),
}

impl<'a> AstType<'a> {
    pub fn get_span(&self) -> Span {
        match self {
            Self::RootName(span, _) |
            Self::MemberOf(span, _, _) |
            Self::GenericOf(span, _, _) => span.clone(),
        }
    }
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
