use yew::prelude::*;

use super::router::Route;
use crate::components::thumbnail::{self, Thumbnail};
use log::info;
use web_sys::HtmlInputElement;
use yew_router::prelude::*;

pub struct Home {
    list: Vec<Thumbnail>,
    thumbnail: Vec<Html>,
    input: String,
    search: bool,
}

pub enum Msg {
    Init(Vec<Thumbnail>),
    Update(Vec<Thumbnail>),
    Input(String),
    Search,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            list: Vec::new(),
            thumbnail: Vec::new(),
            input: String::new(),
            search: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Init(thumbnail) => {
                self.list = thumbnail;
                let cb = _ctx.link().callback(|msg: Vec<Thumbnail>| Msg::Update(msg));
                cb.emit(self.list.clone());
                true
            }
            Msg::Update(thumbnail) => {
                let list = thumbnail;
                let html_list: Vec<Html> = list
                    .iter()
                    .rev()
                    .map(|thumbnail| {
                        let index = thumbnail.index.clone();
                        let title = thumbnail.title.clone();
                        let description = thumbnail.description.clone();
                        let img = thumbnail.img.clone();
                        html! {
                            <Link<Route> classes={classes!("home-thumbnail")} to={Route::Page { index: index.clone() }}>
                                <Thumbnail title={title.clone()} description={description.clone()} img={img.clone()} />
                            </Link<Route>>
                        }
                    })
                    .collect();
                self.thumbnail = html_list;
                true
            }
            Msg::Input(input) => {
                self.input = input;
                false
            }
            Msg::Search => {
                if !self.input.is_empty() {
                    self.search = true;

                    let input = self.input.to_lowercase();
                    let list: Vec<Thumbnail> = self
                        .list
                        .clone()
                        .into_iter()
                        .filter(|thumbnail| {
                            let title = thumbnail.title.to_lowercase();
                            title.contains(&input)
                        })
                        .collect();
                    let cb = _ctx.link().callback(|msg: Vec<Thumbnail>| Msg::Update(msg));
                    cb.emit(list.clone());
                } else {
                    let cb = _ctx.link().callback(|msg: Vec<Thumbnail>| Msg::Update(msg));
                    cb.emit(self.list.clone());
                }
                true
            }
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let oninput = _ctx.link().callback(|e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            Msg::Input(input.value())
        });
        let onkeydown = _ctx.link().callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                Msg::Search
            } else {
                Msg::Input(e.target_unchecked_into::<HtmlInputElement>().value())
            }
        });
        let onclick = _ctx.link().callback(|_| Msg::Search);

        let thumbnail = self.thumbnail.clone();

        html! {
            <div class="home-wrapper">
                <div class="search-outer">
                    <div class="search-inner">
                        <input
                            // ref={self.input_ref.clone()}
                            class="search-input"
                            placeholder="Search Your Tools"
                            {oninput}
                            {onkeydown}
                        />
                        <button class="search-button" {onclick}>
                            { "SEARCH" }
                        </button>
                    </div>
                </div>
                <div class="home-list">
                    { for thumbnail }
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let link = _ctx.link().clone();
            let list: Vec<Thumbnail> = vec![
                Thumbnail {
                    index: "unixtime".to_string(),            // URL INDEX
                    title: "unixtime".to_string(),            // THUMBNAIL TITLE
                    description: "unixtime".to_string(),      // THUMBNAIL DESCRIPTION
                    img: "/assets/img/biped.png".to_string(), // IMG URL
                },
                Thumbnail {
                    index: "quaternion".to_string(),          // URL INDEX
                    title: "quaternion".to_string(),          // THUMBNAIL TITLE
                    description: "quaternion".to_string(),    // THUMBNAIL DESCRIPTION
                    img: "/assets/img/biped.png".to_string(), // IMG URL
                },
            ];
            link.send_message(Msg::Init(list));
        }
    }
}
