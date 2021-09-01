use serde_derive::Deserialize;
use yew::{Component, ComponentLink, format::{Json, Text}, html, services::{ConsoleService, WebSocketService, websocket::{WebSocketStatus, WebSocketTask}}};

struct Model {
    value: i32,
    link: ComponentLink<Self>,
    ws: Option<WebSocketTask>,
    status: String,
}

#[derive(Deserialize, Debug)]
struct Data {
    value: String,
}

enum Msg {
    Connect,
    Disconnected,
    Update,
    Received(Text),
    Status(WebSocketStatus),
    SendText
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            value: 0,
            link,
            status: String::from(""),
            ws: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Connect => {
                let callback = self.link.callback(|Json(data)| Msg::Received(data));
                let notification = self
                    .link
                    .callback(|status: WebSocketStatus| Msg::Status(status));
                if self.ws.is_none() {
                    let task = WebSocketService::connect(
                        "ws://127.0.0.1:8081/ws/",
                        callback,
                        notification,
                    )
                    .expect("Couldn't connect");
                    self.ws = Some(task);
                }
                true
            }
            Msg::Disconnected => {
                self.status = "Disconnected".to_string();
                self.ws = None;
                true
            },
            Msg::Received(data) => {
                ConsoleService::log(format!("{}", data.expect("Couldn't receive data")).as_str());
                true
            }
            Msg::Update => {
                self.value += 1;
                true
            }
            Msg::Status(WebSocketStatus::Opened) => {
                self.status = "Connected".to_string();
                true
            }
            Msg::Status(WebSocketStatus::Closed) => {
                self.status = "Closed".to_string();
                self.ws = None;
                true
            }
            Msg::Status(WebSocketStatus::Error) => {
                self.status = String::from("Error Connection");
                true
            },
            Msg::SendText => {
                match self.ws {
                    Some(ref mut task) => {
                        task.send(Json(&format!("{}", self.value)));
                        true
                    }
                    None => false,
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        html! {
            <div>
                <h1>{format!("Status: {}", self.status)}</h1>
                <h2>{self.value}</h2>
                <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button>
                <button onclick=self.link.callback(|_| Msg::Update)>{ "Add" }</button>
                <button onclick=self.link.callback(|_| Msg::Disconnected)>{ "Disconect" }</button>
                <button onclick=self.link.callback(|_| Msg::SendText)>{ "Send Text" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
