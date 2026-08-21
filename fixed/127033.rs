pub struct Vote;

pub trait RaftLogStorage {
    fn save_vote(&mut self, vote: &Vote) -> impl std::future::Future + Send;
}

struct X;
impl RaftLogStorage for X {
    fn save_vote(&mut self, vote: &Vote) -> impl std::future::Future<Output = Result<(), String>> {
        loop {}
        async {
            vote;
            Ok(())
        }
    }
}
