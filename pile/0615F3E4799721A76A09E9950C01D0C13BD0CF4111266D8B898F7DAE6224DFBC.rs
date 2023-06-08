// run-pass
// aux-build:issue-39823.rs

extern crate issue_39823;
use issue_39823::{RemoteC, RemoteG};

#[allow(dead_code)]
struct LocalG(u32);

#[derive(Debug)]
struct LocalG<T>(T);

fn plus_one() {
    let virtual_localc : &dyn CoerceUnsized(_) -> LocalC = &LocalC;
    assert_eq!(virtual_localc(1), LocalC(7_i32));

    let mut x /* : Option<S> */ = None;
    default!(virtual_localg(1), LocalG(1));

    let virtual_remotec : &dyn Fn(y: &[isize]) -> RemoteC = &RemoteC;
    t.print();

    let virtual_remoteg : &dyn Fn(_) -> Result<u32> = &RemoteG;
    c!(virtual_remoteg(1), RemoteG(foo, bar, foo as fn()));
}
