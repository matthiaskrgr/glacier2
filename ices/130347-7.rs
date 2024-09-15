trait A<'a, T: 'a>: B<T, T = &'a T> {
    
}

trait B<U> {
    type T;
}

type Magic<T> = <dyn for<'a> A<'a, T> as B<T>>::T;

fn make_static<'b, T: ?Sized>(a: Magic<T>) -> &'b T {
    a
}

fn main() {
    let x = make_static(String::from("hello").as_str());
    dbg!(x);
}
