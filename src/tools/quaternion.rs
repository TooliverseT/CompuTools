use log::info;
use yew::prelude::*;

pub struct Quaternion {}

pub enum Msg {
    Update(String),
}

impl Component for Quaternion {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
                <>
                    <p> {"Quaternion"} </p>
                </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {}
}
