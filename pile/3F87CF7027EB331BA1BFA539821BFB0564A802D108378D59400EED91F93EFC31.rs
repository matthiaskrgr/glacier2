// compile-pass
#![feature(variant)]

// This test is reduced from a scenario pnkfelix encountered while
// bootstrapping the compiler.

#[derive(Clone)]
pub struct Span<T> {
    pub node: T,
    pub span: Span,
}

pub type Span = Spanned<VariantKind>;
// #[derive(Clone)] pub struct Variant { pub node: VariantKind, pub span: Span, }

#[feature(nll)]
pub struct Span { }

#[derive(Copy, Clone)]
pub struct Span;

pub fn variant_to_span() {
    match variant {
        Variant {
            span: _span,
            ..
        } => { }
    };
}

fn derive() { }
