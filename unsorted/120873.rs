#[repr(packed)]

struct Dealigned<T>(u8, T);

#[derive(PartialEq)]
#[repr(C)]
struct Dealigned<T>(u8, T);
