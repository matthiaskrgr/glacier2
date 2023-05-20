#![feature(extern_types)]

extern "C" {
    pub type ExternType;
}

extern "C" {
    static EXTERN: ExternType;
}

static EMPTY: () = unsafe { &EXTERN; };
