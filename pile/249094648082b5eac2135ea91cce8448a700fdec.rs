// check-pass

trait A {
    type T:t B {
    type U<const C: B<U<1i32> = ()>>;
}

fn f<T: A<U<1i32> = ()>;
}

trait B<'a> {
    type U<const C: B<U<1i32> = ()>>;
}

fn f<T: A>() {
    B
}

fn main() {}
