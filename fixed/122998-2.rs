// cfi ice
const X: for<'b> fn(&'b ()) = |&()| ();
fn main() {
    let dyn_debug = Box::new(X) as Box<fn(&'static ())> as Box<dyn Send>;
    drop(dyn_debug)
}
