

#![allow(unused)]
#![warn(clippy::pedantic, clippy::unseparated_literal_suffix)]

struct Test {
    field: u64
}

fn main() {
    let _ = Test {
        #[expect(
        //    clippy::unreadable_literal,
        //    clippy::identity_op,
            clippy::unusual_byte_groupings,
        //    clippy::unseparated_literal_suffix,
        )]
        field: 0x1234123412_341234u64 + 0,
    };
}
