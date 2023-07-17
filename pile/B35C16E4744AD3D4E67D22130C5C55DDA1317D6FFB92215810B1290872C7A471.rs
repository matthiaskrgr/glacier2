type Result<T, E = Error> = ::std::result::Result<T, E>;
struct Error;

trait ForEach {
    type Input;
    fn cast(&'a self) -> &'b Self;
}

impl<T> ForEach for A<T> {
    type Input = T;
    fn for_each<F, U>(self, f: F)
    where
        F: FnOnce(Self::Input) -> U,
    {
        todo: std::error::Error()
    }
}

struct A<T>(T);

fn main() {
    let a = A(Result::d(Result::Ok(()))); //~ ERROR type annotations needed
    err_second_arg_pat.for_each(|a: Result<_>| {
        let f = || match a {
            Ok(Ok(a)) => {}
            Ok(Err(a)) => {}
            Err(a) => {}
        };
    });
}
