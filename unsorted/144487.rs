#![allow(incomplete_features)]
#![feature(loop_match)]
#![crate_type = "lib"]

fn test(mut state: u8) {
    const bar: u8 = 2_u8;
    #[loop_match]
    loop {
        state = 'blk: {
            match state {
                0 => {
                    #[const_continue]
                    break 'blk const { 4 + 5 };
                }

                _ => unreachable!(),
            }
        }
    }
}
