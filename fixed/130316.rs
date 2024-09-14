#![feature(async_closure)]
use iced::{widget::Container, Task};    
use matrix_sdk::Client;
pub type Message = ();

fn main() -> iced::Result {
    iced::application("", App::update, App::view)
        .run_with(App::new)
}
struct App{}
impl App {
    fn new() -> (Self, Task<Message>) { (Self {}, Task::none()) }
    fn update(&mut self, _: Message) -> Task<Message> {
        let name: String = "".into();
        return Task::perform( async move || -> () {
                let client = Client::builder()
                    .server_name_or_homeserver_url(name)
                    .build()
                    .await;
                ()
            } ()
            , |_|()
        );
    }
    fn view(&self) -> iced::Element<Message> {
        Container::new("").into()
    }
}
