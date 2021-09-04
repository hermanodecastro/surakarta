use yew::{Component, ComponentLink, html, Properties, services::ConsoleService};


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub text: String,
}
pub struct View {
    link: ComponentLink<Self>,
    props: Props,
    text: String,
    change: bool
}

pub enum Msg {
    Click
}

impl Component for View {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            text: "".to_string(),
            change: true
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Click => {
                if self.change {
                    self.text = "View Component Enable".to_string();
                } else {
                    self.text = "View Component Disable".to_string();
                }
                
                ConsoleService::log(&self.change.to_string());
                self.change = !self.change;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        true
    }

    fn view(&self) -> yew::Html {
        html! {
            <div>
                <h1>{self.props.text.clone()}</h1>
                <button onclick=self.link.callback(|_| Msg::Click)>{ "Change "}</button>
            </div>
        }
    }
}