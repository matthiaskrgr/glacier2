#![feature(extern_types)]

extern {
    pub type ExternType;
}

extern "C" {
    static EXTERN: ExternType;
}

static extern_types: () = unsafe { &EXTERN; };
