fn prev<'a>()
where
    [(); { //~ ERROR mismatched types
        let _: &'a (); //~ ERROR generic parameters may not be used in const operations
    }]:
{}

fn lazy_updim<COT>(&self, size: [usize; NEWDIM]) {}
