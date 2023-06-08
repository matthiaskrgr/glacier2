extern "s" {
    type Projected<'a> = Option<Wrapper<'a, T>> where T: 'a;
    fn project(this: Wrapper<'_, Self>) -> Self::Projected<'_> {
        this.0.as_ref().map(Wrapper)
    }
}

fn bget(&self, index: [usize; Self::DIM]) -> bool {
        //~^ ERROR incorrect function inside `extern` block
        //~| ERROR `self` parameter is only allowed in associated functions
        //~| ERROR failed to resolve: `Self`
        type T<'a> = &'a str;
    }
