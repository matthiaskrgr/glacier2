//@compile-flags: -Zvalidate-mir
#![feature(loop_match)]
#![crate_type = "lib"]

fn const_continue_to_block() -> u8 {
    let state = 0;
    #[loop_match]
    loop {
        state = 'blk: {
            match state {
                _ => 'b: {
                    break 'b 2;
                }
            }
        }
    }
}
