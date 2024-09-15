fn make_static<'a, 'b, T: ?Sized>(x: &'a T) -> &'b T {
    trait A<T>: B<T = T> {}
    
    trait B {
        type T;
    }
    
    struct Erase<T: ?Sized + B>(T::T);

    Erase::<dyn for<'c> A<&'c T>>(x).0
}

fn main() {
    let s = {
        make_static(String::from("hello").as_str())
    };
    
    dbg!(s);
}
