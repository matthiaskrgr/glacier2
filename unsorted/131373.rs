struct Ref<T>(T);

trait Reference: 'static {
    type Ref<'a>;
    fn get_ref(&self) -> Self::Ref<'_>;
}

trait Lock: 'static {
    type Locked<'a>;
    fn locked(&self) -> Self::Locked<'_>;
}

struct SliceRef<'a, T: ?Sized> {
    inner: &'a T
}

impl<'a, 'b, T: ?Sized, SR: Reference> IntoIterator for &'b SliceRef<'a, T> where &'a T: IntoIterator<Item=&'a SR> {
    type Item = SR::Ref<'a>;
    type IntoIter = std::iter::Map<<&'a T as IntoIterator>::IntoIter, for<'c> fn(&'c SR) -> SR::Ref<'c>>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter().map(|cr| { cr.get_ref() })
    }
}

impl<SR: Reference> Reference for Vec<SR> {
    type Ref<'a> = SliceRef<'a, [SR]>;
    fn get_ref(&self) -> Self::Ref<'_> {
        SliceRef {
            inner: &**self,
        }
    }
}

impl<SR: Reference> Lock for Ref<SR> {
    type Locked<'a> = SR::Ref<'a>;
    fn locked(&self) -> Self::Locked<'_> {
        self.0.get_ref()
    }
}

impl Reference for () {
    type Ref<'a> = &'a ();
    fn get_ref(&self) -> Self::Ref<'_> {
        unimplemented!()
    }
}

fn main() {
    let data = Ref(Vec::<()>::new());
    let _ = (&data.locked()).into_iter();
}
