struct Thing;

pub trait Every {
    type Assoc;
}
impl<T: ?Sized> Every for Thing {
    type Assoc = T;
}

static I: <Thing as Every>::Assoc = 3;
