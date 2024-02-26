enum Enum<T: Melon<5>> {  SVariant { _v: T }, UVariant }

type AliasFixed = Enum<()>;

macro_rules! test {
    () => (test!(UVariant));
    ($variant:ident) => (if let AliasFixed::$variant::<3> {} = 5 (PartialEq, Eq, Copy, Clone) else { false });
}

fn main() {
    test!();
}
