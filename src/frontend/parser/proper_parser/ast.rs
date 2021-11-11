//! This module holds all of the structs, enums, and type aliases that you will
//! see used to represent a valid AST node.
//!
//! It hosts the modules: `top_level`, `imports_exports`, `enums`, `structs`,
//! `classes`, `interfaces`, `fields`, `types`, `patterns`, `methods`,
//! `publicity`, and `expressions`.
//!
//! The only things relevant to AST data structures that aren't in this file
//! are:
//! 1. certain parsing implementations for publicity structs
//! 2. [`ParseError`](super::parse_error::ParseError)

pub mod top_level {
    //! This module contains the sum type that represents all the possible values
    //! of a single top-level statement.

    use logos::Span;

    use super::classes::ClassDecAstNode;
    use super::enums::EnumDecAstNode;
    use super::imports_exports::ImportStatementAstNode;
    use super::interfaces::InterfaceDecAstNode;
    use super::structs::StructDecAstNode;
    use super::types::TypeAliasAstNode;

    /// This is the sum type ("enum") that represents all the possible top-level
    /// statements.
    ///
    /// `Export(Span, Box<TopLevelAstNode<'a>>)` (a(n) (non-default) export
    /// statement),
    ///
    /// `ExportDefault(Span, Box<TopLevelAstNode<'a>>)` (a default export
    /// statement),
    ///
    /// `EnumDec(Span, EnumDecAstNode<'a>)` (an enum (sum type) declaration),
    ///
    /// `StructDec(Span, StructDecAstNode<'a>)` (a struct declaration),
    ///
    /// `ClassDec(Span, ClassDecAstNode<'a>)` (a class declaration), and
    /// 
    /// `InterfaceDec(Span, InterfaceDecAstNode<'a>)` (an interface declaration)
    ///
    /// `TypeAlias(Span, TypeAliasAstNode<'a>)` (a type alias)
    ///
    /// `TODO: Add Interface declarations to this.`
    #[derive(Debug)]
    pub enum TopLevelAstNode<'a> {
        ImportFrom(Span, ImportStatementAstNode<'a>),

        Export(Span, Box<TopLevelAstNode<'a>>),
        ExportDefault(Span, Box<TopLevelAstNode<'a>>),

        EnumDec(Span, EnumDecAstNode<'a>),

        StructDec(Span, StructDecAstNode<'a>),
        ClassDec(Span, ClassDecAstNode<'a>),
        InterfaceDec(Span, InterfaceDecAstNode<'a>),

        TypeAlias(Span, TypeAliasAstNode<'a>),

        /// (The span of this node is measured from the beginning of the comment to the
        /// end of the subsequent statement.)
        CommentedNode(Span, &'a str, Box<TopLevelAstNode<'a>>),

        /// This should (hopefully) never be used.
        Empty,
    }

    impl<'comment_contents> TopLevelAstNode<'comment_contents> {
        /// Gets the span of the full AST node.
        ///
        /// This gets the span of the ENTIRE statement, from the signaling keyword to
        /// the closing brackets/semicolon.
        /// (For commented nodes, this is the span from the beginning of the comment to
        /// the end of the contained node.)
        pub fn get_span(&self) -> Span {
            match self {
                Self::ClassDec(span, _)
                | Self::CommentedNode(span, _, _)
                | Self::EnumDec(span, _)
                | Self::Export(span, _)
                | Self::ExportDefault(span, _)
                | Self::ImportFrom(span, _)
                | Self::StructDec(span, _)
                | Self::InterfaceDec(span, _)
                | Self::TypeAlias(span, _) => span.clone(),
                Self::Empty => unimplemented!("Can't get the span of an empty AST node."),
            }
        }
    }
}

pub mod imports_exports {
    //! Although this module is called `imports_exports`, it does not house
    //! anything in relation to exports.
    //!
    //! The only things that are in this module are the `ImportStatementAstNode`
    //! struct, and the `AstModuleLocation` enum.
    //!
    //! `TODO: Make this just named "imports".`

    use logos::Span;

    use super::patterns::AstDestructuringPattern;

    /// This struct represents a top-level import statement, supporting
    /// destructuring and rust-like module paths.
    ///
    /// The fields are represented like this:
    /// ```annotated-uck
    ///                      ImportStatementAstNode.span
    ///  _________________________________|_________________________________
    /// │                                                                   │
    /// import vec: { Vec, IntoIter: IntoIterCalledOnVecType } from std::vec;
    ///        |_____________________________________________|      |______|
    ///                              │                                 │
    ///         ImportStatementAstNode.destructuring_pattern           │
    ///                                                                │
    ///                                             ImportStatementAstNode.module_location
    /// ```
    #[derive(Debug)]
    pub struct ImportStatementAstNode<'a> {
        pub span: Span,
        pub destructuring_pattern: AstDestructuringPattern<'a>,
        pub module_location: AstModuleLocation<'a>,
    }

    /// This enum represents a path to a module.
    ///
    /// It parses into a tree from this:
    /// ```
    /// example::module::path::Type
    /// ```
    /// into this:
    /// ```tree-representation
    ///         MemberOf(_, ., .)
    ///                    /    \
    ///                   /      \
    ///                  /     "Type"
    ///                 /
    ///   _____________/______________
    ///  │    MemberOf(_, ., .)       │
    ///  │               /    \       │
    ///  │              /      \      │
    ///  │             /     "path"   │
    ///  │  __________/_____________  │
    ///  │ │ MemberOf(_, ., .)      │ │
    ///  │ │            /    \      │ │
    ///  │ │           /      \     │ │
    ///  │ │          /    "module" │ │
    ///  │ │         /              │ │
    ///  │ │  ______/___________    │ │
    ///  │ │ │ Root(_, .)       │   │ │
    ///  │ │ │          \       │   │ │
    ///  │ │ │           \      │   │ │
    ///  │ │ │        "example" │   │ │
    ///  │ │ │__________________│   │ │
    ///  │ │________________________│ │
    ///  │____________________________│
    /// ```
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
    //! This module only holds things that are strictly specific to enums. Anything
    //! shared between the representation of enums, structs, classes, or other
    //! syntactical concepts is housed in a different module.
    //!
    //! (This also holds as the pattern for most other things in the codebase. Code
    //! reuse is important.)
    //!
    //! Anyway, the only things housed in this module are the structs `EnumDecAstNode` and
    //! `EnumCaseAstNode`

    use logos::Span;

    use super::methods::MethodList;
    use super::publicity::AstPublicity;
    use super::types::AstType;

    /// This struct represents an entire enum declaration, including the name,
    /// generics, trait bounds, cases, and implementations.
    /// 
    /// Here is a practical example of what each struct field corresponds to:
    /// ```text
    ///                                enum_type                       implements
    ///                    ________________|_______________              ___|____
    ///                   |                                |            |        |
    ///            │ enum EnumExample<T: Interface1 & Interface2> implements Clone {
    /// cases ==*==│===> Case1(Type, Vec<Type3>),
    ///         │  │
    ///         *==│===> Case2(Type3),
    ///         │  │
    ///         *==│===> Case3(HashMap<Type1, Type3>),
    ///            │
    ///          / │     pub fn interface3Method(this, ) -> EnumExample {
    /// methods │  │         // TODO: Some implementation here...
    ///         |__│     }
    ///            │ }
    /// ```
    #[derive(Debug)]
    pub struct EnumDecAstNode<'a> {
        pub span: Span,
        pub enum_type: AstType<'a>,
        pub implements: Option<AstType<'a>>,
        pub cases: CaseList<'a>,
        pub methods: MethodList<'a, AstPublicity>,
    }

    /// This type alias is pretty self-explainatory, it's a list of enum cases.
    ///
    /// ```text
    ///     │ enum Enum1 {
    ///     │
    /// 0 ==│===> Case1(Type, Vec<Type3>),
    ///     │  
    /// 1 ==│===> Case2(Type3),
    ///     │
    /// 2 ==│===> Case3(HashMap<Type1, Type3>),
    ///     │
    ///     │ }
    /// ```
    pub type CaseList<'a> = Vec<EnumCaseAstNode<'a>>;

    /// This struct represents a single enum case, which is basically just a case
    /// name and tuple of types that are contained by that case.
    ///
    /// Here is what each struct field refers to:
    /// ```text
    ///              EnumCaseAstNode::span
    ///              __________|__________
    ///             │                     │
    ///             Case1(Type, Vec<Type3>),
    ///             │___│ │______________│
    ///               │            │
    /// EnumCaseAstNode::case_name │
    ///                            │
    ///               EnumCaseAstNode::case_args
    /// ```
    #[derive(Debug)]
    pub struct EnumCaseAstNode<'a> {
        pub span: Span,
        pub case_name: &'a str,
        pub case_args: Vec<AstType<'a>>,
    }
}

pub mod structs {
    use logos::Span;

    use super::fields::FieldList;
    use super::methods::MethodList;
    use super::publicity::AstPublicity;
    use super::types::AstType;

    #[derive(Debug)]
    pub struct StructDecAstNode<'a> {
        pub span: Span,
        pub struct_type: AstType<'a>,
        pub implements: Option<AstType<'a>>,
        pub fields: FieldList<'a, AstPublicity>,
        pub methods: MethodList<'a, AstPublicity>,
    }
}

pub mod classes {
    use logos::Span;

    use super::fields::FieldList;
    use super::methods::MethodList;
    use super::publicity::AstClassItemPublicity;
    use super::types::AstType;

    #[derive(Debug)]
    pub struct ClassDecAstNode<'a> {
        pub span: Span,
        pub class_type: AstType<'a>,
        pub extends: Option<AstType<'a>>,
        pub implements: Option<AstType<'a>>,
        pub fields: FieldList<'a, AstClassItemPublicity>,
        pub methods: MethodList<'a, AstClassItemPublicity>,
    }
}

pub mod interfaces {
    use logos::Span;

    use super::methods::MethodList;
    use super::publicity::InterfaceMethodPublicity;
    use super::types::AstType;

    #[derive(Debug)]
    pub struct InterfaceDecAstNode<'a> {
        pub span: Span,
        pub interface_type: AstType<'a>,
        pub extends: Option<AstType<'a>>,
        pub methods: MethodList<'a, InterfaceMethodPublicity>,
    }
}

pub mod fields {
    use super::types::AstType;
    use logos::Span;

    #[derive(Debug)]
    pub struct FieldAstNode<'a, PublicityEnum> {
        pub span: Span,
        pub publicity: PublicityEnum,
        pub name: &'a str,
        pub field_type: AstType<'a>,
    }

    pub type FieldList<'a, PublicityEnum> = Vec<FieldAstNode<'a, PublicityEnum>>;
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

    use super::expressions::ExpressionBlockAstNode;
    use super::types::{AstType, Generics};

    #[derive(Debug)]
    pub enum MethodOrConstraintAstNode<'a, Publicity> {
        Method(Span, PossiblyDocumentedMethodAstNode<'a, Publicity>),
        Constraint(Span, Generics<'a>, MethodList<'a, Publicity>),
    }

    impl<'a, Publicity> MethodOrConstraintAstNode<'a, Publicity> {
        pub fn get_span(&self) -> Span {
            match self {
                Self::Method(span, _) | Self::Constraint(span, _, _) => span.clone(),
            }
        }
    }

    #[derive(Debug)]
    pub enum AstMethodArgument<'a> {
        This(Span),
        ThisMut(Span),
        Regular(Span, &'a str, AstType<'a>),
    }

    impl<'a> AstMethodArgument<'a> {
        pub fn get_span(&self) -> Span {
            match self {
                Self::This(span) | Self::ThisMut(span) | Self::Regular(span, _, _) => span.clone(),
            }
        }
    }

    #[derive(Debug)]
    pub enum PossiblyDocumentedMethodAstNode<'a, Publicity> {
        BaseMethod(Span, MethodAstNode<'a, Publicity>),
        DocumentedMethod(
            Span,
            &'a str,
            Box<PossiblyDocumentedMethodAstNode<'a, Publicity>>,
        ),
    }

    impl<'a, Publicity> PossiblyDocumentedMethodAstNode<'a, Publicity> {
        pub fn get_span(&self) -> Span {
            match self {
                Self::BaseMethod(span, _) | Self::DocumentedMethod(span, _, _) => span.clone(),
            }
        }
    }

    #[derive(Debug)]
    pub struct MethodAstNode<'a, Publicity> {
        pub span: Span,
        pub publicity: Publicity,
        pub new_type: AstType<'a>,
        pub args: Vec<AstMethodArgument<'a>>,
        pub return_type: Option<AstType<'a>>,
        pub body: ExpressionBlockAstNode,
    }

    pub type MethodList<'a, Publicity> = Vec<MethodOrConstraintAstNode<'a, Publicity>>;
}

pub mod publicity {
    #[derive(Debug)]
    pub enum AstPublicity {
        Public,
        Private,
        ModulePrivate,
    }

    #[derive(Debug)]
    pub enum AstClassItemPublicity {
        Public,
        Private,
        ModulePrivate,
        Protected,
        ModuleProtected,
    }

    #[derive(Debug)]
    pub enum InterfaceMethodPublicity {
        Public,
    }
}

pub mod expressions {
    //! TODOS:
    //! * CONTROL FLOW
    //! * Loops (while, repeat, for)
    //! * Break, continue, return
    //! * If statements
    //! * Match statements
    //! * Match cases
    //! 
    //! OTHER TODOS:
    //! * Operator usage
    //! * Assignment
    //! * Function call
    //! * Member lookup
    //! * Subscript
    //! * Literal
    //! * Type casting
    //! * Variable declarations

    
    
    #[derive(Debug)]
    pub struct ExpressionBlockAstNode {
        
    }
}
