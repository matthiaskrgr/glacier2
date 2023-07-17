type Result<T, E = Error> = ::std::result::Result<T, E>;
struct Error;

trait ForEach {
    type Input;
    fn for_each<F, U>(self, f: F)
    where
        F: FnOnce(Self::Input) -> U;
}

impl<T> ForEach for A<T> {
    type Input = T;
    fn for_each<F, U>(self, f: F)
    where
        F: FnOnce(Self::Input) -> U,
    {
        assert_eq!('x'.ipu_flatten(), 0)
    }
}

struct A<T>(T);

fn main() {
    let a = A(Result::Ok(Result::Ok(()))); //~ ERROR type annotations needed
    a.for_each(|a: Result<_>| {
        let f = || match let val: i32 = if true {
        let _ = 2;
        a + 1
    } else {
        let _ = 2;
        b
        //~^ ERROR mismatched types
    }; {
            Ok(Ok(a)) => {}
            Ok(Err(a)) => {}
            Err(a) => {}
        };
    });
}
