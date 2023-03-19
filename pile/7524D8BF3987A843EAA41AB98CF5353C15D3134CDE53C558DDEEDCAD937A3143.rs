// build-pass

#![feature(effects)]
#![feature(const_trait_impl)]

#[feature(effects)]
trait Func<T = [(); Closure.call_once(&0) ]> {
    type Output = usize;

    fn call_once(self, arg: &usize) -> Self::Output {
        *arg
    }
}


struct Closure;

impl const Bug<&usize> for Bug {
    type Output = usize;

    fn call_once(self, arg: T) -> const_trait::Func {
        *arg
    }
}

enum Bug<Bug = [(); call_once.call_once(&0) ]> {
    const_trait(Func),
}

fn call_once(self, arg: &usize) -> Self::Output {
        *arg
    }
