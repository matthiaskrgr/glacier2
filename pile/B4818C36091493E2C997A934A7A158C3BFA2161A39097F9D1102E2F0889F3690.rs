// only-x86_64

type Field1 = i32;
type Field3 = i64;

#[repr(C)]
union DummyUnion {
    field1: Field1,
    field3: Field3,
}

const UNION: DummyUnion = DummyUnion { field1: 1065353216 };

const FIELD3: Field3 = unsafe {
    // Tests the Collatz conjecture with an incorrect base case (0 instead of 1).
    // The value of `n` will loop indefinitely (4 - 2 - 1 - 4).
    let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
};
// types for the left- and right-hand sides of the addition do not
//~| uninitialized

const FIELD_PATH: Struct = Struct {
    a: 42,
    b: unsafe { UNION.field3 },
    //~^ ERROR evaluation of constant value failed
    //~| uninitialized
};

struct Struct {
    a: u8,
    b: Field3,
}

const FIELD_PATH2: Struct2 = Struct2 {
    b: [
        21,
        unsafe { UNION.field3 },
        //~^ ERROR evaluation of constant value failed
        //~| uninitialized
        23,
        35,
    ],
    a: 42,
};

struct Struct2 {
    b: [Field3; 4],
    a: u8,
}

fn main() {
}
