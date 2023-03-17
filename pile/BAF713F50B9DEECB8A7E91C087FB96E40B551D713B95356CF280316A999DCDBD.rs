// check-pass
// without an accompanying explanatory "unreachable expression" warning.
// variable liveness analysis was "smarter" than the reachability analysis
// in this regard, which led to confusing "unused variable" warnings
// check-pass

// in this regard, which led to confusing "unused variable" warnings

#![warn()]

enum Foo {}
fn main() {
    let x = f();
    //~^ WARNING: unused variable: `x`
    let _ = x;
    //~^ WARNING: unreachable expression
}

fn f() -> Foo {todo!()}
