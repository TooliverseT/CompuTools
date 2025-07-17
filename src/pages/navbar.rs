use super::router::Route;
use log::info;
use web_sys::{window, HtmlInputElement};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Navbar {
    show_search_modal: bool,
    search_input: String,
    search_results: Vec<Html>,
    tools_list: Vec<SimpleThumbnail>,
}

pub enum Msg {
    ToggleTheme,
    ToggleSearchModal,
    SearchInput(String),
    Search,
    CloseModal,
}

#[derive(Clone, PartialEq)]
pub struct SimpleThumbnail {
    pub title: String,
    pub display_name: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            show_search_modal: false,
            search_input: String::new(),
            search_results: Vec::new(),
            tools_list: vec![
                SimpleThumbnail {
                    title: "unix-timestamp".to_string(),
                    display_name: "Unix Timestamp".to_string(),
                    description: "Convert between Unix timestamps and human-readable dates".to_string(),
                    category: "Time & Date".to_string(),
                    tags: vec!["time", "date", "unix", "timestamp", "convert"].iter().map(|s| s.to_string()).collect(),
                },
                SimpleThumbnail {
                    title: "quaternion".to_string(),
                    display_name: "Quaternion".to_string(),
                    description: "Convert between quaternions and Euler angles for 3D rotations".to_string(),
                    category: "Mathematical".to_string(),
                    tags: vec!["quaternion", "euler", "3d", "rotation", "math"].iter().map(|s| s.to_string()).collect(),
                },
                SimpleThumbnail {
                    title: "crc".to_string(),
                    display_name: "CRC Calculator".to_string(),
                    description: "Generate CRC checksums for data integrity verification".to_string(),
                    category: "Security & Hash".to_string(),
                    tags: vec!["crc", "checksum", "integrity", "hash", "verify"].iter().map(|s| s.to_string()).collect(),
                },
                SimpleThumbnail {
                    title: "ascii".to_string(),
                    display_name: "ASCII Converter".to_string(),
                    description: "Convert text to ASCII codes and vice versa".to_string(),
                    category: "Text & Encoding".to_string(),
                    tags: vec!["ascii", "text", "code", "convert", "character"].iter().map(|s| s.to_string()).collect(),
                },
                SimpleThumbnail {
                    title: "json".to_string(),
                    display_name: "JSON Formatter".to_string(),
                    description: "Format, validate, and beautify JSON data".to_string(),
                    category: "Text & Encoding".to_string(),
                    tags: vec!["json", "format", "validate", "beautify", "parse"].iter().map(|s| s.to_string()).collect(),
                },
                SimpleThumbnail {
                    title: "base64".to_string(),
                    display_name: "Base64 Converter".to_string(),
                    description: "Encode and decode Base64 data for secure transmission".to_string(),
                    category: "Text & Encoding".to_string(),
                    tags: vec!["base64", "encode", "decode", "transmission", "data"].iter().map(|s| s.to_string()).collect(),
                },
                SimpleThumbnail {
                    title: "base".to_string(),
                    display_name: "Number Base".to_string(),
                    description: "Convert numbers between different bases (binary, hex, etc.)".to_string(),
                    category: "Mathematical".to_string(),
                    tags: vec!["base", "binary", "hex", "decimal", "convert"].iter().map(|s| s.to_string()).collect(),
                },
                SimpleThumbnail {
                    title: "file-hash".to_string(),
                    display_name: "File Hash".to_string(),
                    description: "Calculate MD5, SHA-1, SHA-256, SHA-512 hashes for files".to_string(),
                    category: "Security & Hash".to_string(),
                    tags: vec!["file", "hash", "md5", "sha", "integrity"].iter().map(|s| s.to_string()).collect(),
                },
                SimpleThumbnail {
                    title: "html".to_string(),
                    display_name: "HTML Converter".to_string(),
                    description: "Encode and decode HTML entities for web content".to_string(),
                    category: "Text & Encoding".to_string(),
                    tags: vec!["html", "encode", "decode", "entities", "web"].iter().map(|s| s.to_string()).collect(),
                },
                SimpleThumbnail {
                    title: "url".to_string(),
                    display_name: "URL Converter".to_string(),
                    description: "Encode and decode URLs for proper web transmission".to_string(),
                    category: "Text & Encoding".to_string(),
                    tags: vec!["url", "encode", "decode", "web", "transmission"].iter().map(|s| s.to_string()).collect(),
                },
                SimpleThumbnail {
                    title: "uuid".to_string(),
                    display_name: "UUID Generator".to_string(),
                    description: "Generate version 4 UUIDs for unique identification".to_string(),
                    category: "Generators".to_string(),
                    tags: vec!["uuid", "generate", "unique", "identifier", "random"].iter().map(|s| s.to_string()).collect(),
                },
            ],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleTheme => {
                let mut theme = self.get_theme_from_storage();
                // 테마를 토글하고 저장
                if theme == "light" {
                    theme = "dark".to_string();
                } else {
                    theme = "light".to_string();
                }
                // localStorage에 변경된 테마를 저장
                self.save_theme_to_storage(&theme);
                self.update_html_theme(&theme);
                true
            }
            Msg::ToggleSearchModal => {
                self.show_search_modal = !self.show_search_modal;
                if self.show_search_modal {
                    self.search_input = String::new();
                    self.search_results = Vec::new();
                }
                true
            }
            Msg::SearchInput(input) => {
                self.search_input = input.clone();
                if !input.is_empty() {
                    self.perform_search(&input);
                } else {
                    self.search_results = Vec::new();
                }
                true
            }
            Msg::Search => {
                let search_input = self.search_input.clone();
                if !search_input.is_empty() {
                    self.perform_search(&search_input);
                }
                true
            }
            Msg::CloseModal => {
                self.show_search_modal = false;
                self.search_input = String::new();
                self.search_results = Vec::new();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let theme = self.get_theme_from_storage();
        self.update_html_theme(&theme);

        let search_oninput = _ctx.link().callback(|e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            Msg::SearchInput(input.value())
        });

        let search_onkeydown = _ctx.link().callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                Msg::Search
            } else {
                let input = e.target_unchecked_into::<HtmlInputElement>();
                Msg::SearchInput(input.value())
            }
        });

        let modal_onclick = _ctx.link().callback(|e: MouseEvent| {
            // 모달 배경 클릭시 닫기
            if let Some(target) = e.target() {
                if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                    if element.class_name().contains("search-modal-overlay") {
                        return Msg::CloseModal;
                    }
                }
            }
            Msg::CloseModal // 기본적으로 닫기
        });

        html! {
            <>
                <div class="navbar-wrapper">
                    <div class="navbar-container">
                        <Link<Route> classes={classes!("navbar-content")} to={Route::Home}>
                            { "CompuTools" }
                        </Link<Route>>
                        <div class="navbar-content subtitle" style="justify-content: right;">
                            <Link<Route> classes={classes!("subtitle-content")} to={Route::About}>
                                { "About" }
                            </Link<Route>>
                            <Link<Route> classes={classes!("subtitle-content")} to={Route::Contact}>
                                { "Contact" }
                            </Link<Route>>
                        </div>
                        <div class="navbar-icons">
                            <button class="icon-btn search-btn" onclick={_ctx.link().callback(|_| Msg::ToggleSearchModal)}>
                                <i class="fa-solid fa-search"></i>
                            </button>
                            <button class="icon-btn toggle-btn" onclick={_ctx.link().callback(|_| Msg::ToggleTheme)}>
                                <i class="fa-solid fa-circle-half-stroke"></i>
                            </button>
                        </div>
                    </div>
                </div>

                if self.show_search_modal {
                    <div class="search-modal-overlay" onclick={modal_onclick}>
                        <div class="search-modal" onclick={|e: MouseEvent| e.stop_propagation()}>
                            <div class="search-modal-header">
                                <h3>{ "Search Tools" }</h3>
                                <button class="close-btn" onclick={_ctx.link().callback(|_| Msg::CloseModal)}>
                                    <i class="fa-solid fa-times"></i>
                                </button>
                            </div>
                            <div class="search-modal-body">
                                <div class="search-input-container">
                                    <input
                                        class="modal-search-input"
                                        placeholder="Search tools by name, description, or tags..."
                                        value={self.search_input.clone()}
                                        oninput={search_oninput}
                                        onkeydown={search_onkeydown}
                                        autofocus=true
                                    />
                                </div>
                                { self.render_search_results(_ctx) }
                            </div>
                        </div>
                    </div>
                }
            </>
        }
    }
}

impl Navbar {
    fn perform_search(&mut self, query: &str) {
        let input = query.to_lowercase();
        let filtered_tools: Vec<SimpleThumbnail> = self
            .tools_list
            .iter()
            .filter(|tool| {
                let title = tool.display_name.to_lowercase();
                let description = tool.description.to_lowercase();
                let tags = tool.tags.join(" ").to_lowercase();
                title.contains(&input) || description.contains(&input) || tags.contains(&input)
            })
            .cloned()
            .collect();

        self.search_results = filtered_tools
            .iter()
            .map(|tool| {
                let title = tool.title.clone();
                let display_name = tool.display_name.clone();
                let description = tool.description.clone();
                let category = tool.category.clone();

                html! {
                    <Link<Route> 
                        classes={classes!("search-result-item")} 
                        to={Route::Page { title: title.clone() }}
                    >
                        <div class="result-header">
                            <div class="result-title">{ display_name }</div>
                            <div class="result-category">{ category }</div>
                        </div>
                        <div class="result-description">{ description }</div>
                    </Link<Route>>
                }
            })
            .collect();
    }

    fn render_search_results(&self, ctx: &Context<Self>) -> Html {
        if !self.search_results.is_empty() {
            let results: Vec<Html> = self.tools_list
                .iter()
                .filter(|tool| {
                    let input = self.search_input.to_lowercase();
                    let title = tool.display_name.to_lowercase();
                    let description = tool.description.to_lowercase();
                    let tags = tool.tags.join(" ").to_lowercase();
                    title.contains(&input) || description.contains(&input) || tags.contains(&input)
                })
                .map(|tool| {
                    let title = tool.title.clone();
                    let display_name = tool.display_name.clone();
                    let description = tool.description.clone();
                    let category = tool.category.clone();
                    let close_modal = ctx.link().callback(|_| Msg::CloseModal);

                    html! {
                        <div class="search-result-item" onclick={close_modal}>
                            <Link<Route> 
                                classes={classes!("search-result-link")} 
                                to={Route::Page { title: title.clone() }}
                            >
                                <div class="result-header">
                                    <div class="result-title">{ display_name }</div>
                                    <div class="result-category">{ category }</div>
                                </div>
                                <div class="result-description">{ description }</div>
                            </Link<Route>>
                        </div>
                    }
                })
                .collect();

            html! {
                <div class="search-results">
                    { for results }
                </div>
            }
        } else if !self.search_input.is_empty() {
            html! {
                <div class="no-results">
                    { "No tools found matching your search." }
                </div>
            }
        } else {
            html! {}
        }
    }

    pub fn get_theme_from_storage(&self) -> String {
        // localStorage에서 theme 값을 읽어옴
        let window = window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
        local_storage
            .get_item("data-theme")
            .unwrap_or(Some("light".to_string()))
            .unwrap_or_else(|| "light".to_string())
    }

    pub fn save_theme_to_storage(&self, theme: &str) {
        // localStorage에 theme 값을 저장
        let window = window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
        local_storage.set_item("data-theme", theme).unwrap();
    }

    pub fn update_html_theme(&self, theme: &str) {
        // <html> 요소에 data-theme 속성 설정
        let window = window().unwrap();
        let document = window.document().unwrap();
        let html = document.document_element().unwrap();
        html.set_attribute("data-theme", theme).unwrap();
    }
}
