use serde_derive::Deserialize;
use yew::{Component, ComponentLink, Html, InputData, format::{Json, Text}, html, services::{
        websocket::{WebSocketStatus, WebSocketTask},
        ConsoleService, WebSocketService,
    }};

mod view;
use view::View;
struct Model {
    data: Data,
    link: ComponentLink<Self>,
    ws: Option<WebSocketTask>,
    status: String,
    html: Vec<Html>
}

#[derive(Deserialize, Debug)]
struct Data {
    message: String,
}

enum Msg {
    Connect, //conectar automaticamente?
    Disconnected,
    Received(Text),
    Status(WebSocketStatus),
    SendMessage,
    UpdateMessage(String),
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            data: Data { message: String::from("") },
            link,
            status: String::from(""),
            ws: None,
            html: vec![ html!{} ]
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
            }
            Msg::Received(data) => {
                ConsoleService::log(
                    format!("Received: {}", data.expect("Couldn't receive data")).as_str(),
                );
                self.html.push(
                    html! {
                        <p>{self.data.message.clone()}</p>
                    }
                );
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
                self.status = "Error Connection".to_string();
                true
            }
            Msg::SendMessage => match self.ws {
                Some(ref mut task) => {
                    task.send(Json(&format!("{}", self.data.message)));
                    true
                }
                None => false,
            },
            Msg::UpdateMessage(message) => {
                self.data.message = message;
                true
            }
        }
    }

    // Ótima referencia sobre yew: https://dev.to/rusty_sys_dev/understanding-yew-part-1-3cfn

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        html! {
            <div>
                <h1>{format!("Status: {}", self.status)}</h1>
                <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button>
                <input oninput=self.link.callback(|event: InputData| Msg::UpdateMessage(event.value)) placeholder="Message" />
                <button onclick=self.link.callback(|_| Msg::SendMessage)>{ "Send" }</button>
                <ul class="item-list">
                    // { for self.props.items.iter().map(renderItem) }
                    {for self.html.clone() }
                </ul>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
