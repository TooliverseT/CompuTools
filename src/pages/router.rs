use super::{home::Home, navbar::Navbar, page::Page, about::About, privacy::Privacy, terms::Terms, contact::Contact};
use log::info;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Router {}

impl Component for Router {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Navbar />
                <div class="main-wrapper">
                    <div class="main-content">
                        <Switch<Route> render={switch} />
                    </div>
                </div>
                <footer>
                    <p style="font-size: 0.9em;">
                        <a href="/about/" style="margin-right: 15px;">{"About"}</a>
                        <a href="/privacy/" style="margin-right: 15px;">{"Privacy Policy"}</a>
                        <a href="/terms/" style="margin-right: 15px;">{"Terms of Service"}</a>
                        <a href="/contact/">{"Contact"}</a>
                    </p>
                </footer>
            </BrowserRouter>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::About => {
            html! { <About /> }
        }
        Route::Privacy => {
            html! { <Privacy /> }
        }
        Route::Terms => {
            html! { <Terms /> }
        }
        Route::Contact => {
            html! { <Contact /> }
        }
        Route::Page { title } => {
            html! { <Page title={title.clone()} /> }
        }
        Route::NotFound => {
            html! {
                <>
                    <div class="not-found">
                        {"Page Not Found"}
                    </div>
                    <div class="not-found-contents">
                        { "The page you visited has an invalid or deleted address." }
                    </div>
                </>
            }
        }
    }
}

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about/")]
    About,
    #[at("/privacy/")]
    Privacy,
    #[at("/terms/")]
    Terms,
    #[at("/contact/")]
    Contact,
    #[at("/:title/")]
    Page { title: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
