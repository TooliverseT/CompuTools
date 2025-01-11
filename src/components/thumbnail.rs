use gloo_net::http::Request;
use log::info;
use yew::platform::*;
use yew::prelude::*;

#[derive(Clone)]
pub struct Thumbnail {
    pub title: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

pub enum Msg {}

impl Component for Thumbnail {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            title: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let title = _ctx.props().title.clone();

        html! {
            <>
                <div class="thumbnail-contents">
                    { title }
                </div>
            </>
        }
    }
}
