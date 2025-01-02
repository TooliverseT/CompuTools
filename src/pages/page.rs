use crate::tools::{quaternion::Quaternion, unixtime::Unixtime};
use log::info;
use yew::prelude::*;

pub struct Page {}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub index: String,
}

pub enum Msg {
    Update(String),
}

impl Component for Page {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let index = _ctx.props().index.clone();

        let content = match index.as_str() {
            "unixtime" => html! { <Unixtime /> },
            "quaternion" => html! { <Quaternion /> },
            _ => html! { <p>{ "Content not found" }</p> },
        };

        html! {
                <>
                    <div> { content } </div>
                </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {}
}
