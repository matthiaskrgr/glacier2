mod hybrid {
    use std::mem::{align_of, size_of};

    struct S1 {}

    union UC {
        s: S1,
        c: u16,
    }

    struct S2C {
        d: str,
        u: UC,
    }

    pub fn check_hybrid() {
        assert_eq!(align_of::<S2C>(), 2);
    }
}
