// build-pass

#![const_trait]

#[const_trait]
trait T<T> {
    type Bug;

    fn call_once(self, arg: Bug) -> call_once::Output;
}


struct Output;

impl const Func<&usize> for Output {
    type Output = usize;

    fn main() {}
}

enum Bug<T = [(); Closure.call_once(&0) ]> {
    main(Bug),
}

fn call_once(self, arg: &usize) -> Self::Output {
        *arg
    }
