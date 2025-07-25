use super::router::Route;
use crate::components::tool_category::ToolCategoryManager;
use log::info;
use web_sys::{window, HtmlInputElement};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Navbar {
    show_search_modal: bool,
    search_input: String,
    search_results: Vec<Html>,
}

pub enum Msg {
    ToggleTheme,
    ToggleSearchModal,
    SearchInput(String),
    Search,
    CloseModal,
    ScrollToTop, // 새로운 메시지 추가
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
            Msg::ScrollToTop => {
                if let Some(window) = window() {
                    // 더 안정적인 스크롤 방법 사용
                    let _ = window.scroll_to_with_x_and_y(0.0, 0.0);
                    
                    // 또는 document의 scrollIntoView 사용
                    if let Some(document) = window.document() {
                        if let Some(body) = document.body() {
                            let _ = body.scroll_into_view();
                        }
                    }
                }
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

                // 새로운 플로팅 아이콘들 (오른쪽 하단 고정)
                <div class="floating-icons">
                    <button class="floating-icon-btn search-btn" onclick={_ctx.link().callback(|_| Msg::ToggleSearchModal)} title="Search Tools">
                        <i class="fa-solid fa-search"></i>
                    </button>
                    <button class="floating-icon-btn toggle-btn" onclick={_ctx.link().callback(|_| Msg::ToggleTheme)} title="Toggle Theme">
                        <i class="fa-solid fa-circle-half-stroke"></i>
                    </button>
                    <button class="floating-icon-btn scroll-btn" onclick={_ctx.link().callback(|_| Msg::ScrollToTop)} title="Scroll to Top">
                        <i class="fa-solid fa-angle-up"></i>
                    </button>
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
        let search_results = ToolCategoryManager::search_tools(&input);
        
        self.search_results = search_results
            .iter()
            .map(|tool| {
                let route_name = tool.route_name.clone();
                let display_name = tool.display_name.clone();
                let description = tool.description.clone();
                let category = tool.category.display_name().to_string();

                html! {
                    <Link<Route> 
                        classes={classes!("search-result-item")} 
                        to={Route::Page { title: route_name.clone() }}
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
            let search_results = ToolCategoryManager::search_tools(&self.search_input);
            let results: Vec<Html> = search_results
                .iter()
                .map(|tool| {
                    let route_name = tool.route_name.clone();
                    let display_name = tool.display_name.clone();
                    let description = tool.description.clone();
                    let category = tool.category.display_name().to_string();
                    let close_modal = ctx.link().callback(|_| Msg::CloseModal);

                    html! {
                        <div class="search-result-item" onclick={close_modal}>
                            <Link<Route> 
                                classes={classes!("search-result-link")} 
                                to={Route::Page { title: route_name.clone() }}
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
