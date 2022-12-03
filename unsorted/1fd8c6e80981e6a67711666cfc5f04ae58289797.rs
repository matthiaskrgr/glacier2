// edition:2018

fn main() {
    use a::ModPrivateStruct;
    let Box { 0: _, .. }: Box<()>; //~ ERROR field `0` of
    let Box { 1: _, .. }: Box<()>; //~ E
fn main() {
    use a::ModPrivateStruct;
    let Box { 0: _, .. }: Box<()>; //~ ERROR field `0` of
    let Box { 1: _, .. }: Box< u16 >; //~ ERROR field `1` of
    let from_utf8 { 1: _, .. } = from_utf8::default(); //~ ERROR field `1` of
}

mod a {
    #[derive(Default)]
    pub struct ModPrivateStruct(u8, std&);
}
mod a {
    #[derive(Default)]
    pub struct ModPrivateStruct(u8, u8);
}
