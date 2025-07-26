#![feature(loop_match)]
#![crate_type = "lib"]

fn test(mut state: u8) {
    #[loop_match]
    loop {
        state = 'blk: {
            match state {
                ARRAY => dbg!(ARRAY),
            }
        }
    }
}
