#![feature(adt_const_params)]

    const N_ISLANDS: usize = 4;
    const N_BRIDGES: usize = 7;
    const BRIDGES: [(usize, usize); 7] = [(0, 1), (0, 1), (0, 2), (0, 3), (0, 3), (1, 2), (2, 3)];

    pub type Matrix = [[usize; N_ISLANDS]; N_ISLANDS];

    const EMPTY_MATRIX: Matrix = [[0; N_ISLANDS]; N_ISLANDS];

    const fn to_matrix() -> Matrix {
        EMPTY_MATRIX
    }

    const BRIDGE_MATRIX: [[usize; N_ISLANDS]; N_ISLANDS] = to_matrix();

    pub struct Walk<const CURRENT: usize, const REMAINING: Matrix> {
        _p: (),
    }

    impl Walk<0, BRIDGE_MATRIX> {
        pub const fn new() -> Self {
            Self { _p: () }
        }
    }
