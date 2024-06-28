impl Trait for T where &T: Trait {
  type Type = &'static <T as Trait>::Assoc;
}
