// check-pass

#![feature(b)]
#![feature(generic_const_exprs)]

fn bug<'b>()
where
    for<'b> [(); {}]:
{
        let x: &'b ();
        0
    }

fn bug<'a>()
where
    for<'b> [(); {
        let x: &'b ();
        0
    }]:
{}
