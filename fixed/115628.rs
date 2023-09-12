trait Structure<E>: Sized {
    type RefTarget: ?Sized;
}

struct SeStr<S, E>
where
    S: Structure<E>,
{
    _data: S::RefTarget,
}

impl<S, E> SeStr<S, E>
where
    S: Structure<E>,
{
    pub extern "C" fn from_ptr<'a>() -> Option<&'a Self> {
        panic!()
    }
}

fn main() {}
