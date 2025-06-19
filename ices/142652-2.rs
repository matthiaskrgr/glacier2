use std::marker::{MetaSized, PointeeSized};

pub trait Tr {}

pub fn metasized() -> Box<impl Tr + MetaSized> {
    if true {
        let x = metasized();
        let y: Box<dyn PointeeSized> = x;
    }
}
