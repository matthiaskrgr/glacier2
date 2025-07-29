fn unsupported_type() {
    let mut state = 0;

    'a: loop {
        || {
            #[loop_match]
            'a: loop {
                state = 'blk: {
                    match state {
                        -1 => {
                            #[const_continue]
                            break 'blk 2;
                        }

                        _ => unreachable!("weird value {:?}", state),
                    }
                }
            }

            state
        };
    }
}
