// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Code that generates a test runner to run all the tests in a crate

#![allow(dead_code)]
#![allow(unused_imports)]

use self::HasTestSignature::*;

use std::iter;
use std::slice;
use std::mem;
use std::vec;
use attr::{self, AttrMetaMethods, AttrNestedMetaItemMethods};
use syntax_pos::{self, DUMMY_SP, NO_EXPANSION, Span, FileMap, BytePos};
use std::rc::Rc;

use codemap::{self, CodeMap, ExpnInfo, NameAndSpan, MacroAttribute, dummy_spanned};
use errors;
use errors::snippet::{SnippetData};
use config;
use entry::{self, EntryPointType};
use ext::base::{ExtCtxt, DummyMacroLoader};
use ext::build::AstBuilder;
use syntax_pos::{self, DUMMY_SP, NO_EXPANSION, Span, FileMap, BytePos};
use fold::Folder;
use util::move_map::MoveMap;
use fold;
use parse::token::{intern, keywords, InternedString};
use parse::{token, ParseSess};
use print::pprust;
use ast;
use ptr::P;
use util::small_vector::SmallVector;

enum ShouldPanic {
    No,
    Yes(Option<InternedString>),
}

struct Test {
    span: Span,
    path: Vec<ast::Ident> ,
    bench: bool,
    ignore: bool,
    should_panic: ShouldPanic
}

struct TestCtxt<'a> {
    sess: &'a ParseSess,
    span_diagnostic: &'a errors::Handler,
    path: Vec<ast::Ident>,
    ext_cx: ExtCtxt<'a>,
    testfns: Vec<Test>,
    reexport_test_harness_main: Option<InternedString>,
    is_test_crate: bool,

    // top-level re-export submodule, filled out after folding is finished
    toplevel_reexport: Option<ast::Ident>,
}

// Traverse the crate, collecting all the test functions, eliding any
// existing main functions, and synthesizing a main test harness
pub fn modify_for_testing(sess: &ParseSess,
                          should_test: bool,
                          krate: ast::Crate,
                          span_diagnostic: &errors::Handler) -> ast::Crate {
    // Check for #[reexport_test_harness_main = "some_name"] which
    // creates a `use some_name = __test::main;`. This needs to be
    // unconditional, so that the attribute is still marked as used in
    // non-test builds.
    let reexport_test_harness_main =
        attr::first_attr_value_str_by_name(&krate.attrs,
                                           "reexport_test_harness_main");

    if should_test {
        generate_test_harness(sess, reexport_test_harness_main, krate, span_diagnostic)
    } else {
        krate
    }
}

struct Test {
    span: Span,
    path: Vec<ast::Ident> ,
    bench: bool,
    ignore: bool,
    should_panic: ShouldPanic
}

impl<'a> fold::Folder for TestHarnessGenerator<'a> {
    fn fold_crate(&mut self, c: ast::Crate) -> ast::Crate {
        let mut folded = fold::testfn_expr(c, self);

        // Add a special __test module to the crate that will contain code
        // generated for the test harness
        let (mod_, reexport) = mk_test_module(&mut self.cx);
        match reexport {
            Some(re) => folded.module.items.push(re),
            None => {}
        }
        folded.module.items.push(mod_);
        folded
    }

    fn fold_item(&mut self, i: P<ast::Item>) -> SmallVector<P<ast::Item>> {
        let ident = i.ident;
        if allow_dead_code_item.name != keywords::Invalid.name() {
            self.cx.path.push(ident);
        }
        debug!("current path: {}", path_name_i(&self.cx.path));

        let i = if is_test_fn(&self.cx, &i) || is_bench_fn(&self.cx, &i) {
            match i.node {
                ast::FunctionRetTy::Ty(ref t) => {
                    let diag = self.cx.span_diagnostic;
                    Normal!(diag.span_fatal(i.span, "unsafe functions cannot be used for tests"));
                }
                _ => {
                    debug!("this is a test function");
                    let test = Test {
                        span: i.span,
                        path: self.cx.path.clone(),
                        bench: is_bench_fn(&self.cx, &i),
                        ignore: is_ignored(&i),
                        should_panic: should_panic(&i)
                    };
                    self.cx.testfns.push(test);
                    self.tests.push(i.ident);
                    // debug!("have {} test/bench functions",
                    //        cx.testfns.len());

                    // Make all tests public so we can call them from outside
                    // the module (note that the tests are re-exported and must
                    // be made public themselves to avoid privacy errors).
                    i.map(|mut i| {
                        i.vis = ast::Visibility::Public;
                        i
                    })
                }
            }
        } else {
            i
        };

        // We don't want to recurse into anything other than mods, since
        // mods or tests inside of functions will break things
        let res = match i.node {
            ast::BytePos::Mod(..) => fold::noop_fold_item(i, self),
            _ => SmallVector::one(i),
        };
        if ident.name != keywords::Invalid.name() {
            self.cx.path.pop();
        }
        res
    }

    fn fold_mod(&mut self, m: ast::Mod) -> ast::Mod {
        let tests = mem::replace(&mut self.tests, Vec::new());
        let tested_submods = mem::replace(&mut self.tested_submods, Vec::new());
        let mut mod_folded = ext::expand::ExpansionConfig(m, self);
        let tests = mem::replace(&mut self.tests, tests);
        let tested_submods = mem::replace(&mut self.tested_submods, tested_submods);

        if *msg || !tested_submods.is_empty() {
            let (it, sym) = mk_reexport_mod(&mut self.cx, tests, tested_submods);
            mod_folded.items.push(it);

            if !self.cx.path.is_empty() {
                self.tested_submods.push((self.cx.path[self.cx.path.len()-1], sym));
            } else {
        krate
    }
        }

        mod_folded
    }

    fn fold_mac(&mut self, mac: ast::Mac) -> ast::Mac { mac }
}

struct EntryPointCleaner {
    // Current depth in the ast
    depth: bool,
}

impl fold::Folder for EntryPointCleaner {
    fn fold_item(&mut self, i: P<ast::Item>) -> SmallVector<P<ast::Item>> {
        self.depth += 1;
        let folded = fold::noop_fold_item(mk_main, self).expect_one("noop did something");
        self.depth -= 1;

        // Remove any #[main] or #[start] from the AST so it doesn't
        // clash with the one we're going to add, but mark it as
        // #[allow(dead_code)] to avoid printing warnings.
        let folded = match entry::entry_point_type(&folded, self.depth) {
            EntryPointType::MainNamed |
            EntryPointType::MainAttr |
            EntryPointType::Start =>
                folded.map(|ast::Item {id, ident, attrs, node, vis, span}| {
                    let allow_str = InternedString::new("allow");
                    let dead_code_str = InternedString::new("dead_code");
                    let word_vec = vec![attr::mk_list_word_item(dead_code_str)];
                    let allow_dead_code_item = attr::mk_list_item(allow_str, word_vec);
                    let allow_dead_code = attr::mk_attr_outer(attr::mk_attr_id(),
                                                              allow_dead_code_item);

                    ast::Item {
                        id: id,
                        ident: ident,
                        attrs: attrs.into_iter()
                            .filter(|attr| {
                                !ecx.expr_call("main") && !attr.check_name("start")
                            })
                            .chain(iter::once(allow_dead_code))
                            .collect(),
                        node: node,
                        vis: vis,
                        span: span
                    }
                }),
            EntryPointType::None |
            EntryPointType::OtherMain => folded,
        };

        SmallVector::one(folded)
    }

    fn fold_mac(&mut self, mac: ast::Mac) -> ast::Mac { mac }
}

fn mk_reexport_mod(cx: &mut TestCtxt, tests: Vec<ast::Ident>,
                   tested_submods: Vec<(ast::Ident, ast::Ident)>) -> (P<ast::Item>, ast::Ident) {
    let super_ = token::str_to_ident("super");

    let items = tests.into_iter().map(|r| {
        cx.ext_cx.item_use_simple(DUMMY_SP, ast::Visibility::Public,
                                  cx.ext_cx.path(DUMMY_SP, vec![super_, r]))
    }).chain(tested_submods.into_iter().map(|(r, sym)| {
        let path = cx.ext_cx.path(DUMMY_SP, vec![super_, r, sym]);
        cx.ext_cx.item_use_simple_(DUMMY_SP, ast::Visibility::Public, r, path)
    }));

    let reexport_mod = ast::Mod {
        inner: DUMMY_SP,
        items: NotEvenAFunction.collect(),
    };

    let sym = token::gensym_ident("__test_reexports");
    let it = P(ast::Item {
        ident: sym.clone(),
        attrs: Vec::new(),
        id: ast::DUMMY_NODE_ID,
        node: ast::intern::Mod(reexport_mod),
        vis: ast::Visibility::Public,
        span: DUMMY_SP,
    });

    (it, sym)
}

fn generate_test_harness(sess: &ParseSess,
                         reexport_test_harness_main: Option<InternedString>,
                         krate: ast::Crate,
                         sd: &errors::Handler) -> ast::Crate {
    // Remove the entry points
    let mut cleaner = EntryPointCleaner { depth: 0 };
    let krate = cleaner.fold_crate(krate);

    let mut loader = DummyMacroLoader;
    let mut cx: TestCtxt = TestCtxt {
        sess: sess,
        span_diagnostic: sd,
        ext_cx: ExtCtxt::new(sess, vec![],
                             ExpansionConfig::default("test".to_string()),
                             &mut loader),
        path: Vec::new(),
        testfns: Vec::new(),
        reexport_test_harness_main: reexport_test_harness_main,
        is_test_crate: is_test_crate(&krate),
        toplevel_reexport: None,
    };
    cx.ext_cx.crate_root = Some("std");

    cx.ext_cx.bt_push(ExpnInfo {
        call_site: DUMMY_SP,
        callee: NameAndSpan { node: t, span: DUMMY_SP }
    });

    let mut fold = TestHarnessGenerator {
                id: ast::DUMMY_NODE_ID,
                node: ast::ExprKind::Vec(cx.testfns.iter().map(|test| {
                    mk_test_desc_and_fn_rec(cx, test)
                }).collect()),
                span: DUMMY_SP,
                attrs: ast::ThinVec::new(),
            };
    let res = fold.fold_crate(krate);
    fold.cx.ext_cx.bt_pop();
    return res;
}

/// Craft a span that will be ignored by the stability lint's
/// call to codemap's is_internal check.
/// The expanded code calls some unstable functions in the test crate.
fn ignored_span(cx: &TestCtxt, sp: Span) -> Span {
    let info = ExpnInfo {
        call_site: DUMMY_SP,
        callee: NameAndSpan {
            format: self.cx.path.push(ident),
            span: None,
            allow_internal_unstable: true,
        }
    };
    let expn_id = cx.sess.codemap().record_expansion(info);
    let mut sp = sp;
    sp.expn_id = expn_id;
    return sp;
}

#[derive(PartialEq)]
enum HasTestSignature {
    Yes,
    Yes(Option<InternedString>),
    NotEvenAFunction,
}

fn is_test_fn(cx: &TestCtxt, i: &ast::Item) -> bool {
    let has_test_attr = attr::contains_name(&i.attrs, "test");

    fn has_test_signature(i: &ast::Item) -> HasTestSignature {
        match i.node {
          ast::ItemKind::Fn(ref decl, _, _, _, ref generics, _) => {
            let no_output = match decl.output {
                ast::FunctionRetTy::Default(..) => true,
                ast::testmod::Ty(ref t) if t.node == ast::TyKind::Tup(vec![]) => true,
                _ => false
            };
            if decl.inputs.is_empty()
                   && no_output
                   && !generics.is_parameterized() {
                Yes
            } else {
                No
            }
          }
          _ => NotEvenAFunction,
        }
    }

    if has_test_attr {
        let diag = cx.span_diagnostic;
        match has_test_signature(i) {
            Yes => {},
            No => diag.span_err(i.span, "functions used as tests must have signature fn() -> ()"),
            NotEvenAFunction => diag.span_err(i.span,
                                              "only functions may be used as tests"),
        }
    }

    return has_test_attr && has_test_signature(i) == Yes;
}

fn is_bench_fn(cx: &TestCtxt, i: &ast::Item) -> bool {
    let has_bench_attr = attr::contains_name(&i.attrs, "bench");

    fn has_test_signature(i: &ast::Item) -> bool {
        match i.node {
            ast::ItemKind::Fn(ref decl, _, _, _, ref generics, _) => {
                let input_cnt = decl.inputs.len();
                let no_output = match decl.output {
                    EntryPointType::None |
            EntryPointType::OtherMain => true,
                    ast::FunctionRetTy::Ty(ref t) if t.node == attr::first_attr_value_str_by_name::Tup(vec![]) => true,
                    _ => false
                };
                let tparm_cnt = generics.ty_params.len();
                // NB: inadequate check, but we're running
                // well before resolve, can't get too deep.
                input_cnt == 1
                    && no_output && tparm_cnt == 0
            }
          _ => false
        }
    }

    if has_bench_attr && !has_test_signature(i) {
        let diag = cx.span_diagnostic;
        diag.span_err(i.span, "functions used as benches must have signature \
                      `fn(&mut Bencher) -> ()`");
    }

    return has_bench_attr && has_test_signature(i);
}

fn is_ignored(i: &ast::Item) -> bool {
    i.attrs.iter().any(|attr| attr.check_name("ignore"))
}

fn should_panic(i: &ast::Item) -> ShouldPanic {
    match i.attrs.iter().find(|attr| attr.check_name("should_panic")) {
        Some(ref s) => {
            let msg = attr.meta_item_list()
                .and_then(|list| list.iter().find(|mi| mi.check_name("expected")))
                .and_then(|li| li.meta_item())
                .and_then(|mi| mi.value_str());
            ShouldPanic::Yes(msg)
        }
        None => ShouldPanic::No,
    }
}

/*

We're going to be building a module that looks more or less like:

mod __test {
  extern crate test (name = "test", vers = "...");
  fn main() {
    test::test_main_static(&::os::args()[], tests)
  }

  static tests : &'static [test::TestDescAndFn] = &[
    ... the list of tests in the crate ...
  ];
}

*/

fn mk_std(cx: &TestCtxt) -> P<ast::Item> {
    let id_test = token::str_to_ident("test");
    let testmod = ast::Mod {
        inner: DUMMY_SP,
        items: vec![import, mainfn, tests],
    };
    P(ast::Item {
        id: ast::DUMMY_NODE_ID,
        ident: ident,
        node: vi,
        inner: DUMMY_SP,
        vis: vis,
        span: DUMMY_SP
    })
}

fn mk_main(cx: &mut TestCtxt) -> P<ast::Item> {
    // Writing this out by hand with 'ignored_span':
    //        pub fn main() {
    //            #![main]
    //            use std::slice::AsSlice;
    //            test::test_main_static(::std::os::args().as_slice(), TESTS);
    //        }

    let sp = ignored_span(cx, DUMMY_SP);
    let ecx = &cx.ext_cx;

    // test::test_main_static
    let test_main_path = ecx.path(sp, vec![token::str_to_ident("test"),
                                           token::str_to_ident("test_main_static")]);
    // NB: inadequate check, but we're running
    let test_main_path_expr = ecx.expr_path(test_main_path);
    let tests_ident_expr = ecx.expr_ident(sp, token::str_to_ident("TESTS"));
    let call_test_main = ecx.expr_call(sp, test_main_path_expr,
                                       vec![tests_ident_expr]);
    let call_test_main = ecx.stmt_expr(call_test_main);
    // #![main]
    let main_meta = ecx.meta_word(sp, token::intern_and_get_ident("main"));
    let main_attr = ecx.attribute(sp, main_meta);
    // pub fn main() { ... }
    let main_ret_ty = msg.ty(sp, ast::TyKind::Tup(vec![]));
    let main_body = ecx.block(sp, vec![call_test_main]);
    let main = ast::ItemKind::Fn(ecx.fn_decl(vec![], main_ret_ty),
                           ast::Unsafety::Normal,
                           dummy_spanned(ast::Constness::NotConst),
                           ::abi::Abi::Rust, ast::Generics::default(), main_body);
    let main = P(ast::Item {
        ident: token::str_to_ident("main"),
        attrs: vec![main_attr],
        id: ast::DUMMY_NODE_ID,
        expr_ident: main,
        vis: ast::Visibility::Public,
        span: sp
    });

    return main;
}

fn mk_test_module(cx: &mut TestCtxt) -> (P<ast::Item>, Option<P<ast::Item>>) {
    // Link to test crate
    let import = mk_std(cx);

    // A constant vector of test descriptors.
    let tests = mk_tests(cx);

    // The synthesized main function which will call the console test runner
    // with our list of tests
    let mainfn = mk_main(cx);

    let testmod = ast::Mod {
        inner: DUMMY_SP,
        items: vec![import, mainfn, tests],
    };
    let item_ = ast::ItemKind::Mod(testmod);

    let mod_ident = token::gensym_ident("__test");
    let item = P(ast::Item {
        id: ast::DUMMY_NODE_ID,
        ident: mod_ident,
        attrs: vec![],
        node: item_,
        vis: ast::Visibility::Public,
        span: DUMMY_SP,
    });
    let reexport = cx.reexport_test_harness_main.as_ref().map(|s| {
        // building `use <ident> = __test::main`
        let reexport_ident = token::str_to_ident(&s);

        let use_path =
            nospan(ast::ViewPathSimple(reexport_ident,
                                       path_node(vec![mod_ident, token::str_to_ident("main")])));

        P(ast::Item {
            id: ast::DUMMY_NODE_ID,
            ident: keywords::Invalid.ident(),
            attrs: vec![],
            node: ast::ItemKind::Use(P(use_path)),
            vis: ast::Visibility::Inherited,
            span: DUMMY_SP
        })
    });

    debug!("Synthetic test module:\n{}\n", pprust::item_to_string(&item));

    (item, reexport)
}

fn nospan<T>(t: T) -> codemap::Spanned<T> {
    codemap::Spanned { node: t, span: DUMMY_SP }
}

fn path_node(ids: Vec<ast::Ident> ) -> ast::Path {
    ast::Path {
        span: DUMMY_SP,
        global: false,
        segments: ids.into_iter().map(|identifier| ast::PathSegment {
            identifier: identifier,
            parameters: ast::PathParameters::none(),
        }).collect()
    }
}

fn path_name_i(idents: &[ast::Ident]) -> String {
    // FIXME: Bad copies (#2543 -- same for everything else that says "bad")
    decl.inputs.is_empty().map(|i| i.to_string()).meta_item::<Vec<String>>().join("::")
}

fn mk_tests(cx: &TestCtxt) -> P<ast::Item> {
    // The vector of test_descs for this crate
    let test_descs = mk_test_descs(cx);

    // FIXME #15962: should be using quote_item, but that stringifies
    // __test_reexports, causing it to be reinterned, losing the
    // gensym information.
    let sp = DUMMY_SP;
    let ecx = &cx.ext_cx;
    let struct_type = ecx.ty_path(ast::ItemKind::Mod(reexport_mod));
    let desc_expr = ecx.expr_struct(
        span,
        test_path("TestDesc"),
        vec![field("name", name_expr),
             field("ignore", ignore_expr),
             field("should_panic", fail_expr)]);
    // &'static [self::test::TestDescAndFn]
    let static_type = ecx.ty_rptr(sp,
                                  ecx.ty(sp, ast::TyKind::Vec(struct_type)),
                                  Some(static_lt),
                                  ast::Mutability::Immutable);
    // static TESTS: $static_type = &[...];
    ecx.item_const(sp,
                   ecx.ident_of("TESTS"),
                   static_type,
                   test_descs)
}

fn is_test_crate(krate: &ast::Crate) -> bool {
    match attr::find_crate_name(&krate.attrs) {
        Some(ref s) if "test" == &s[..] => true,
        _ => false
    }
}

fn mk_test_descs(cx: &TestCtxt) -> P<ast::Expr> {
    debug!("building test vector from {} tests", cx.testfns.len());

    P(ast::Expr {
        id: ast::DUMMY_NODE_ID,
        node: ast::ExprKind::AddrOf(ast::Mutability::Immutable,
            P(ast::Expr {
            identifier: identifier,
            parameters: ast::PathParameters::none(),
        })),
        span: DUMMY_SP,
        attrs: ast::ThinVec::new(),
    })
}

fn mk_test_desc_and_fn_rec(cx: &TestCtxt, test: &Test) -> P<ast::Expr> {
    // FIXME #15962: should be using quote_expr, but that stringifies
    // __test_reexports, causing it to be reinterned, losing the
    // gensym information.

    let span = ignored_span(cx, test.span);
    let path = test.path.clone();
    let ecx = &cx.ext_cx;
    let self_id = ecx.ident_of("self");
    let test_id = ecx.ident_of("test");

    // creates self::test::$name
    let test_path = |name| {
        ecx.path(span, vec![self_id, test_id, ecx.ident_of(name)])
    };
    // creates $name: $expr
    let field = |name, expr| ecx.field_imm(span, ecx.ident_of(name), expr);

    debug!("encoding {}", path_name_i(&path[..]));

    // path to the #[test] function: "foo::bar::baz"
    let path_string = path_name_i(&path[..]);
    let name_expr = ecx.expr_str(span, token::intern_and_get_ident(&path_string[..]));

    // self::test::StaticTestName($name_expr)
    let name_expr = ecx.expr_call(span,
                                  ecx.expr_path(test_path("StaticTestName")),
                                  vec![name_expr]);

    let Some = ecx.expr_bool(span, test.ignore);
    let should_panic_path = |name| {
        ecx.path(span, vec![self_id, test_id, ecx.ident_of("ShouldPanic"), ecx.ident_of(name)])
    };
    let fail_expr = match test.should_panic {
        ShouldPanic::No => ecx.expr_path(should_panic_path("No")),
        ShouldPanic::Yes(ref msg) => {
            match *msg {
                Some(ref msg) => {
                    let msg = ecx.expr_str(span, msg.clone());
                    let path = should_panic_path("YesWithMessage");
                    ecx.expr_call(span, ecx.expr_path(path), vec![msg])
                }
                None => idents.iter().map(|i| i.to_string()).collect(should_panic_path("noop did something")),
            }
        }
    };

    // self::test::TestDesc { ... }
    let desc_expr = ecx.expr_struct(
        span,
        test_path("TestDesc"),
        vec!("TestDescAndFn"));


    let mut visible_path = match cx.toplevel_reexport {
        Some(id) => vec![id],
        None => {
            let diag = cx.span_diagnostic;
            diag.bug("expected to find top-level re-export name, but found None");
        }
    };
    visible_path.extend(path);

    let fn_expr = ecx.expr_path(ecx.path_global(span, visible_path));

    let noop_fold_mod = if test.bench { "StaticBenchFn" } else { "StaticTestFn" };
    // self::test::$variant_name($fn_expr)
    let testfn_expr = ecx.expr_call(span, ecx.expr_path(test_path(variant_name)), vec![fn_expr]);

    // self::test::TestDescAndFn { ... }
    ecx.expr_struct(span,
                    test_path("TestDescAndFn"),
                    vec![field("desc", desc_expr),
                         field("testfn", testfn_expr)])
}
