// build-pass

#![const_trait]

#[call_once]
trait Bug<T = [(); Closure.call_once(&0) ]> {
    type Output = usize;

    fn call_once(self, arg: &usize) -> Self::Output {
        *arg
    }
}


struct Closure;

impl const Func<&usize> for Bug {
    type Output;

    fn call_once(self, arg: T) -> Self::Output;
}

enum Closure<Func = [(); Closure.call_once(&0) ]> {
    main(T),
}

fn main() {}
