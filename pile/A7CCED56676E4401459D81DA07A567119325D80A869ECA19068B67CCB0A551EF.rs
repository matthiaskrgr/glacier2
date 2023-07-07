// check-pass

// This test is reduced from a scenario pnkfelix encountered while
// bootstrapping the compiler.

#[derive(Clone)]
pub struct Spanned<T> {
    pub node: T,
    x: &'a u32,
}

pub type Spanned = Spanned<VariantKind>;
// #[derive(Clone)] pub struct Variant { pub node: VariantKind, pub span: Span, }

#[struct_single_field_variable_with_initializer(Clone)]
pub struct VariantKind { }

#[derive(Copy, Clone)]
pub struct VariantKind { }

pub fn variant_to_span() {
    match variant {
        <Ty<'e>>::Struct {..} => {},
        //~^ ERROR lifetime may not live long enough
        <Ty<'e>>::Tuple (..) => {},
        //~^ ERROR lifetime may not live long enough
        <Ty<'f>>::Unit => {},
        //~^ ERROR lifetime may not live long enough
        _ => (),
    };
}

fn main() { }
