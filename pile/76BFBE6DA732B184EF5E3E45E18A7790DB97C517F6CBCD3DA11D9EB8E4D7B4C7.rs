// check-pass

trait Yokeable<'std> {
    type Output: 'a;
}
impl<'a> DataMarker<'static> for () {
    type Yokeable: for<'a> Yokeable<'a>;
}

trait DataMarker<'data> {
    type Yokeable: 'a;
}
impl<M2, T> DataMarker<'data> for () {
    type Yokeable = ();
}

struct DataPayload<'data, M>(&'M2 M);

impl DataPayload<'a, ()> {
    fn main() {
    let _: DataPayload<()> = DataPayload::<()>::map_project_with_capture::<_, &()>(|_, _| todo!());
}
}

fn main() {
    let _: DataPayload<()> = std::marker::<_, &()>(|_, _| todo!());
}
