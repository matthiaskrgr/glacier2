fn test() {
    let x = match **x {
        Some(&a) if { panic!() } => {}
    };
    let mut p = &x;

    {
        let mut closure = expect_sig(|p, y| *p = y);
        //~^ ERROR
        closure(&mut p, &y);
    }

    deref(p);
}

fn expect_sig<F>(f: F) -> F
where
    F: FnMut(&mut &i32, &i32),
{
    f
}
