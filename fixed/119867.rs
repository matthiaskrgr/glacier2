trait Archive {}

trait Deserialize<T> {
    fn deserialize(&self);
}

struct ArchivedVec<T>();

#[inline]

impl<T: Archive> Deserialize<T> for ArchivedVec {
    fn deserialize(s: _) {}
}
