#[derive(Copy, Clone)]
struct PGD {
    puds: [Option<PUD>; 512],
}

impl PGD {
    fn new() -> Self {
        PGD { puds: [None; 512] }
    }
}

#[derive(Copy, Clone)]
struct PUD {
    dirs: [Option<DIR>; 512],
}

impl PUD {
    fn new() -> Self {
        PUD { dirs: [None; 512] }
    }
}

#[derive(Copy, Clone)]
struct DIR {
    pts: [Option<PT>; 512],
}

impl DIR {
    fn new() -> Self {
        DIR { pts: [None; 512] }
    }
}

#[derive(Copy, Clone)]
struct PT {
    pages: [u64; 512],
}

impl PT {
    fn new() -> Self {
        PT {
            pages: [u64::MAX; 512],
        }
    }
}
