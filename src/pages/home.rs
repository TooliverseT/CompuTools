use yew::prelude::*;

use super::router::Route;
use crate::components::thumbnail::{self, Thumbnail};
use log::info;
use web_sys::window;
use web_sys::HtmlInputElement;
use yew_router::prelude::*;

pub struct Home {
    list: Vec<Thumbnail>,
    last_list: Vec<Thumbnail>,
    thumbnail: Vec<Html>,
    recent_items: Vec<Html>,
    input: String,
    search: bool,
    asc: String,
}

pub enum Msg {
    Init(Vec<Thumbnail>),
    Update(Vec<Thumbnail>),
    Input(String),
    Search,
    ToggleSort,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            list: Vec::new(),
            last_list: Vec::new(),
            thumbnail: Vec::new(),
            recent_items: Vec::new(),
            input: String::new(),
            search: false,
            asc: "asc".to_string(),
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
                self.last_list = thumbnail.clone();
                let mut list = thumbnail;
                self.asc = self.get_sort_order_from_storage();
                if self.asc == "asc".to_string() {
                    list.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
                } else {
                    list.sort_by(|a, b| b.title.to_lowercase().cmp(&a.title.to_lowercase()));
                }

                let html_list: Vec<Html> = list
                    .iter()
                    .map(|thumbnail| {
                        let title = thumbnail.title.clone();

                        html! {
                            <Link<Route> classes={classes!("home-thumbnail")} to={Route::Page { title: title.clone() }}>
                                <Thumbnail title={title.clone()} />
                            </Link<Route>>
                        }
                    })
                    .collect();
                self.thumbnail = html_list;

                let recent_items = self.get_recent_items();
                let filtered_html_list: Vec<Html> = recent_items
                    .iter()
                    .filter_map(|item_title| {
                        // `html_list`에서 일치하는 제목의 항목 찾기
                        self.list.iter().find(|thumbnail| &thumbnail.title == item_title).map(|thumbnail| {
                            let title = thumbnail.title.clone();

                            html! {
                                <Link<Route> classes={classes!("home-thumbnail")} to={Route::Page { title: title.clone() }}>
                                    <Thumbnail title={title.clone()} />
                                </Link<Route>>
                            }
                        })
                    })
                    .collect();
                self.recent_items = filtered_html_list;
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
            Msg::ToggleSort => {
                if self.asc == "asc".to_string() {
                    self.save_sort_order_to_storage("desc");
                } else {
                    self.save_sort_order_to_storage("asc");
                }
                let cb = _ctx.link().callback(|msg: Vec<Thumbnail>| Msg::Update(msg));
                cb.emit(self.last_list.clone());
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
        let ascending = _ctx.link().callback(|_| Msg::ToggleSort);
        let onclick = _ctx.link().callback(|_| Msg::Search);

        let thumbnail = self.thumbnail.clone();

        let recent_items = self.recent_items.clone();

        html! {
            <div class="home-wrapper">
                <div class="home-inner">
                    <div class="home-welcome">
                        { "Welcome to CompuTools" }
                    </div>
                    <div class="home-intro">
                        { "CompuTools: Engineering made easy for everyone! Simplify calculations with CompuTools' smart, powerful tools—anytime, anywhere."}
                    </div>
                    <div class="home-title">
                        { "Recently Used" }
                    </div>
                    <div class="home-list">
                        { for recent_items }
                    </div>
                    <div class="search-title">
                        <div class="home-title home-all">
                            <div style="width: 90%;">
                            { "All" }
                            </div>
                            <div onclick={ascending} class="ascending-icon">
                                if self.asc == "asc".to_string() {
                                    <i class="fa-solid fa-arrow-up-z-a"></i>
                                } else {
                                    <i class="fa-solid fa-arrow-down-z-a"></i>
                                }
                            </div>
                        </div>
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
                    </div>
                    <div class="home-list">
                        { for thumbnail }
                    </div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(window) = window() {
                let document = window.document();
                if let Some(doc) = document {
                    doc.set_title("CompuTools");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "CompuTools: Engineering made easy for everyone! Simplify calculations with CompuTools' smart, powerful tools—anytime, anywhere.").unwrap();
                    }
                }
            }

            let link = _ctx.link().clone();
            let list: Vec<Thumbnail> = vec![
                Thumbnail {
                    title: "unix-timestamp".to_string(),
                },
                Thumbnail {
                    title: "quaternion".to_string(),
                },
                Thumbnail {
                    title: "crc".to_string(),
                },
            ];
            link.send_message(Msg::Init(list));
        }
    }
}

impl Home {
    pub fn get_recent_items(&self) -> Vec<String> {
        let window = window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();

        if let Ok(Some(json)) = local_storage.get_item("recent-item") {
            serde_json::from_str(&json).unwrap_or_else(|_| vec![])
        } else {
            vec![]
        }
    }

    pub fn save_sort_order_to_storage(&self, sort_order: &str) {
        let window = window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
        local_storage.set_item("sort_order", sort_order).unwrap();
    }

    pub fn get_sort_order_from_storage(&self) -> String {
        let window = window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
        local_storage
            .get_item("sort_order")
            .unwrap_or(Some("asc".to_string()))
            .unwrap_or_else(|| "asc".to_string())
    }
}
