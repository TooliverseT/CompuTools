use pages::router::Router;

mod components;
mod pages;
mod tools;

// TODO: 반응형
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Router>::new().render();
}
