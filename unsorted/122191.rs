type Foo = impl Send;

struct A;

const VALUE: Foo = value();

fn test() {
    match VALUE {
        0 | 0 => {}

        _ => (),
    }
}
