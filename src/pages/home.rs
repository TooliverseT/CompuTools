use yew::prelude::*;

use super::router::Route;
use crate::components::thumbnail::{self, Thumbnail};
use log::info;
use web_sys::window;
use web_sys::HtmlInputElement;
use yew_router::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ToolCategory {
    pub name: String,
    pub description: String,
    pub tools: Vec<Thumbnail>,
    pub icon: String,
}

#[derive(Clone, PartialEq)]
pub struct EnhancedThumbnail {
    pub title: String,
    pub display_name: String,
    pub description: String,
    pub category: String,
    pub popularity: u32, // 사용 빈도 기반 점수
    pub tags: Vec<String>,
}

pub struct Home {
    list: Vec<EnhancedThumbnail>,
    categories: Vec<ToolCategory>,
    last_list: Vec<EnhancedThumbnail>,
    thumbnail: Vec<Html>,
    recent_items: Vec<Html>,
    input: String,
    search: bool,
    asc: String,
    current_category: String, // 현재 선택된 카테고리
}

pub enum Msg {
    Init(Vec<EnhancedThumbnail>),
    Update(Vec<EnhancedThumbnail>),
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
                let cb = _ctx.link().callback(|msg: Vec<EnhancedThumbnail>| Msg::Update(msg));
                cb.emit(self.list.clone());
                true
            }
            Msg::Update(tools) => {
                self.last_list = tools.clone();
                let mut list = tools;
                self.asc = self.get_sort_order_from_storage();
                
                // 카테고리 필터링
                if self.current_category != "All" {
                    list = list.into_iter()
                        .filter(|tool| tool.category == self.current_category)
                        .collect();
                }
                
                // 정렬
                if self.asc == "asc".to_string() {
                    list.sort_by(|a, b| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()));
                } else if self.asc == "popularity" {
                    list.sort_by(|a, b| b.popularity.cmp(&a.popularity));
                } else {
                    list.sort_by(|a, b| b.display_name.to_lowercase().cmp(&a.display_name.to_lowercase()));
                }

                let html_list: Vec<Html> = list
                    .iter()
                    .map(|tool| {
                        let title = tool.title.clone();
                        let display_name = tool.display_name.clone();
                        let description = tool.description.clone();
                        let category = tool.category.clone();
                        let popularity = tool.popularity;

                        html! {
                            <Link<Route> classes={classes!("home-thumbnail", "enhanced")} to={Route::Page { title: title.clone() }}>
                                <div class="thumbnail-header">
                                    <div class="thumbnail-title">{ display_name }</div>
                                    <div class="thumbnail-category">{ category }</div>
                                </div>
                                <div class="thumbnail-description">{ description }</div>
                                if popularity > 80 {
                                    <div class="popularity-badge popular">{ "Popular" }</div>
                                } else if popularity > 60 {
                                    <div class="popularity-badge">{ "Recommended" }</div>
                                }
                            </Link<Route>>
                        }
                    })
                    .collect();
                self.thumbnail = html_list;

                let recent_items = self.get_recent_items();
                let filtered_html_list: Vec<Html> = recent_items
                    .iter()
                    .filter_map(|item_title| {
                        self.list.iter().find(|tool| &tool.title == item_title).map(|tool| {
                            let title = tool.title.clone();
                            let display_name = tool.display_name.clone();
                            let description = tool.description.clone();

                            html! {
                                <Link<Route> classes={classes!("home-thumbnail", "recent")} to={Route::Page { title: title.clone() }}>
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
                    let input = self.input.to_lowercase();
                    let list: Vec<EnhancedThumbnail> = self
                        .list
                        .clone()
                        .into_iter()
                        .filter(|tool| {
                            let title = tool.display_name.to_lowercase();
                            let description = tool.description.to_lowercase();
                            let tags = tool.tags.join(" ").to_lowercase();
                            title.contains(&input) || description.contains(&input) || tags.contains(&input)
                        })
                        .collect();
                    let cb = _ctx.link().callback(|msg: Vec<EnhancedThumbnail>| Msg::Update(msg));
                    cb.emit(list.clone());
                } else {
                    let cb = _ctx.link().callback(|msg: Vec<EnhancedThumbnail>| Msg::Update(msg));
                    cb.emit(self.list.clone());
                }
                true
            }
            Msg::ToggleSort => {
                let current_sort = self.get_sort_order_from_storage();
                let new_sort = match current_sort.as_str() {
                    "asc" => "desc",
                    "desc" => "popularity",
                    _ => "asc",
                };
                self.save_sort_order_to_storage(new_sort);
                let cb = _ctx.link().callback(|msg: Vec<EnhancedThumbnail>| Msg::Update(msg));
                cb.emit(self.last_list.clone());
                true
            }
            Msg::SelectCategory(category) => {
                self.current_category = category;
                let cb = _ctx.link().callback(|msg: Vec<EnhancedThumbnail>| Msg::Update(msg));
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
        let onclick = _ctx.link().callback(|_| Msg::Search);

        let thumbnail = self.thumbnail.clone();
        let recent_items = self.recent_items.clone();
        let sort_icon = match self.get_sort_order_from_storage().as_str() {
            "asc" => "fa-arrow-up-z-a",
            "desc" => "fa-arrow-down-z-a",
            "popularity" => "fa-fire",
            _ => "fa-arrow-up-z-a",
        };

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
                        
                        <div class="search-title">
                            <div class="home-title home-all">
                                <div style="width: 90%;">
                                    if self.current_category == "All" {
                                        { "All Tools" }
                                    } else {
                                        { format!("{} Tools", self.current_category) }
                                    }
                                </div>
                                <div onclick={ascending} class="ascending-icon" title="Sort by: Name, Popularity">
                                    <i class={format!("fa-solid {}", sort_icon)}></i>
                                </div>
                            </div>
                            <div class="search-outer">
                                <div class="search-inner">
                                    <input
                                        class="search-input"
                                        placeholder="Search tools by name, description, or tags..."
                                        value={self.input.clone()}
                                        {oninput}
                                        {onkeydown}
                                    />
                                    <button class="search-button" {onclick}>
                                        { "SEARCH" }
                                    </button>
                                </div>
                            </div>
                        </div>
                        <div class="home-list enhanced-grid">
                            { for thumbnail }
                        </div>
                    </div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(window) = window() {
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
            let list: Vec<EnhancedThumbnail> = vec![
                EnhancedThumbnail {
                    title: "unix-timestamp".to_string(),
                    display_name: "Unix Timestamp".to_string(),
                    description: "Convert between Unix timestamps and human-readable dates".to_string(),
                    category: "Time & Date".to_string(),
                    popularity: 85,
                    tags: vec!["time", "date", "unix", "timestamp", "convert"].iter().map(|s| s.to_string()).collect(),
                },
                EnhancedThumbnail {
                    title: "quaternion".to_string(),
                    display_name: "Quaternion".to_string(),
                    description: "Convert between quaternions and Euler angles for 3D rotations".to_string(),
                    category: "Mathematical".to_string(),
                    popularity: 45,
                    tags: vec!["quaternion", "euler", "3d", "rotation", "math"].iter().map(|s| s.to_string()).collect(),
                },
                EnhancedThumbnail {
                    title: "crc".to_string(),
                    display_name: "CRC Calculator".to_string(),
                    description: "Generate CRC checksums for data integrity verification".to_string(),
                    category: "Security & Hash".to_string(),
                    popularity: 60,
                    tags: vec!["crc", "checksum", "integrity", "hash", "verify"].iter().map(|s| s.to_string()).collect(),
                },
                EnhancedThumbnail {
                    title: "ascii".to_string(),
                    display_name: "ASCII Converter".to_string(),
                    description: "Convert text to ASCII codes and vice versa".to_string(),
                    category: "Text & Encoding".to_string(),
                    popularity: 75,
                    tags: vec!["ascii", "text", "code", "convert", "character"].iter().map(|s| s.to_string()).collect(),
                },
                EnhancedThumbnail {
                    title: "json".to_string(),
                    display_name: "JSON Formatter".to_string(),
                    description: "Format, validate, and beautify JSON data".to_string(),
                    category: "Text & Encoding".to_string(),
                    popularity: 90,
                    tags: vec!["json", "format", "validate", "beautify", "parse"].iter().map(|s| s.to_string()).collect(),
                },
                EnhancedThumbnail {
                    title: "base64".to_string(),
                    display_name: "Base64 Converter".to_string(),
                    description: "Encode and decode Base64 data for secure transmission".to_string(),
                    category: "Text & Encoding".to_string(),
                    popularity: 80,
                    tags: vec!["base64", "encode", "decode", "transmission", "data"].iter().map(|s| s.to_string()).collect(),
                },
                EnhancedThumbnail {
                    title: "base".to_string(),
                    display_name: "Number Base".to_string(),
                    description: "Convert numbers between different bases (binary, hex, etc.)".to_string(),
                    category: "Mathematical".to_string(),
                    popularity: 70,
                    tags: vec!["base", "binary", "hex", "decimal", "convert"].iter().map(|s| s.to_string()).collect(),
                },
                EnhancedThumbnail {
                    title: "file-hash".to_string(),
                    display_name: "File Hash".to_string(),
                    description: "Calculate MD5, SHA-1, SHA-256, SHA-512 hashes for files".to_string(),
                    category: "Security & Hash".to_string(),
                    popularity: 65,
                    tags: vec!["file", "hash", "md5", "sha", "integrity"].iter().map(|s| s.to_string()).collect(),
                },
                EnhancedThumbnail {
                    title: "html".to_string(),
                    display_name: "HTML Converter".to_string(),
                    description: "Encode and decode HTML entities for web content".to_string(),
                    category: "Text & Encoding".to_string(),
                    popularity: 55,
                    tags: vec!["html", "encode", "decode", "entities", "web"].iter().map(|s| s.to_string()).collect(),
                },
                EnhancedThumbnail {
                    title: "url".to_string(),
                    display_name: "URL Converter".to_string(),
                    description: "Encode and decode URLs for proper web transmission".to_string(),
                    category: "Text & Encoding".to_string(),
                    popularity: 75,
                    tags: vec!["url", "encode", "decode", "web", "transmission"].iter().map(|s| s.to_string()).collect(),
                },
                EnhancedThumbnail {
                    title: "uuid".to_string(),
                    display_name: "UUID Generator".to_string(),
                    description: "Generate version 4 UUIDs for unique identification".to_string(),
                    category: "Generators".to_string(),
                    popularity: 70,
                    tags: vec!["uuid", "generate", "unique", "identifier", "random"].iter().map(|s| s.to_string()).collect(),
                },
            ];
            link.send_message(Msg::Init(list));
        }
    }
}

impl Home {
    fn create_categories(&self) -> Vec<ToolCategory> {
        vec![
            ToolCategory {
                name: "All".to_string(),
                description: "All available tools".to_string(),
                tools: vec![],
                icon: "fa-th-large".to_string(),
            },
            ToolCategory {
                name: "Text & Encoding".to_string(),
                description: "Text conversion and encoding tools".to_string(),
                tools: vec![],
                icon: "fa-file-text".to_string(),
            },
            ToolCategory {
                name: "Security & Hash".to_string(),
                description: "Security and hashing utilities".to_string(),
                tools: vec![],
                icon: "fa-shield-alt".to_string(),
            },
            ToolCategory {
                name: "Mathematical".to_string(),
                description: "Mathematical calculation tools".to_string(),
                tools: vec![],
                icon: "fa-calculator".to_string(),
            },
            ToolCategory {
                name: "Time & Date".to_string(),
                description: "Time and date conversion tools".to_string(),
                tools: vec![],
                icon: "fa-clock".to_string(),
            },
            ToolCategory {
                name: "Generators".to_string(),
                description: "Data generation utilities".to_string(),
                tools: vec![],
                icon: "fa-magic".to_string(),
            },
        ]
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
        let window = window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();

        if let Ok(Some(json)) = local_storage.get_item("recent-item") {
            let mut items: Vec<String> = serde_json::from_str(&json).unwrap_or_else(|_| vec![]);
            items.truncate(6); // 최대 6개까지만 유지
            items
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
