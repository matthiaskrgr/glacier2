use std::ops::Index;

impl<T, F, Idx> Index<Idx> for Map<T, Output> {}
