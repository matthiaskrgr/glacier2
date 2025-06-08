//@compile-flags: --edition=2024 -Zthreads=8 -Wunused-import-braces -Wunused-lifetimes -Wunused-macro-rules -Wunused-qualifications -Wunused-results -Wvariant-size-differences --crate-type=lib
#![feature(async_drop)]
// This testcase used to ICE in codegen due to inconsistent field reordering
// in the coroutine state, claiming a ZST field was after a non-ZST field,
// while those two fields were at the same offset (which is impossible).

//@ build-pass
//@ edition:2018

async fn foo<F>(_: &(), _: F) {
    async {
        foo(&(), || {}).await;
    };
}

fn main() {
    foo(&(), || {});
    async {};
}
