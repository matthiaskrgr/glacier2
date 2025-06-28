fn main() {
    let state = 0;
    #[loop_match]
    loop {
        state = 'blk: {
            match state {
                0 =>
                {
                    #[const_continue]
                    break 'blk 1
                }
            }
        };
    }
}
