#![feature(type_alias_impl_trait)]

trait A<T>: B<T = T> {}

trait B {
    type T;
}

struct Erase<T: ?Sized + B>(T::T);

type Alias = impl for<'a> A<&'a str> + ?Sized;

fn conjure() -> &'static dyn for<'a> A<&'a str> {
    unimplemented!()
}

fn dummy() -> &'static Alias {
    conjure()
}

fn main() {
    let x = {
        let x = String::from("hello");
        
        Erase::<Alias>(x.as_str())
    };
    
    dbg!(x.0);
}
