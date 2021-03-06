use source::{CodeMap, FileName};

use syntax::parse;
use syntax::translation::FromConcrete;

use super::*;

fn parse(src: &str) -> RcTerm {
    let mut codemap = CodeMap::new();
    let filemap = codemap.add_filemap(FileName::virtual_("test"), src.into());

    let (concrete_term, errors) = parse::term(&filemap);
    assert!(errors.is_empty());

    RcTerm::from_concrete(&concrete_term)
}

mod alpha_eq {
    use super::*;

    #[test]
    fn var() {
        assert_eq!(parse(r"x"), parse(r"x"));
    }

    #[test]
    #[should_panic]
    fn var_diff() {
        assert_eq!(parse(r"x"), parse(r"y"));
    }

    #[test]
    fn ty() {
        assert_eq!(parse(r"Type"), parse(r"Type"));
    }

    #[test]
    fn lam() {
        assert_eq!(parse(r"\x : Type => x"), parse(r"\a : Type => a"));
    }

    #[test]
    fn pi() {
        assert_eq!(parse(r"(x : Type) -> x"), parse(r"(a : Type) -> a"));
    }

    #[test]
    fn lam_app() {
        assert_eq!(
            parse(r"\x : Type -> Type => x Type"),
            parse(r"\a : Type -> Type => a Type")
        );
    }

    #[test]
    fn pi_app() {
        assert_eq!(
            parse(r"(x : Type -> Type) -> x Type"),
            parse(r"(a : Type -> Type) -> a Type")
        );
    }

    #[test]
    fn lam_lam_app() {
        assert_eq!(
            parse(r"\x : Type -> Type => \y : Type => x y"),
            parse(r"\a : Type -> Type => \b : Type => a b"),
        );
    }

    #[test]
    fn pi_pi_app() {
        assert_eq!(
            parse(r"(x : Type -> Type) -> (y : Type) -> x y"),
            parse(r"(a : Type -> Type) -> (b : Type) -> a b"),
        );
    }
}
