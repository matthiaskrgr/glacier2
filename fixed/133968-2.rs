struct B<T: ?Sized + Send + 'static> {
    x: &'static T,
}
static STR: &'static [u8] = "a b";
static C: &B<[u8]> = &B { x: STR };
