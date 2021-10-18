pub mod top_level {
    use logos::Span;

    use super::classes::ClassDecAstNode;
    use super::enums::EnumDecAstNode;
    use super::imports_exports::ImportStatementAstNode;
    use super::structs::StructDecAstNode;
    use super::types::TypeAliasAstNode;

    #[derive(Debug)]
    pub enum TopLevelAstNode<'a> {
        ImportFrom(Span, ImportStatementAstNode<'a>),

        Export(Span, Box<TopLevelAstNode<'a>>),
        ExportDefault(Span, Box<TopLevelAstNode<'a>>),

        EnumDec(Span, EnumDecAstNode<'a>),

        StructDec(Span, StructDecAstNode<'a>),
        ClassDec(Span, ClassDecAstNode<'a>),

        TypeAlias(Span, TypeAliasAstNode<'a>),

        CommentedNode(Span, &'a str, Box<TopLevelAstNode<'a>>),

        Empty,
    }

    impl<'comment_contents> TopLevelAstNode<'comment_contents> {
        pub fn get_span(&self) -> Span {
            match self {
                Self::ClassDec(span, _)
                | Self::CommentedNode(span, _, _)
                | Self::EnumDec(span, _)
                | Self::Export(span, _)
                | Self::ExportDefault(span, _)
                | Self::ImportFrom(span, _)
                | Self::StructDec(span, _)
                | Self::TypeAlias(span, _) => span.clone(),
                Self::Empty => unimplemented!("Can't get the span of an empty AST node."),
            }
        }
    }
}

pub mod imports_exports {
    use logos::Span;

    use super::patterns::AstDestructuringPattern;

    #[derive(Debug)]
    pub struct ImportStatementAstNode<'a> {
        pub span: Span,
        pub destructuring_pattern: AstDestructuringPattern<'a>,
        pub module_location: AstModuleLocation<'a>,
    }

    #[derive(Debug)]
    pub enum AstModuleLocation<'a> {
        Root(Span, &'a str),
        MemberOf(Span, Box<AstModuleLocation<'a>>, &'a str),
    }

    impl<'a> AstModuleLocation<'a> {
        pub fn get_span(&self) -> Span {
            match self {
                Self::Root(span, _) | Self::MemberOf(span, _, _) => span.clone(),
            }
        }
    }
}

pub mod enums {
    use logos::Span;

    use super::types::AstType;
    use super::methods::MethodList;

    #[derive(Debug)]
    pub struct EnumDecAstNode<'a> {
        pub span: Span,
        pub enum_type: AstType<'a>,
        pub implements: Option<AstType<'a>>,
        pub cases: CaseList<'a>,
        pub methods: MethodList<'a>,
    }

    pub type CaseList<'a> = Vec<EnumCaseAstNode<'a>>;

    #[derive(Debug)]
    pub struct EnumCaseAstNode<'a> {
        pub span: Span,
        pub case_name: &'a str,
        pub case_args: Vec<AstType<'a>>,
    }
}

pub mod structs {
    use logos::Span;
    use std::collections::HashMap;

    use super::types::{AstType, Generics};

    #[derive(Debug)]
    pub struct StructDecAstNode<'a> {
        pub span: Span,
        pub name: &'a str,
        pub generics: Generics<'a>,
        pub implements: Vec<AstType<'a>>,
        pub properties: HashMap<String, AstType<'a>>,
    }
}

pub mod classes {
    use logos::Span;
    use std::collections::HashMap;

    use super::types::AstType;

    #[derive(Debug)]
    pub struct ClassDecAstNode<'a> {
        pub span: Span,
        pub new_type: AstType<'a>,
        pub extends: Option<AstType<'a>>,
        pub implements: Vec<AstType<'a>>,
        pub properties: HashMap<String, AstType<'a>>,
    }
}

pub mod types {
    use logos::Span;
    use std::borrow::Cow;
    use std::collections::HashMap;

    pub type Generics<'a> = (Span, HashMap<Cow<'a, str>, (Span, Option<AstType<'a>>)>);

    #[derive(Debug)]
    pub enum AstType<'a> {
        RootName(Span, &'a str),
        MemberOf(Span, Box<AstType<'a>>, &'a str),
        GenericOf(Span, Box<AstType<'a>>, Generics<'a>),
    }

    impl<'a> AstType<'a> {
        pub fn get_span(&self) -> Span {
            match self {
                Self::RootName(span, _)
                | Self::MemberOf(span, _, _)
                | Self::GenericOf(span, _, _) => span.clone(),
            }
        }
    }

    #[derive(Debug)]
    pub struct TypeAliasAstNode<'a> {
        pub span: Span,
        pub aliased_type: AstType<'a>,
        pub orig_type: AstType<'a>,
    }
}

pub mod patterns {
    use logos::Span;

    #[derive(Debug)]
    pub enum AstDestructuringPattern<'a> {
        Name(Span, &'a str),
        AliasedName(Span, &'a str, &'a str),
        Destructured(Span, &'a str, Vec<AstDestructuringPattern<'a>>),
    }

    impl<'a> AstDestructuringPattern<'a> {
        pub fn get_span(&self) -> Span {
            match self {
                Self::Name(span, _)
                | Self::AliasedName(span, _, _)
                | Self::Destructured(span, _, _) => span.clone(),
            }
        }
    }

    #[derive(Debug)]
    pub enum AstPattern {}
}

pub mod methods {
    use logos::Span;

    use super::types::{AstType, Generics};
    use super::publicity::AstPublicity;
    use super::expressions::ExpressionBlockAstNode;


    #[derive(Debug)]
    pub enum MethodOrConstraintAstNode<'a> {
        Method(Span, PossiblyDocumentedMethodAstNode<'a>),
        Constraint(Span, Generics<'a>, MethodList<'a>),
    }

    impl<'a> MethodOrConstraintAstNode<'a> {
        pub fn get_span(&self) -> Span {
            match self {
                Self::Method(span, _)
                | Self::Constraint(span, _, _) => span.clone(),
            }
        }
    }


    #[derive(Debug)]
    pub enum AstMethodArgument<'a> {
        This(Span),
        ThisMut(Span),
        Regular(Span, &'a str, AstType<'a>)
    }

    impl<'a> AstMethodArgument<'a> {
        pub fn get_span(&self) -> Span {
            match self {
                Self::This(span)
                | Self::ThisMut(span)
                | Self::Regular(span, _, _) => span.clone(),
            }
        }
    }

    #[derive(Debug)]
    pub enum PossiblyDocumentedMethodAstNode<'a> {
        BaseMethod(Span, MethodAstNode<'a>),
        DocumentedMethod(Span, &'a str, Box<PossiblyDocumentedMethodAstNode<'a>>),
    }

    impl<'a> PossiblyDocumentedMethodAstNode<'a> {
        pub fn get_span(&self) -> Span {
            match self {
                Self::BaseMethod(span, _)
                | Self::DocumentedMethod(span, _, _) => span.clone()
            }
        }
    }

    #[derive(Debug)]
    pub struct MethodAstNode<'a> {
        pub span: Span,
        pub publicity: AstPublicity,
        pub new_type: AstType<'a>,
        pub args: Vec<AstMethodArgument<'a>>,
        pub body: ExpressionBlockAstNode,
    }

    pub type MethodList<'a> = Vec<MethodOrConstraintAstNode<'a>>;
}

pub mod publicity {
    #[derive(Debug)]
    pub enum AstPublicity {
        Public,
        Private,
        Unknown,
    }
}


pub mod expressions {
    #[derive(Debug)]
    pub struct ExpressionBlockAstNode {

    }
}