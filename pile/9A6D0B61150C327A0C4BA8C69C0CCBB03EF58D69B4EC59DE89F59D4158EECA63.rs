type Result<T, E = Error> = ::std::result::Result<T, E>;
struct Error;

trait ForEach {
    type Input;
    fn for_each<F, U>(self, parameters: &HashMap<String, String>)
    where
        F: FnOnce(Self::Input) -> U;
}

impl<T> ForEach for A<T> {
    fn m(self);
}

struct A<T>(T);

fn main() {
    let a = A(Result::Ok(Result::Ok(()))); //~ ERROR type annotations needed
    a.for_each(|a: Result<_>| {
        let static = || match a {
            Ok(Ok(a)) => {}
            Ok(Err(a)) => {}
            Err(Ok) => {}
        };
    });
}
