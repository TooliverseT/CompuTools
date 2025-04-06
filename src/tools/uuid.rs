use uuid::Uuid;
use yew::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{window, HtmlInputElement, HtmlSelectElement};

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
                <div class="tool-wrapper ver2">
                    <div>
                        <h1 class="tool-title">
                            { "UUID Generator" }
                        </h1>
                        <div class="tool-intro">
                            <div style="display: flex; flex-direction: column; justify-content: center; align-items: center; margin-bottom: 10px;">
                                <div
                                    class="uuid-value"
                                    style="cursor: pointer; user-select: none;"
                                    onclick={ctx.link().callback({
                                        let uuid = self.uuid.clone();
                                        move |_| Msg::CopyToClipboard(uuid.clone())
                                    })}
                                >
                                    { &self.uuid }
                                </div>
                                <button class="tool-btn" style="width: auto; margin-top: 5px; padding-left: 50px; padding-right: 50px;" onclick={ctx.link().callback(|_| Msg::Generate)}>{ "Generate" }</button>
                            </div>
                            <p>
                                { "This tool allows you to generate UUIDs (Universally Unique Identifiers) using version 4, which relies entirely on random numbers to ensure uniqueness." }
                            </p>
                            <p>{ "With this tool, you can:" }</p>
                            <ul>
                                <li>{ "Generate UUID v4 values instantly with a single click." }</li>
                                <li>{ "Copy the generated UUIDs to your clipboard effortlessly." }</li>
                                <li>{ "Use the UUIDs in your applications, databases, or APIs where unique identification is critical." }</li>
                            </ul>
                            <p>
                                { "UUID v4 is one of the most commonly used types because it does not require any input like timestamps or names—just randomness. The chance of collision is astronomically low, making it suitable for most practical applications." }
                            </p>
                            <p>{ "Note:" }</p>
                            <ul>
                                <li>{ "UUIDs are 128-bit numbers displayed as 36-character strings, including hyphens." }</li>
                                <li>{ "All UUIDs generated conform to the RFC 4122 specification for version 4." }</li>
                            </ul>
                            <p>
                                { "This tool is ideal for developers and engineers who need fast, secure, and reliable UUID generation." }
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
