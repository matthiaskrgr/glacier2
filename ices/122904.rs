trait T {}

type Alias<'a> = impl T;

struct S;
impl<'a> T for &'a S {}

fn with_positive(fun: impl Fn(Alias<'_>)) {
    with_positive(|&n| ());
}