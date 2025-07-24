#[derive(PartialEq)]
struct Foo<'a>(&'a ());

#[allow(dead_code)]
struct Dummy<'a>(&'a ());
impl<'a> Dummy<'a> {
    const X: Foo<'a> = Foo(&());
}

fn identity<T>(x: T) -> T { x }

pub fn weird<'a>(_: &'a ()) {
    let Dummy::<'static>::X = { Dummy::<'a>::X };
    let Dummy::<'a>::X = { Dummy::<'static>::X };
    let Dummy::<'static>::X = Dummy::<'a>::X;
    let Dummy::<'a>::X = Dummy::<'static>::X;
    let Dummy::<'static>::X = { Foo(&()) as Foo<'a> };
    let Dummy::<'a>::X = { Foo(&()) as Foo<'static> };
    let Dummy::<'static>::X = identity(Foo(&()) as Foo<'a>);
    let Dummy::<'a>::X = identity(Foo(&()) as Foo<'static>);
    let Dummy::<'static>::X = Foo(&()) as Foo<'a>;
    // For some reason, the line below doesn't compile.
    let Dummy::<'a>::X = Foo(&()) as Foo<'static>;
}
