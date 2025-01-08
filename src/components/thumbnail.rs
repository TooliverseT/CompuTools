use gloo_net::http::Request;
use log::info;
use yew::platform::*;
use yew::prelude::*;

#[derive(Clone)]
pub struct Thumbnail {
    pub title: String,
    pub description: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub description: String,
}

pub enum Msg {}

impl Component for Thumbnail {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            title: "".to_string(),
            description: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let title = _ctx.props().title.clone();
        let description = _ctx.props().description.clone();

        html! {
            <>
                <div class="thumbnail-contents">
                    <div class="thumbnail-title">
                        { title }
                    </div>
                    <div class="thumbnail-description">
                        { description }
                    </div>
                </div>
            </>
        }
    }
}
