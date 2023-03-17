// check-pass

trait A {
    type A: B<1i32>;
}

trait B {
    type U<const C: i32>;
}

fn C<T: A>() {
    let _: <<T as B>::T as B>::U<1i32> = ();
}

fn main() {}
