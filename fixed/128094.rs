pub enum Request {
    TestSome(T),
}

pub async fn handle_event(event: Request) {
    async move {
        static instance: Request = Request { bar: 17 };
        &instance
    }
    .await;
}
