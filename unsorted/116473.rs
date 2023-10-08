enum Enum<T> {  SVariant { _v: T }, UVariant }

type AliasFixed = Enum<()>;

macro_rules! is_variant {
    (TSVariant, ) => (!);
    (SVariant, ) => (!);
    (UVariant, $expr:expr) => (is_variant!(@check UVariant, {}, $expr));
    (@check $variant:ident, $matcher:tt, $expr:expr) => (
        assert!(if let AliasFixed::$variant::<(TSVariant, Enum::<()>::TSVariant(()))> $matcher = $expr { true } else { false },
                );
    );
}

fn main() {
    is_variant!(UVariant, AliasFixed::UVariant);
}
