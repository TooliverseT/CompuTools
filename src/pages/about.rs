use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::window;
use crate::components::tool_category::ToolCategoryManager;
use crate::pages::router::Route;

pub struct About {}

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // 모든 툴을 가져와서 이름 순으로 정렬
        let mut all_tools = ToolCategoryManager::get_all_tools();
        all_tools.sort_by(|a, b| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()));

        html! {
            <>
                <div class="tool-wrapper ver2">
                    <div>
                        <h1 class="tool-title">
                            { "About CompuTools" }
                        </h1>
                        <div class="tool-intro">
                            <h2>{ "Our Mission" }</h2>
                            <p>
                                { "CompuTools was created with a simple yet powerful vision: to make engineering and computational tasks accessible to everyone. We believe that powerful tools shouldn't be complex to use, and that's why we've built a comprehensive suite of conversion and calculation utilities that are both intuitive and robust." }
                            </p>
                            
                            <h2>{ "What We Offer" }</h2>
                            <p>
                                { "Our platform provides a comprehensive collection of essential tools for developers, engineers, students, and professionals working with data conversion and computational tasks. Each tool is designed with precision, performance, and user experience in mind." }
                            </p>
                            
                            <h3>{ format!("Our {} Tools Include:", all_tools.len()) }</h3>
                            <ul>
                                {
                                    all_tools.iter().map(|tool| {
                                        html! {
                                            <li>
                                                <Link<Route> to={Route::Page { title: tool.route_name.clone() }}>
                                                    <strong>{ &tool.display_name }</strong>
                                                </Link<Route>>
                                                { " - " }
                                                { &tool.description }
                                            </li>
                                        }
                                    }).collect::<Html>()
                                }
                            </ul>
                            
                            <h2>{ "Why Choose CompuTools?" }</h2>
                            <ul>
                                <li><strong>{ "Privacy First" }</strong>{ " - All processing happens locally in your browser. We never store or transmit your data." }</li>
                                <li><strong>{ "Fast & Reliable" }</strong>{ " - Built with Rust and WebAssembly for optimal performance and reliability." }</li>
                                <li><strong>{ "Always Available" }</strong>{ " - No registration required, no downloads needed. Just open and use." }</li>
                                <li><strong>{ "Open Source" }</strong>{ " - Our tools are transparent and continuously improved by the community." }</li>
                                <li><strong>{ "Cross-Platform" }</strong>{ " - Works on any device with a modern web browser." }</li>
                                <li><strong>{ "No Limits" }</strong>{ " - Use our tools as much as you need, completely free." }</li>
                            </ul>
                            
                            <h2>{ "Technology Stack" }</h2>
                            <p>
                                { "CompuTools is built using cutting-edge web technologies to ensure the best possible user experience:" }
                            </p>
                            <ul>
                                <li><strong>{ "Rust" }</strong>{ " - For high-performance, memory-safe computations" }</li>
                                <li><strong>{ "Yew Framework" }</strong>{ " - Modern reactive web framework for Rust" }</li>
                                <li><strong>{ "WebAssembly (WASM)" }</strong>{ " - Near-native performance in the browser" }</li>
                                <li><strong>{ "Progressive Web App (PWA)" }</strong>{ " - For offline access and app-like experience" }</li>
                            </ul>
                            
                            <h2>{ "Our Commitment" }</h2>
                            <p>
                                { "We are committed to continuously improving and expanding CompuTools based on user feedback and emerging needs in the engineering and development communities. Our goal is to become the go-to resource for reliable, fast, and secure computational tools." }
                            </p>
                            
                            <h2>{ "Contact Us" }</h2>
                            <p>
                                { "We value your feedback and suggestions. Whether you've found a bug, have an idea for a new tool, or just want to say hello, we'd love to hear from you:" }
                            </p>
                            <p>
                                <strong>{ "Email: " }</strong>
                                <a href="mailto:tooliverse0520@gmail.com">{ "tooliverse0520@gmail.com" }</a>
                            </p>
                            
                            <p style="margin-top: 40px; font-style: italic; color: #666;">
                                { "Thank you for choosing CompuTools. We're here to make your computational tasks simpler, faster, and more reliable." }
                            </p>
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
                    doc.set_title("About CompuTools - Engineering Made Easy");
                    
                    if let Some(meta_tag) = doc.query_selector("meta[name=\"description\"]").unwrap() {
                        meta_tag.set_attribute("content", "Learn about CompuTools - a comprehensive suite of engineering and computational tools built with Rust and WebAssembly. Privacy-first, fast, and reliable tools for developers and engineers.").unwrap();
                    }
                }
            }
        }
    }
} 