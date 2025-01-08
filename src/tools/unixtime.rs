use log::info;
use yew::prelude::*;

pub struct Unixtime {}

pub enum Msg {
    Update(String),
}

impl Component for Unixtime {
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
                    <div class="home-intro">
                        { "Unixtime Description" }
                    </div>
                </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {}
}
