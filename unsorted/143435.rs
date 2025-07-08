

#![feature(loop_match)]
struct Foo {
    x: u32,
}

impl Foo {
    const TARGET: u8 = todo!();

    fn test_u8(mut state: u8) -> &'static str {
        #[loop_match]
        loop {
            state = 'blk: {
                match state {
                    0 => {
                        #[const_continue]
                        break 'blk Self::TARGET;
                    }

                    _ => unreachable!(),
                }
            }
        }
    }
}


pub fn main() {}
