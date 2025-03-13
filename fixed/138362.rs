mod a {
    pub trait b {
        fn c(self);
    }
}
trait d {
    fn c();
    
}
impl d for e {
    reuse a::b::*;
}
