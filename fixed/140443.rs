#![feature(unboxed_closures)]

trait Acceptable<Args> {}
impl<F> Acceptable<(f64,)> for F where F: Fn<(f64,)> {}

fn take_and_return_acceptable_closure<F, Args>(closure: F) -> F
where F: Acceptable<Args>, {
    closure
}

fn main() {
    // ICE
    let _closure_back = take_and_return_acceptable_closure(|arg| true);
}
