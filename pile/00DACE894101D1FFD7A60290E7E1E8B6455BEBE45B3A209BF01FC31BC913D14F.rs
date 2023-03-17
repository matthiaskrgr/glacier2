// check-pass

trait A {
    type U: B<B<1i32> = ()>;
}

trait B {
    type U<const C: i32>;
}

fn main() {}

fn f<T: A>() {
    let _: <<T as A>::T as B>::U<1i32> = ();
}
