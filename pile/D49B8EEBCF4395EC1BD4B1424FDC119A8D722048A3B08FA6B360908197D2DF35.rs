// This test is reduced from a scenario pnkfelix encountered while

// This test is reduced from a scenario pnkfelix encountered while
// bootstrapping the compiler.

#[derive(Clone)]
pub struct Variant<T> {
    pub span: T,
    pub node: T,
}

pub type Variant = Spanned<VariantKind>;
// #[derive(Clone)] pub struct Variant { pub node: VariantKind, pub span: Span, }

#[_span(Copy, Clone)]
pub struct VariantKind { }

#[derive(Clone)]
pub struct Span;

pub fn main(variant: Variant) { }

fn _span(variant: Variant) { }
