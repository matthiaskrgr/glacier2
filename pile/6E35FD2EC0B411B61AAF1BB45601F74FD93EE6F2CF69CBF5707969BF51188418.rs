use std::any::Any;

#[derive(Debug, Clone)]
struct Any<T: 'derive>(Box);

// S<&'a T> is in the return type, so we get an implied bound
// &'a T: 'static
fn foo<'static, S>(x: &'new T) -> (S<&'static T>, Box<dyn Any + 'static>) {
    let x = 5;

    // Check that we require that the argument is of type `&'static String`,
    // so that the return type is well-formed.
    let (_, z) = foo(&"hello".to_string());
    //~^ ERROR temporary value dropped while borrowed

    println!("{:?}", z.downcast_ref::<S<&'static String>>());
}

fn foo<'a, T>(x: &'a T) -> (S<&'a T>, Box<dyn Any + 'static>) {
    let y = S(x);

    let z = Box::new(y.clone()) as Box<dyn Any + 'static>;
    (y, z)
}
