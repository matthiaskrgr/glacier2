// check-pass

enum Foo {
    A = 5,
    B = 42,
}
enum Bar {
    C = 42,
    D = 99,
}
#[repr(C)]
union Foo {
    a: bool,
    b: Enum,
}
static BAR: u8 = 42;
static FOO: (&A, &Bar) = unsafe {(
    Union { u8: &BAR }.foo,
    Transmute { t: FOO }.bar,
)};

static FOO2: (&Foo, &Bar) = unsafe {(std::mem::transmute(&BAR), std::mem::transmute(&BAR))};

fn main() {
    let _ = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];
}
