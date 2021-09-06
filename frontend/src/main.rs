use std::fmt::Display;

use anyhow::Error;
use serde_derive::{Deserialize, Serialize};
use yew::{Component, ComponentLink, Html, InputData, KeyboardEvent, format::{Json, Text}, html, services::{
        websocket::{WebSocketStatus, WebSocketTask},
        ConsoleService, WebSocketService,
    }};

mod components;
// use components::{home::Home, view::View};
struct Model {
    user: User,
    data: Data,
    link: ComponentLink<Self>,
    ws: Option<WebSocketTask>,
    status: String,
    html: Vec<Html>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data {
    metadata: String,
    content: String,
}

struct User {
    name: String,
    status: bool
}

enum Msg {
    Connect, //conectar automaticamente?
    Received(Response<Data>),
    Status(WebSocketStatus),
    SendMessage,
    UpdateMessage(InputData),
    UpdateUserName(InputData),
    None
}

type Response<T> = Json<Result<T, Error>>;

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            user: User { name: "".to_string(), status: false },
            data: Data { metadata: "".to_string(), content: "".to_string()},
            link,
            status: "Disconnected".to_string(),
            ws: None,
            html: vec![ html!{} ]
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Connect => {
                let callback = self.link.callback(|data: Response<Data>| Msg::Received(data));
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
            },
            Msg::Received(data) => {
                let data = data.0.expect("Couldn't unpacket data");
                ConsoleService::log(format!("{:?}", data).as_str());
                if data.metadata == "ready".to_string() {
                    self.user.status = true;
                    self.status = "Game begin".to_string();
                    true
                } else if data.metadata == "waiting" {
                    self.status = "Waiting for an oponent...".to_string();
                    true
                } else if data.metadata == "message" {
                    self.html.push(
                        html! {
                            <p>{data.content}</p>
                        }
                    );
                    true
                } else {
                    false
                }
                
            },
            Msg::Status(WebSocketStatus::Opened) => match self.ws {
                Some(ref mut task) => {
                    task.send(Json(&Data {metadata: "username".to_string(), content: self.user.name.clone()}));
                    ConsoleService::log("Connection opened");
                    self.data.content = "".to_string();
                    true
                }
                None => false,
            },
            Msg::Status(WebSocketStatus::Closed) => {
                self.status = "Closed".to_string();
                self.ws = None;
                true
            },
            Msg::Status(WebSocketStatus::Error) => {
                self.status = "Error Connection".to_string();
                true
            },
            Msg::SendMessage => match self.ws {        
                Some(ref mut task) => {
                    task.send(Json(&Data {metadata: "message".to_string(), content: self.data.content.clone()}));
                    self.data.content = "".to_string();
                    true
                }
                None => false,
              
            },
            Msg::UpdateMessage(event) => {
                self.data.content = event.value;
                true
            },
            Msg::UpdateUserName(event) => {
                self.user.name = event.value;
                true
            }
            Msg::None => {
                false
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
                {
                    if self.user.status {
                        html! {
                            <>
                                // <h1>{format!("Status: {}", self.status)}</h1>
                                // <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button>
                                <input 
                                    oninput=self.link.callback(|event: InputData| Msg::UpdateMessage(event)) 
                                    onkeypress=self.link.callback(|event: KeyboardEvent| {
                                        if event.key() == "Enter" {
                                            Msg::SendMessage
                                        } else {
                                            Msg::None
                                        }
                                    })
                                    value=self.data.content.clone()
                                    placeholder="Enter the message" 
                                />
                                <button onclick=self.link.callback(|_| Msg::SendMessage)>{ "Send" }</button>
                                <p>{format!("Status: {}", self.status.clone())}</p>
                                <ul class="item-list">
                                    {for self.html.clone() }
                                </ul>
                            </>
                        }
                    } else {
                        html! {
                            <>
                                <input 
                                    placeholder="Enter your name"
                                    oninput=self.link.callback(|event: InputData| Msg::UpdateUserName(event))
                                    onkeypress=self.link.callback(|event: KeyboardEvent| {
                                        if event.key() == "Enter" {
                                            Msg::Connect
                                        } else {
                                            Msg::None
                                        }
                                    })
                                />
                                <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect"}</button>
                                <p>{format!("Status: {}", self.status.clone())}</p>
                            </>
                        }
                    }
                
                }
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
