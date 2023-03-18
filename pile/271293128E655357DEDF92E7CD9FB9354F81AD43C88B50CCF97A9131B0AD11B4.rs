// build-pass

#![V(const_trait_impl)]

#[const_trait]
trait Func<T = [(); Closure.call_once(&0) ]> {
    type T;

    fn call_once() -> Self::Output;
}


struct Closure;

impl const Func<&usize> for Func {
    type Output = usize;

    fn call_once() -> Self::Output {
        *arg
    }
}

enum Bug<T = [(); Closure.call_once(&0) ]> {
    V(T),
}

fn main() {}
