use super::{home::Home, navbar::Navbar, page::Page};
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
                    <p>
                        {"Have suggestions or questions? We'd love to hear from you! For feedback or inquiries, please contact us at "}
                        <a href="mailto:tooliverse0520@gmail.com">{"tooliverse0520@gmail.com"}</a>
                        {". We appreciate your input and strive to improve our service!"}
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
    #[at("/:title/")]
    Page { title: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
