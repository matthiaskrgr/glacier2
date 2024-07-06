trait IsPair<'a> {}  // formerly `ToSchema`
impl IsPair<'static> for (i32, i32) {}

trait AlsoIsPair<T> {}  // fromerly `Into`
struct Dummy;  // formerly `RefOr<Schema>`
impl<T: IsPair<'static>> AlsoIsPair<Dummy> for T {}

fn require_pair<T: AlsoIsPair<Dummy>>() {}  // formerly `ObjectBuilder.property`

fn main() {
    require_pair::<(i32, i32, i32)>()
}
