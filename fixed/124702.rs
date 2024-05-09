trait X {}

trait Z {
    type Assoc: Y;
}
struct A<T>(T);

impl<T: X> Z for A<T> {
    type Assoc = T;
}

impl<T> From<<A<A<T>> as Z>::Assoc> for T {}
