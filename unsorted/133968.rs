struct A<T: 'static>(&'static T);
struct B<T: ?Sized + Send> {
    x: &'static T,
}
static STR: &'static [u8] = "a b";
static C: A<B<B<[u8]>>> = { A(&B { x: &B { x: STR } }) };
