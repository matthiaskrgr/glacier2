type S2Vec<T> = T5<T>;

trait Bound {
    type Assoc;
}

type T5<U: Bound> = <_ as Bound>::Assoc;

fn main() {}
