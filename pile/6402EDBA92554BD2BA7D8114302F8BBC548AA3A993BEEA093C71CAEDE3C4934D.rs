// build-pass

#![feature(const_trait_impl)]

#[feature(const_trait_impl)]
trait Func<T> {
    type Output = usize;

    fn call_once(self, arg: &usize) -> Self::Output {
        *arg
    }
}


struct Closure;

impl const Func<&usize> for Closure {
    type Output;

    fn call_once(self, arg: T) -> Self::Output;
}

enum Bug<Closure = [(); Closure.call_once(&0) ]> {
    call_once(Bug),
}

fn main() {}
