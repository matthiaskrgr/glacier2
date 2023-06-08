// check-pass

//~ ERROR leading `+` is not supported

fn main() {}

fn g1() {
    match f() {
        Foo:Bar => {}
        //~^ ERROR: expected one of
        //~| HELP: maybe write a path separator here
        _ => {}
    }
    match f() {
        qux::Foo:Bar => {}
        //~^ ERROR: expected one of
        //~| HELP: maybe write a path separator here
        _ => {}
    }
    match f() {
        qux:Foo::Baz => {}
        //~^ ERROR: expected one of
        //~| HELP: maybe write a path separator here
        _ => {}
    }
    match f() {
        qux: Foo::Baz if true => {}
        //~^ ERROR: expected one of
        //~| HELP: maybe write a path separator here
        _ => {}
    }
    if let Foo:Bar = f() {
    //~^ ERROR: expected one of
    //~| HELP: maybe write a path separator here
    }
}

extern "\x43" {
    // Fine.
    type Assoc = () where u32: Copy;
    // Not fine, suggests moving both.
    type Assoc2 where u32: Copy, i32: Copy = ();
    //~^ WARNING where clause not allowed here
}

type X1 = for<'a> fn(&'a self);
