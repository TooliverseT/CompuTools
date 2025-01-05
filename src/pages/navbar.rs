use super::router::Route;
use log::info;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Navbar;

pub enum Msg {
    ToggleTheme,
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
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
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let theme = self.get_theme_from_storage();
        self.update_html_theme(&theme);

        html! {
            <div class="navbar-wrapper">
                <div class="navbar-container">
                    <Link<Route> classes={classes!("navbar-content")} to={Route::Home}>
                        { "CompuTools" }
                    </Link<Route>>
                    <div>
                    </div>
                    // <div class="navbar-content subtitle" style="justify-content: right;">
                    //     <Link<Route> classes={classes!("subtitle-content")} to={Route::Post}>
                    //         { "POST" }
                    //     </Link<Route>>
                    //     <Link<Route> classes={classes!("subtitle-content")} to={Route::Portfolio}>
                    //         { "PORTFOLIO" }
                    //     </Link<Route>>
                    //     <Link<Route> classes={classes!("subtitle-content")} to={Route::Archive}>
                    //         { "ARCHIVE" }
                    //     </Link<Route>>
                    //     <Link<Route> classes={classes!("subtitle-content")} to={Route::About}>
                    //         { "ABOUT" }
                    //     </Link<Route>>
                    // </div>
                    // <Link<Route> classes={classes!("navbar-content", "icon")} to={Route::Search}>
                        // <i class="fa-solid fa-magnifying-glass"></i>
                    // </Link<Route>>
                    <button class="toggle-btn" onclick={_ctx.link().callback(|_| Msg::ToggleTheme)}>
                        <i class="fa-solid fa-circle-half-stroke"></i>
                    </button>
                </div>
            </div>
        }
    }
}

impl Navbar {
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
