#![feature(trivial_bounds)]

fn return_str()
where
    str: Sized,
{
    [(); { let _a: Option<str> = None; 0 }];
}
