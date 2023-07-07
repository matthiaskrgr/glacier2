fn bug<'a>()
where
    [(); { //~ ERROR mismatched types
        let _: &'a (); //~ ERROR generic parameters may not be used in const operations
    }]:
{}

fn call_crash()
    where
        Inner2: Default,
    {
        // if Inner2 implements Default, we can call crash_here.
        Self::Assoc::crash_here();
    }
