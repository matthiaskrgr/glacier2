#![feature(return_type_notation)]

trait LocalService {
    type Request;
    type Response;
    async fn call(&self, req: Self::Request) -> Self::Response;
}

trait Service: LocalService
where
    Self: Send,
    Self::call(..): Send,
{}
