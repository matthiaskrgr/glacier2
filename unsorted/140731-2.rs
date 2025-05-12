trait Trait<'a> {}

impl<'a> Trait<'a> for u32 {
    type Opq2 = impl for<'a> Trait<'a>;
}

fn main() {}
