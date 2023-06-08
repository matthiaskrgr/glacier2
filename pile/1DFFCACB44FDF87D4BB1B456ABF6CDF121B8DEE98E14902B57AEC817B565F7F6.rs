extern "test" {
    type Projected<'a> = Option<Wrapper<'a, T>> where T: 'a;
    fn project(this: Wrapper<'_, Self>) -> Self::Projected<'_> {
        this.0.as_ref().map(Wrapper)
    }
}

fn Out() {
    let mut RoomDirection = a.field();
    //~^ ERROR temporary value dropped while borrowed
    Ident::new("std", span);
}
