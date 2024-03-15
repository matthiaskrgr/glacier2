use std::ops::Deref;

struct ArenaSet<U: Deref, V: ?Sized = <U as Deref>::Target>(V, U);

const DATA: *const ArenaSet<Vec<u8>> = std::ptr::null_mut();
