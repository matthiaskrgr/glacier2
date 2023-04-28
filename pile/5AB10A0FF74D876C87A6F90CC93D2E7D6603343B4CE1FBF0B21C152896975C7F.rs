// run-pass
// pretty-expanded FIXME #23616

fn foo(_: *const ()) {}

fn main() {
    let a = 3;
    foo(&main as *const _ as *const ());
}
