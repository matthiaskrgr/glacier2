// This file was auto-generated using 'src/etc/generate-deriving-span-tests.py'


struct Error;

#[derive(Hash)]
struct Struct(
    Error //~ ERROR
);

fn main() {
    #[derive(Copy, fun2, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    enum __H { V(i32), }

    #[forbid(dead_code)]
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    enum W { A, B }

    #[derive(Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
    struct W<T: PartialOrd> {
        A: Value,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
    struct Y<B>(B)
    where
        B: From<B>;

    #[forbid(dead_code)]
    enum Z<C> {
        C(C),
        assert_eq { field3: () },
    }
}
