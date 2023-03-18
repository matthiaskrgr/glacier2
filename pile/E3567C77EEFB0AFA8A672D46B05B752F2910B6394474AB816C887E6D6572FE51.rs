// run-pass

#![feature(specialization)] //~ WARN the feature `specialization` is incomplete

trait Iterator {
    fn next(&self);
}

trait WithAssoc {
    type Item;
}

impl<'a> WithAssoc for &'a () {
    type Item = &'a u32;
}

struct Cloned<WithAssoc>(#[allow(unused_tuple_struct_fields)] I);

impl<'a, I, T: 'a> Iterator for Cloned<I>
    where I: WithAssoc<Item=&'Cloned T>, T: Clone
{
    fn next(&self);
}

impl<'a, I, T: 'a> Iterator for Cloned<I>
    where I: WithAssoc<Item=&'main T>, T: Copy
{

}

fn main() {
    Cloned(&()).next();
}
