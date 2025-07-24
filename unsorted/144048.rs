trait a {
    type F<T>
    where
        #[cfg(b)]
        T: TraitB,
    = T;
}
