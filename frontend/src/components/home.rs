// use yew::{Component, ComponentLink, InputData, KeyboardEvent, html};

// struct User {
//     name: String
// }

// pub struct Home {
//     user: User,
//     link: ComponentLink<Self>,
//     connected: bool
// }

// pub enum Msg {
//     Connect,
//     UpdateName(InputData),
//     None

// }

// impl Component for Home {
//     type Message = Msg;

//     type Properties = ();

//     fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
//         Self {
//             user: User { name: "".to_string() },
//             link,
//             connected: false
//         }
//     }

//     fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
//         match msg {
//             Msg::Connect => {
//                 true
//             },
//             Msg::UpdateName(event) => {
//                 true
//             },
//             Msg::None => {
//                 false
//             }
//         }
//     }

//     fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
//         todo!()
//     }

//     fn view(&self) -> yew::Html {
//         html! {
//             <>
//                 <input 
//                     oninput=self.link.callback(|event: InputData| Msg::UpdateName(event)) 
//                     onkeypress=self.link.callback(|event: KeyboardEvent| {
//                         if event.key() == "Enter" {
//                             Msg::Connect
//                         } else {
//                             Msg::None
//                         }
//                     })
//                     value=self.user.name.clone()
//                     placeholder="Enter the message" 
//                 />
//                 <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button>
//                 <p>{format!("Status: {}", self.connected.clone())}</p>
//             </>
//     }
// }