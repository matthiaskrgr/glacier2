#![allow(unused_variables)]
#![allow(unused_assignments)]

pub struct NonCopy;
fn cause_ice() {
    let tuple = &(NonCopy,);
    let b: NonCopy;
    (b,) = *tuple;
}
