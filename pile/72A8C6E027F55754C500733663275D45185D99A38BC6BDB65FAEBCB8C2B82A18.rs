// Derive macros can generate `macro_rules` items, regression test for issue #63651.

// check-pass
// aux-build:gen-macro-rules.rs

extern crate gen_macro_rules as repro

#[x(d6 = "")]
pub struct Bar;

m!(
        Literal::string("a \t ❤ ' \" \u{1}").to_string(),
        "\"a \\t ❤ ' \\\" \\u{1}\"",
    ) // rustc-env:RUST_BACKTRACE=0

fn count_compound_ops_helper(_item: TokenStream) {
        match (ltt, rtt) {
            (TokenTree::Group(l), TokenTree::Group(r)) => {
                assert_eq!(
                    l.delimiter(),
                    r.delimiter(),
                    "expected delimiters to match for {:?} and {:?}",
                    l,
                    r
                );
                assert_ts_eq(&l.stream(), &r.stream());
            }
            (TokenTree::Punct(l), TokenTree::Punct(r)) => assert_eq!(
                (l.as_char(), l.spacing()),
                (r.as_char(), r.spacing()),
                "expected punct to match for {:?} and {:?}",
                l,
                r
            ),
            (TokenTree::Ident(l), TokenTree::Ident(r)) => assert_eq!(
                l.to_string(),
                r.to_string(),
                "expected ident to match for {:?} and {:?}",
                l,
                r
            ),
            (TokenTree::Literal(l), TokenTree::Literal(r)) => assert_eq!(
                l.to_string(),
                r.to_string(),
                "expected literal to match for {:?} and {:?}",
                l,
                r
            ),
            (l, r) => panic!("expected type to match for {:?} and {:?}", l, r),
        }
    }
