use yew::prelude::*;

use super::router::Route;
use crate::components::thumbnail::{self, Thumbnail};
use crate::components::tool_category::{ToolCategoryManager, ToolInfo, ToolCategory as ManagedToolCategory};
use web_sys::HtmlInputElement;
use yew_router::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ToolCategory {
    pub name: String,
    pub description: String,
    pub tools: Vec<Thumbnail>,
    pub icon: String,
}

pub struct Home {
    list: Vec<ToolInfo>,
    categories: Vec<ToolCategory>,
    last_list: Vec<ToolInfo>,
    thumbnail: Vec<Html>,
    recent_items: Vec<Html>,
    input: String,
    search: bool,
    asc: String,
    current_category: String,
}

pub enum Msg {
    Init(Vec<ToolInfo>),
    Update(Vec<ToolInfo>),
    Input(String),
    Search,
    ToggleSort,
    SelectCategory(String),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            list: Vec::new(),
            categories: Vec::new(),
            last_list: Vec::new(),
            thumbnail: Vec::new(),
            recent_items: Vec::new(),
            input: String::new(),
            search: false,
            asc: "asc".to_string(),
            current_category: "All".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Init(tools) => {
                self.list = tools;
                self.categories = self.create_categories();
                let cb = _ctx.link().callback(|msg: Vec<ToolInfo>| Msg::Update(msg));
                cb.emit(self.list.clone());
                true
            }
            Msg::Update(tools) => {
                self.last_list = tools.clone();
                let mut list = tools;
                
                // 카테고리 필터링
                if self.current_category != "All" {
                    list = list.into_iter()
                        .filter(|tool| tool.category.display_name() == self.current_category)
                        .collect();
                }
                
                // 정렬 (알파벳 순서만)
                if self.asc == "asc" {
                    list.sort_by(|a, b| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()));
                } else {
                    list.sort_by(|a, b| b.display_name.to_lowercase().cmp(&a.display_name.to_lowercase()));
                }

                let html_list: Vec<Html> = list
                    .iter()
                    .map(|tool| {
                        let route_name = tool.route_name.clone();
                        let display_name = tool.display_name.clone();
                        let description = tool.description.clone();
                        let category = tool.category.display_name().to_string();

                        html! {
                            <Link<Route> classes={classes!("home-thumbnail")} to={Route::Page { title: route_name.clone() }}>
                                <div class="thumbnail-header">
                                    <div class="thumbnail-title">{ display_name }</div>
                                    <div class="thumbnail-category">{ category }</div>
                                </div>
                                <div class="thumbnail-description">{ description }</div>
                            </Link<Route>>
                        }
                    })
                    .collect();
                self.thumbnail = html_list;

                let recent_items = self.get_recent_items();
                let filtered_html_list: Vec<Html> = recent_items
                    .iter()
                    .filter_map(|item_title| {
                        self.list.iter().find(|tool| &tool.route_name == item_title).map(|tool| {
                            let route_name = tool.route_name.clone();
                            let display_name = tool.display_name.clone();
                            let description = tool.description.clone();

                            html! {
                                <Link<Route> classes={classes!("home-thumbnail", "recent")} to={Route::Page { title: route_name.clone() }}>
                                    <div class="thumbnail-header">
                                        <div class="thumbnail-title">{ display_name }</div>
                                        <div class="recent-badge">{ "Recent" }</div>
                                    </div>
                                    <div class="thumbnail-description">{ description }</div>
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
                    let search_results = ToolCategoryManager::search_tools(&self.input);
                    let cb = _ctx.link().callback(|msg: Vec<ToolInfo>| Msg::Update(msg));
                    cb.emit(search_results);
                } else {
                    let cb = _ctx.link().callback(|msg: Vec<ToolInfo>| Msg::Update(msg));
                    cb.emit(self.list.clone());
                }
                true
            }
            Msg::ToggleSort => {
                let new_sort = if self.asc == "asc" { "desc" } else { "asc" };
                self.asc = new_sort.to_string();
                let cb = _ctx.link().callback(|msg: Vec<ToolInfo>| Msg::Update(msg));
                cb.emit(self.last_list.clone());
                true
            }
            Msg::SelectCategory(category) => {
                self.current_category = category;
                let cb = _ctx.link().callback(|msg: Vec<ToolInfo>| Msg::Update(msg));
                cb.emit(self.list.clone());
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
        let onclick = _ctx.link().callback(|_: MouseEvent| Msg::Search);

        let thumbnail = self.thumbnail.clone();
        let recent_items = self.recent_items.clone();
        let sort_icon = if self.asc == "asc" { "fa-arrow-up-z-a" } else { "fa-arrow-down-z-a" };

        html! {
            <>
                <div class="home-wrapper">
                    <div class="home-inner">
                        <h1 class="home-welcome">
                            { "Welcome to CompuTools" }
                        </h1>
                        <div class="home-intro">
                            { "CompuTools: Engineering made easy for everyone! Simplify calculations with CompuTools' smart, powerful tools—anytime, anywhere."}
                        </div>
                        
                        // 카테고리 필터
                        <div class="category-filter">
                            <div class="category-title">{ "Categories" }</div>
                            <div class="category-buttons">
                                {for self.render_category_buttons(_ctx)}
                            </div>
                        </div>
                        
                        if !recent_items.is_empty() {
                            <div class="home-title">
                                { "Recently Used" }
                            </div>
                            <div class="home-list recent-list">
                                { for recent_items }
                            </div>
                        }
                        
                        <div class="tools-header">
                            <div class="home-title home-all">
                                <div style="width: 90%;">
                                    if self.current_category == "All" {
                                        { "All Tools" }
                                    } else {
                                        { format!("{} Tools", self.current_category) }
                                    }
                                </div>
                                <div onclick={ascending} class="ascending-icon" title="Sort by: A-Z">
                                    <i class={format!("fa-solid {}", sort_icon)}></i>
                                </div>
                            </div>
                        </div>
                        <div class="home-list">
                            { for thumbnail }
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(window) = web_sys::window() {
                let document = window.document();
                if let Some(doc) = document {
                    doc.set_title("CompuTools - Engineering Made Easy");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "CompuTools: Engineering made easy for everyone! Comprehensive suite of conversion and calculation tools for developers, engineers, and professionals.").unwrap();
                    }
                }
            }

            let link = _ctx.link().clone();
            let list = ToolCategoryManager::get_all_tools();
            link.send_message(Msg::Init(list));
        }
    }
}

impl Home {
    fn create_categories(&self) -> Vec<ToolCategory> {
        let mut categories = vec![
            ToolCategory {
                name: "All".to_string(),
                description: "All available tools".to_string(),
                tools: vec![],
                icon: "fa-solid fa-th-large".to_string(),
            }
        ];

        // ToolCategoryManager에서 카테고리를 가져와서 추가
        let managed_categories = ToolCategoryManager::get_all_categories();
        for managed_category in managed_categories {
            categories.push(ToolCategory {
                name: managed_category.display_name().to_string(),
                description: managed_category.description().to_string(),
                tools: vec![],
                icon: managed_category.icon().to_string(),
            });
        }

        categories
    }

    fn render_category_buttons(&self, ctx: &Context<Self>) -> Vec<Html> {
        self.categories.iter().map(|category| {
            let category_name = category.name.clone();
            let is_active = self.current_category == category.name;
            let onclick = ctx.link().callback({
                let name = category_name.clone();
                move |_| Msg::SelectCategory(name.clone())
            });

            html! {
                <button 
                    class={classes!("category-btn", if is_active { Some("active") } else { None })}
                    {onclick}
                >
                    <i class={format!("fa-solid {}", category.icon)}></i>
                    <span>{ &category.name }</span>
                </button>
            }
        }).collect()
    }

    pub fn get_recent_items(&self) -> Vec<String> {
        let window = web_sys::window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();

        if let Ok(Some(json)) = local_storage.get_item("recent-item") {
            let mut items: Vec<String> = serde_json::from_str(&json).unwrap_or_else(|_| vec![]);
            items.truncate(8); // 최대 8개까지만 유지
            items
        } else {
            vec![]
        }
    }
}
