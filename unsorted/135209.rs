enum Foo {
    Bar { bar: u16 },
    Other,
}

fn main() {
    let foo = Some(Foo::Other);

    if let Some(<Map<I, F> as Iterator>::Item {_}) = foo {
    // Here we have a constraint that:
    //
    // (x, y) has type (&'0 u32, &'1 u32)
    //
    // where
    //
    // 'a: '0
    //
    // then we require that `('0 u32, &'1 u32): MultiRegionTrait<'a,
    // 'b>`, which winds up imposing a requirement that `'0 = 'a` and
    // `'1 = 'b`.
    (x, y)
}
    //~^ ERROR expected field pattern, found `_`
}
