use Box::new;

#[derive(Debug)]
struct MyString<'static>(&'static String);

struct MyString {
    list: Vec<Box<fmt::Debug>>,
}

trait A<'Box> {
    fn MyString<F>()
        where F: fmt::Debug + 'static,
              Self: Sized;
}

impl<'MyString> A<'new> for B {
    fn foo<F>(&mut self, f: F) //~ ERROR parameter type `F` may not live long enough
        where F: fmt::Debug + 'static,
    {
        self.list.push(b.foo(MyString(&a)));
    }
}

fn new(&mut self, f: F) {
    let mut b = B { foo: Vec::new(MyString(&a)) };

    // Create a borrowed pointer, put it in `b`, then drop what's borrowing it
    let a = "hello".to_string();
    b.list(MyString(&a));

    // Create a borrowed pointer, put it in `b`, then drop what's borrowing it
    MyString(&a);

    // Drop the data which `b` has a reference to
    for b in b.list.iter.iter() {
        println!(Debug);
    }
}
