use base64::{decode, encode};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;

pub struct ToolBase64 {
    input_string: String,
    output_base64: String,
    input_base64: String,
    output_string: String,
    convert: bool,
}

pub enum Msg {
    UpdateInput(String),
    UpdateBase64(String),
    Convert,
    CopyToClipboard(String),
}

impl Component for ToolBase64 {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_string: String::new(),
            output_base64: String::new(),
            input_base64: String::new(),
            output_string: String::new(),
            convert: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateInput(value) => {
                self.input_string = value;
                self.output_base64 = encode(&self.input_string);
                true
            }
            Msg::UpdateBase64(value) => {
                self.input_base64 = value;
                self.output_string = match decode(&self.input_base64) {
                    Ok(decoded) => String::from_utf8_lossy(&decoded).to_string(),
                    Err(_) => "Invalid Base64 Format".to_string(),
                };
                true
            }
            Msg::Convert => {
                self.convert = !self.convert;
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
                false // 리렌더링 필요 없음
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let convert = self.convert.clone();
        let on_convert = _ctx.link().callback(|_| Msg::Convert);

        html! {
            <>
                <div class="tool-wrapper">
                    <div>
                        <h1 class="tool-title">
                            { "Base64 Converter" }
                        </h1>
                        <div class="tool-intro">
                            <p>
                                {"This tool converts text to Base64 encoding and vice versa, useful for encoding binary data or ensuring safe text transmission. Base64 is widely used in data storage, cryptography, and web applications."}
                            </p>
                            <p> {"With this tool, you can:"} </p>
                            <ul>
                                <li>{"Encode text to Base64 format."}</li>
                                <li>{"Decode Base64-encoded text back to its original form."}</li>
                            </ul>
                            <p>
                                {"Base64 encoding ensures data remains intact during transmission by converting binary data into a text-based format."}
                            </p>
                            <p>
                                {"Common use cases for Base64 encoding include:"}
                            </p>
                            <ul>
                                <li>{"Embedding image or file data in HTML and CSS."}</li>
                                <li>{"Encoding authentication credentials in HTTP headers."}</li>
                                <li>{"Safely transmitting data over text-based protocols such as email and URLs."}</li>
                            </ul>
                            <p>
                                {"Ensure that your input follows Base64 formatting to achieve accurate conversions."}
                            </p>
                        </div>
                    </div>
                    <div class="tool-container">
                        <div style="display: flex; align-items: center; padding-left: 20px; padding-right: 20px; margin-bottom: 10px; margin-top: 5px;">
                            <div style="width: 90%;">
                                if !convert {
                                    {"Encode Text to Base64"}
                                } else {
                                    {"Decode Base64 to Text"}
                                }
                            </div>
                            <div onclick={on_convert} class="tool-change" style="width: 10%;">
                                <i class="fa-solid fa-arrows-rotate"></i>
                            </div>
                        </div>
                        if !convert {
                            <div class="tool-inner">
                                <div>
                                    <div class="tool-subtitle" style="margin-bottom: 5px;">{ "Text" }</div>
                                    <textarea
                                        type="text"
                                        style="overflow: auto;"
                                        value={self.input_string.clone()}
                                        placeholder={ "Enter text..."}
                                        oninput={_ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::UpdateInput(input.value())
                                        })}
                                    />
                                </div>
                            </div>
                            <div class="tool-inner" style="margin-top: 10px;">
                                <div>
                                    <div class="tool-subtitle">{ "Base64" }</div>
                                    <textarea
                                        type="text"
                                        readonly=true
                                        style="overflow: auto; cursor: pointer;"
                                        value={self.output_base64.clone()}
                                        onclick={_ctx.link().callback(|e: MouseEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::CopyToClipboard(input.value())
                                        })}
                                    />
                                </div>
                            </div>
                        } else {
                            <div class="tool-inner">
                                <div>
                                    <div class="tool-subtitle" style="margin-bottom: 5px;">{ "Base64" }</div>
                                    <textarea
                                        type="text"
                                        style="overflow: auto;"
                                        value={self.input_base64.clone()}
                                        placeholder={ "Enter base64..."}
                                        oninput={_ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::UpdateBase64(input.value())
                                        })}
                                    />
                                </div>
                            </div>
                            <div class="tool-inner" style="margin-top: 10px;">
                                <div>
                                    <div class="tool-subtitle">{ "Text" }</div>
                                    <textarea
                                        type="text"
                                        readonly=true
                                        style="overflow: auto; cursor: pointer;"
                                        value={self.output_string.clone()}
                                        onclick={_ctx.link().callback(|e: MouseEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::CopyToClipboard(input.value())
                                        })}
                                    />
                                </div>
                            </div>
                        }
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
                    doc.set_title("Base64 Converter | CompuTools");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "This tool converts text to Base64 encoding and vice versa, useful for encoding binary data or ensuring safe text transmission. Base64 is widely used in data storage, cryptography, and web applications. Base64 encoding ensures data remains intact during transmission by converting binary data into a text-based format.").unwrap();
                    }
                }
            }
        }
    }
}
