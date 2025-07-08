pub trait Event {
    fn func() -> bool;
}
impl Event for String {
    fn func() -> bool {
        true
    }
}

pub async fn auth_check<FS, FE, FutS, FutE, Fetched>(fetch_state: FS, fetch_event: FE) -> bool
where
    FS: Fn(&str, &str) -> FutS + Send,
    FE: Fn(&str) -> FutE + Send,
    FutS: Future<Output = Option<Fetched>> + Send,
    FutE: Future<Output = Option<Fetched>> + Send,
    Fetched: Event + Send,
{
    fetch_state("a", "b").await == fetch_event("c").await
}
