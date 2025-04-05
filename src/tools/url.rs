use yew::prelude::*;
use std::borrow::Cow;
use urlencoding::{encode, decode};
use web_sys::{window, HtmlInputElement};
use wasm_bindgen_futures::JsFuture;
use log::info;

pub struct ToolUrl {
    input_text: String,
    output_text: String,
    mode: String,
}

pub enum Msg {
    UpdateInput(String),
    CopyToClipboard(String),
    UpdateMode(String),
}

impl Component for ToolUrl {
    type Message = Msg;
    type Properties = (); 

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_text: String::new(),
            output_text: String::new(),
            mode: "encode".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateInput(text) => {
                self.input_text = text;

                if self.mode == "encode" {
                    self.output_text = encode(&self.input_text).into_owned();
                } else {
                    self.output_text = decode(&self.input_text).unwrap_or(Cow::from("Decoding Error")).into_owned();
                }
                true
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
                false
            }
            Msg::UpdateMode(mode) => {
                self.mode = mode;
                let cb = _ctx.link().callback(|value| Msg::UpdateInput(value));
                cb.emit(self.input_text.clone());
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="tool-wrapper ver2">
                    <div>
                        <h1 class="tool-title">
                            { "URL Converter" }
                        </h1>
                        <div class="tool-intro">
                            <p>{ "This tool helps you encode and decode URLs effortlessly. URL encoding ensures that special characters are correctly transferred in HTTP requests, while decoding restores them to a human-readable form." }</p>
                            <p>{ "With this tool, you can:" }</p>
                            <ul>
                                <li>{ "Convert characters (such as spaces, symbols, and non-ASCII text) into their percent-encoded form for safe transmission in URLs." }</li>
                                <li>{ "Decode percent-encoded URLs back to their original format for readability or further processing." }</li>
                                <li>{ "Ensure compatibility across browsers, systems, and languages by encoding URLs properly." }</li>
                            </ul>
                            <p>{ "This tool is especially useful for web developers, API integrators, and anyone dealing with URLs containing special characters or international text." }</p>
                            <p>{ "Note: URL encoding uses UTF-8 character encoding by default. Reserved characters like '/' or '?' are encoded to ensure the correct structure and interpretation of URLs." }</p>
                        </div>
                    </div>
                    <div class="tool-container ver2 column">
                        <div>
                            <div style="display: flex; align-items: center; padding-left: 20px; padding-right: 20px; margin-bottom: 10px; margin-top: 5px; padding-top: 5px; padding-bottom: 5px;">
                                <div class="tool-subtitle" style="width: 60%; margin-bottom: 0px;">{ "Input" }</div>
                                <select
                                    id="input-mode-select"
                                    style="width: 40%;"
                                    onchange={_ctx.link().callback(|e: Event| {
                                        let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                        Msg::UpdateMode(value)
                                    })}>
                                    <option value="encode" selected={self.mode == "encode"}>{ "Encode" }</option>
                                    <option value="decode" selected={self.mode == "decode"}>{ "Decode" }</option>
                                </select>
                            </div>
                            <div class="tool-inner">
                                <div>
                                    <textarea
                                        type="text"
                                        style="overflow-y: auto; overflow-x: hidden; height: 250px; white-space: pre-wrap; word-wrap: break-word;"
                                        wrap="off"
                                        value={self.input_text.clone()}
                                        placeholder={"Enter here ..."}
                                        oninput={_ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::UpdateInput(input.value())
                                        })}
                                    />
                                </div>
                            </div>
                        </div>
                        <div>
                            <div style="display: flex; align-items: center; padding-left: 20px; padding-right: 20px; margin-bottom: 10px; margin-top: 5px;">
                                <div class="tool-subtitle">{ "Output" }</div>
                            </div>
                            <div class="tool-inner">
                                <div>
                                    <textarea
                                        type="text"
                                        readonly=true
                                        wrap="off"
                                        style={"cursor: pointer; overflow-y: auto; overflow-x: hidden; height: 250px; white-space: pre-wrap; word-wrap: break-word;"}
                                        value={self.output_text.clone()}
                                        placeholder={"Output here ..."}
                                        onclick={_ctx.link().callback(|e: MouseEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::CopyToClipboard(input.value())
                                        })} />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
