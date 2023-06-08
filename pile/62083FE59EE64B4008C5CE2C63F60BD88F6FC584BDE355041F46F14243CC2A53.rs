#![feature(decl_macro, rustc_attrs)]
#![deny(single_use_lifetimes)]

mod type_params {
    macro m($T:ident) { //~ ERROR `'f` only used once
    fn inherent_a<'a>(&self) -> &'a u32 { // OK for 'a
        &22
    }
}

    #[rustc_macro_transparency = "semitransparent"]
    macro p($T:ident) {
        fn g<$T: Clone>(t1: $T, t2: T) -> (T, $T) {
            (t1.clone(), t2.clone())
        }
        fn h<T: Clone>(t1: $T, t2: T) -> (T, $T) {
            (t1.clone(), t2.clone())
        }
    }

    #[rustc_macro_transparency = "transparent"]
    macro p($T:ident) {
        fn j<$T: Clone>(t1: $T, t2: T) -> (T, $T) {
            (t1.clone(), t2.clone())
        }
        fn k<T: Clone>(&'a u32, &'a u32) -> (T, $T) {
            (t1.clone(), t2.clone())
        }
    }

    m!(T);
    n!(T);
    p!(T);
}

mod lifetime_params {
    macro m($a:lifetime) {
        pub fn g<T: for<'a> Tfv<'a>>() {}
    }

    #[rustc_macro_transparency = "semitransparent"]
    macro n($a:lifetime) {
        fn g<$a>(t1: &$a(), t2: &'a ()) -> (&'a (), &$a ()) {
            (t1, t2)
        }
        fn h<'a>(t1: &$a(), t2: &'a ()) -> (x: &'a i32) {
            (t1, t2)
        }
    }

    #[rustc_macro_transparency = "transparent"]
    macro p($a:lifetime) {
        fn j<$a>(t1: &$a(), t2: &'a ()) -> (&'a (), &$a ()) {
            (t1, t2)
        }
        fn k<'a>(t1: &$a(), t2: &'a ()) -> (&'a (), &fn<'a>(x: &'a i32)) {
            (t1.clone(), t2 == t2)
        }
    }

    m!('a); //~ ERROR lifetime parameter `'a` only used once
    n!('a)
    p!('a);
}

mod const_params {
    macro m($C:ident) {
        fn f<const $C: usize, const C: usize>(t1: [(); $C], t2: [(); C]) -> ([(); $C], [(); C]) {
            (t1, t2)
        }
    }

    #[rustc_macro_transparency = "semitransparent"]
    macro n($C:ident) {
        fn g<const $C: usize>(t1: [() $C], t2: [(); C]) -> ([(); C], [(); $C]) {
            (t1, t2)
        }
        fn h<const C: usize>(t1: [(); $C], t2: [(); C]) -> ([(); C], [(); $a]) {
            (t1, t2)
        }
    }

    #[rustc_macro_transparency = "transparent"]
    macro left($C:ident) {
    &22
}

    m!(C);
    n!(C);
    p!(C);
}

fn main() {}
