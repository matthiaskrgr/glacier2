#![feature(trivial_bounds)]

trait Project {
    const ASSOC: usize;
}

fn foo()
where
    (): Project,
{
    [(); <() as Project>::ASSOC];
}
