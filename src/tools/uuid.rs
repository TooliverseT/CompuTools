use js_sys;
use uuid::Uuid;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{window, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use crate::components::tool_category::ToolCategoryManager;

pub struct ToolUuid {
    uuid: String,
}

pub enum Msg {
    Generate,
    CopyToClipboard(String),
}

impl Component for ToolUuid {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Generate => {
                self.uuid = Uuid::new_v4().to_string();
                true // re-render
            }
            Msg::CopyToClipboard(value) => {
                // input_ref에서 HtmlInputElement를 가져옴
                if let Some(clipboard) = window().map(|w| w.navigator().clipboard()) {
                    // 클립보드 작업 수행
                    wasm_bindgen_futures::spawn_local(async move {
                        let promise = clipboard.write_text(&value);
                        let future = JsFuture::from(promise);

                        match future.await {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    });
                } else {
                    {};
                }
                false // 리렌더링 필요 없음
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1 class="tool-title">{ "UUID Generator" }</h1>    
                <div class="tool-wrapper">
                    <div class="tool-intro">
                        <div class="content-section">
                            <h2>{"🆔 What is a UUID?"}</h2>
                            <p>{"A UUID (Universally Unique Identifier) is a 128-bit value used to uniquely identify information in computer systems. UUID v4 is generated using random numbers, making collisions extremely unlikely."}</p>
                            <p><strong>{"A typical UUID looks like: "}</strong></p>
                            <ul><li>{"123e4567-e89b-12d3-a456-426614174000"}</li></ul>
                        </div>
                        <div class="content-section">
                            <h2>{"⚙️ How This UUID Generator Works"}</h2>
                            <ul>
                                <li><strong>{"Generate UUID v4:"}</strong> {"Create a new random UUID v4 with a single click."}</li>
                                <li><strong>{"Copy with Notification:"}</strong> {"Click the UUID to copy it to your clipboard with visual feedback."}</li>
                                <li><strong>{"Local Processing:"}</strong> {"All generation and copying happens in your browser for privacy and speed."}</li>
                            </ul>
                        </div>
                        <div class="content-section">
                            <h2>{"📚 Example"}</h2>
                            <div class="example-box">
                                <p><strong>{"Generated UUID v4:"}</strong></p>
                                <ul><li>{"e.g. 550e8400-e29b-41d4-a716-446655440000"}</li></ul>
                            </div>
                        </div>
                        <div class="content-section">
                            <h2>{"💡 Common Use Cases"}</h2>
                            <ul>
                                <li><strong>{"Database Keys:"}</strong> {"Assign unique IDs to database records."}</li>
                                <li><strong>{"API Tokens:"}</strong> {"Generate unique tokens for authentication and session management."}</li>
                                <li><strong>{"Distributed Systems:"}</strong> {"Ensure uniqueness across multiple servers or services."}</li>
                                <li><strong>{"File Names:"}</strong> {"Create unique file names to avoid collisions."}</li>
                            </ul>
                        </div>
                        <div class="content-section">
                            <h2>{"❓ Frequently Asked Questions"}</h2>
                            <div class="faq-item">
                                <h3>{"Q: Are UUIDs really unique?"}</h3>
                                <p>{"A: The probability of generating two identical UUID v4 values is astronomically low, making them unique for all practical purposes."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: What standard does this tool follow?"}</h3>
                                <p>{"A: All UUIDs generated conform to the RFC 4122 specification for version 4."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Can I use this tool offline?"}</h3>
                                <p>{"A: Yes, all generation and copying is performed locally in your browser."}</p>
                            </div>
                        </div>
                        <div class="content-section">
                            <h2>{"🎯 Best Practices"}</h2>
                            <ul>
                                <li><strong>{"Use UUIDs for Uniqueness:"}</strong> {"Use UUIDs when you need a unique identifier that is hard to guess or collide."}</li>
                                <li><strong>{"Store as Strings:"}</strong> {"Store UUIDs as strings in databases for compatibility."}</li>
                                <li><strong>{"Validate Format:"}</strong> {"Ensure UUIDs match the standard 8-4-4-4-12 format."}</li>
                                <li><strong>{"Security Awareness:"}</strong> {"Do not use UUIDs as secrets or passwords."}</li>
                            </ul>
                        </div>
                        <div class="content-section">
                            <h2>{"🔗 Related Tools"}</h2>
                            <ul>
                                {
                                    ToolCategoryManager::get_related_tools("uuid")
                                        .iter()
                                        .map(|tool| {
                                            html! {
                                                <li>
                                                    <a href={format!("/{}/", tool.route_name)}>
                                                        { &tool.display_name }
                                                    </a>
                                                    { " - " }
                                                    { &tool.description }
                                                </li>
                                            }
                                        })
                                        .collect::<Html>()
                                }
                            </ul>
                        </div>
                    </div>
                    <div class="tool-container">
                        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%;">
                            <div
                                class="uuid-value"
                                style="cursor: pointer; user-select: none; font-size: 1.3em; font-weight: bold; margin-bottom: 10px;"
                                onclick={ctx.link().callback({
                                    let uuid = self.uuid.clone();
                                    move |_| Msg::CopyToClipboard(uuid.clone())
                                })}
                            >
                                { &self.uuid }
                            </div>
                            <button class="tool-btn" style="width: auto; margin-top: 5px; padding-left: 50px; padding-right: 50px;" onclick={ctx.link().callback(|_| Msg::Generate)}>{ "Generate" }</button>
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
                    doc.set_title("UUID Generator | CompuTools");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "This tool allows you to generate UUIDs (Universally Unique Identifiers) using version 4, which relies entirely on random numbers to ensure uniqueness. UUID v4 is one of the most commonly used types because it does not require any input like timestamps or names—just randomness. The chance of collision is astronomically low, making it suitable for most practical applications.").unwrap();
                    }
                }
            }
        }
    }
}
