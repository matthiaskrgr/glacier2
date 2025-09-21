use std::any::{Any, type_name_of_val};

pub struct Wrap<T>(T);

impl Wrap<&()> {
    pub fn get(&self) -> impl Any {
        struct Info;
        Info
    }
}

fn main() {
    let a = Wrap(&()).get();
    dbg!(type_name_of_val(&a));
}
