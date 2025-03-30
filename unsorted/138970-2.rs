#![feature(trivial_bounds)]

trait Bad {
    type Assert
    where
        Self: Sized;
}

impl Bad for [()] {}

fn foo() where [()]: Sized {
    let _: <[()] as Bad>::Assert;
}
