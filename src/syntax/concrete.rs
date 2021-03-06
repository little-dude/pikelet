//! The concrete syntax of the language

use source::pos::{BytePos, Span};
use std::fmt;
use std::usize;

use syntax::pretty::{self, ToDoc};

/// Commands entered in the REPL
#[derive(Debug, Clone)]
pub enum ReplCommand {
    /// Evaluate a term
    ///
    /// ```text
    /// <term>
    /// ```
    Eval(Box<Term>),
    /// Print some help about using the REPL
    ///
    /// ```text
    /// :?
    /// :h
    /// :help
    /// ```
    Help,
    ///  No command
    NoOp,
    /// Quit the REPL
    ///
    /// ```text
    /// :q
    /// :quit
    /// ```
    Quit,
    /// Print the type of the term
    ///
    /// ```text
    /// :t <term>
    /// :type <term>
    /// ```
    TypeOf(Box<Term>),
}

/// A module definition:
///
/// ```text
/// module my-module;
///
/// <declarations>
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    /// The name of the module
    pub name: (Span, String),
    /// The declarations contained in the module
    pub declarations: Vec<Declaration>,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_doc(pretty::Options::default().with_debug_indices(f.alternate()))
            .group()
            .render_fmt(f.width().unwrap_or(usize::MAX), f)
    }
}

/// Top level declarations
#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    /// Imports a module into the current scope
    ///
    /// ```text
    /// import foo;
    /// import foo as my-foo;
    /// import foo as my-foo (..);
    /// ```
    Import {
        span: Span,
        name: (Span, String),
        rename: Option<(Span, String)>,
        exposing: Option<Exposing>,
    },
    /// Claims that a term abides by the given type
    ///
    /// ```text
    /// foo : some-type
    /// ```
    Claim { name: (Span, String), ann: Term },
    /// Declares the body of a term
    ///
    /// ```text
    /// foo = some-body
    /// foo x (y : some-type) = some-body
    /// ```
    Definition {
        name: (Span, String),
        params: LamParams,
        body: Term,
    },
    /// Declarations that could not be correctly parsed
    ///
    /// This is used for error recovery
    Error(Span),
}

impl Declaration {
    /// Return the span of source code that this declaration originated from
    pub fn span(&self) -> Span {
        match *self {
            Declaration::Import { span, .. } => span,
            Declaration::Claim { ref name, ref ann } => name.0.to(ann.span()),
            Declaration::Definition {
                ref name, ref body, ..
            } => name.0.to(body.span()),
            Declaration::Error(span) => span,
        }
    }
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_doc(pretty::Options::default().with_debug_indices(f.alternate()))
            .group()
            .render_fmt(f.width().unwrap_or(usize::MAX), f)
    }
}

/// A list of the definitions imported from a module
#[derive(Debug, Clone, PartialEq)]
pub enum Exposing {
    /// Import all the definitions in the module into the current scope
    ///
    /// ```text
    /// (..)
    /// ```
    All(Span),
    /// Import an exact set of definitions into the current scope
    ///
    /// ```text
    /// (foo, bar as baz)
    /// ```
    Exact(Span, Vec<((Span, String), Option<(Span, String)>)>),
    /// Exposing declarations that could not be correctly parsed
    ///
    /// This is used for error recovery
    Error(Span),
}

impl fmt::Display for Exposing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_doc(pretty::Options::default().with_debug_indices(f.alternate()))
            .group()
            .render_fmt(f.width().unwrap_or(usize::MAX), f)
    }
}

/// Terms
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    /// A term that is surrounded with parentheses
    ///
    /// ```text
    /// (e)
    /// ```
    Parens(Span, Box<Term>),
    /// A term annotated with a type
    ///
    /// ```text
    /// e : t
    /// ```
    Ann(Box<Term>, Box<Term>),
    /// Type of types
    ///
    /// ```text
    /// Type
    /// ```
    Universe(Span, Option<u32>),
    /// Variables
    ///
    /// ```text
    /// x
    /// ```
    Var(Span, String),
    /// Lambda abstractions
    ///
    /// ```text
    /// \x => t
    /// \x y => t
    /// \x : t1 => t2
    /// \(x : t1) y (z : t2) => t3
    /// \(x y : t1) => t3
    /// ```
    Lam(BytePos, LamParams, Box<Term>),
    /// Dependent function types
    ///
    /// ```text
    /// (x : t1) -> t2
    /// (x y : t1) -> t2
    /// ```
    Pi(BytePos, PiParams, Box<Term>),
    /// Non-Dependent function types
    ///
    /// ```text
    /// t1 -> t2
    /// ```
    Arrow(Box<Term>, Box<Term>),
    /// Term application
    ///
    /// ```text
    /// e1 e2
    /// ```
    App(Box<Term>, Box<Term>),
    /// Terms that could not be correctly parsed
    ///
    /// This is used for error recovery
    Error(Span),
}

impl Term {
    /// Return the span of source code that this term originated from
    pub fn span(&self) -> Span {
        match *self {
            Term::Parens(span, _)
            | Term::Universe(span, _)
            | Term::Var(span, _)
            | Term::Error(span) => span,
            Term::Lam(start, _, ref body) | Term::Pi(start, _, ref body) => {
                Span::new(start, body.span().hi())
            },
            Term::Ann(ref term, ref ty) => term.span().to(ty.span()),
            Term::Arrow(ref ann, ref body) => ann.span().to(body.span()),
            Term::App(ref fn_term, ref arg) => fn_term.span().to(arg.span()),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_doc(pretty::Options::default().with_debug_indices(f.alternate()))
            .group()
            .render_fmt(f.width().unwrap_or(usize::MAX), f)
    }
}

/// The parameters to a lambda abstraction
pub type LamParams = Vec<(Vec<(Span, String)>, Option<Box<Term>>)>;

/// The parameters to a dependent function type
pub type PiParams = (Vec<(Span, String)>, Box<Term>);
