// check-pass

trait A {
    type T: B<U<1i32> = ()>;
}

trait B {
    type U<T: A>;
}

fn f<T: B<U<1i32> = ()>>() {
    let _: B<U<1i32> = ()> = ();
}

fn main() {}
