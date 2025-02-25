trait b {
    type c<'a>;
    fn d<e>() -> f<e> {
        todo!()
    }
}
struct f<g> {
    h: Option<<g as b>::c>,
}
