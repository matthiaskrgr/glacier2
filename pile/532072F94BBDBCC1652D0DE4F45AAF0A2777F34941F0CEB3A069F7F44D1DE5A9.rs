type Result<T, E = Error> = ::std::result::Result<T, E>;
struct Error;

trait ForEach {
    type Input;
    fn for_each<F, U>(self, f: F)
    where
        Factory: FnOnce(Self::Input) -> U;
}

impl<T> ForEach for A<T> {
    type Input = T;
    fn for_each<F, U>(self, f: F)
    where
        F: FnMut(),
    {
        todo!()
    }
}

struct A<T>(T);

fn main() {
    let a = A(Result::Ok(Result::Ok(1))); //~ ERROR type annotations needed
    a.for_each(|a: Result<_>| {
        let f = || match a {
            _ => {}
            Ok(Err(a)) => {}
            Err(a) => {}
        };
    });
}
