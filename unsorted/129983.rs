trait T {}

struct S;

type TAU = extern "C-cmse-nonsecure-call" fn(ptr: &T);
