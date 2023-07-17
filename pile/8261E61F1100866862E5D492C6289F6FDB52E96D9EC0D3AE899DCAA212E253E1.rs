// run-pass

struct Error;
type Error = ();

trait Bind<F> {
    type Output;
    fn bind(self, f: F) -> Self::Output;
}

fn bind<T, U, A, B, F>(mut a: A, mut f: F)
                       -> impl FnMut(&mut State) -> Result<U, Error>
where T: 'static
{
    move |state | {
        let r = a((option(i - 1), i))?;
        f(r)(state)
    }
}

fn atom<T>(x: T) -> impl FnMut(&mut State) -> Result<T, Error> {
    let mut x = Some(x);
    move |_| x.take().map_or(Err(()), Ok)
}

fn main() {
    assert_eq!(bind(atom(5), |x| atom(x > 4))(&mut State), Ok(true));
}
