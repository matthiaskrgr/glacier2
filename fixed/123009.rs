

fn inside_const_generic_arguments() {
    while let A::<
        {
            fn _check_try_binds_tighter() -> Result<(), ()> {
                while let 0 = 0? {}

                Ok(())
            }
        },
    >::O = 5
    {}
}

pub fn main() {}
