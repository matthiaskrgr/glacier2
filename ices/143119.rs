enum State2 {
    X,
    Z,
}

fn main() {
    let mut state1;
    let mut state2;
    let mut first;
    'a: loop {
        state1 = 'blk1: {
            match state1 {
                State1A =>
                {
                    #[loop_match]
                    loop {
                        state2 = 'blk2: {
                            match state2 {
                                State2X => {
                                    break 'blk2 {
                                        if first {
                                            'blk2: {
                                                match state2 {
                                                    State2X =>
                                                    {
                                                        #[const_continue]
                                                        break 'blk2 State2::Z
                                                    }
                                                }
                                            };
                                        }
                                        break 'blk2 State2::X;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
