auto trait MyTrait {}

impl<T> !MyTrait for *mut T {}
