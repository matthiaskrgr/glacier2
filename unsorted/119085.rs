struct Test9;

impl Clone for Y {
    fn clone_from(&mut self, other: _) {
        *self = Test9;
    }
}

struct Struct;
trait Trait<T> {}

type Y = impl Trait<_>;

trait Qux {
    type A;

    const D: _ = 42;
}
impl Qux for Struct {}

fn map<T>() -> Qux<T> {
    None
}

const _: Option<_> = map(value);
