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
                <h1 class="tool-title">{ "URL Converter" }</h1>
                <div class="tool-wrapper">
                    <div class="tool-intro">
                        <div class="content-section">
                            <h2>{"🔗 What is URL Encoding?"}</h2>
                            <p>{"URL encoding (percent encoding) is a method to encode special characters in URLs so they can be safely transmitted over the internet. It replaces unsafe ASCII characters with a '%' followed by two hexadecimal digits."}</p>
                            <p>{"For example, a space character is encoded as '%20'."}</p>
                        </div>
                        <div class="content-section">
                            <h2>{"⚙️ How This URL Converter Works"}</h2>
                            <ul>
                                <li><strong>{"Encode:"}</strong> {"Convert characters (such as spaces, symbols, and non-ASCII text) into their percent-encoded form for safe transmission in URLs."}</li>
                                <li><strong>{"Decode:"}</strong> {"Restore percent-encoded URLs back to their original format for readability or further processing."}</li>
                                <li><strong>{"UTF-8 Support:"}</strong> {"All encoding/decoding uses UTF-8 by default."}</li>
                                <li><strong>{"Copy with Notification:"}</strong> {"Click any output field to copy results with visual feedback."}</li>
                                <li><strong>{"Local Processing:"}</strong> {"All conversions happen in your browser for privacy and speed."}</li>
                            </ul>
                        </div>
                        <div class="content-section">
                            <h2>{"📚 Example"}</h2>
                            <div class="example-box">
                                <p><strong>{"Input:"}</strong></p>
                                <ul><li>{"Hello, world! こんにちは"}</li></ul>
                                <p><strong>{"Encoded output:"}</strong></p>
                                <pre style="color: var(--color-font); white-space: pre; font-family: inherit; margin: 0; padding-left: 40px;">
{r#"Hello%2C%20world%21%20%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF"#}
                                </pre>
                            </div>
                        </div>
                        <div class="content-section">
                            <h2>{"💡 Common Use Cases"}</h2>
                            <ul>
                                <li><strong>{"Web Development:"}</strong> {"Encode URLs for HTTP requests, query strings, and form data."}</li>
                                <li><strong>{"API Integration:"}</strong> {"Safely transmit data containing special characters via APIs."}</li>
                                <li><strong>{"Internationalization:"}</strong> {"Handle non-ASCII characters in URLs for global applications."}</li>
                                <li><strong>{"Debugging:"}</strong> {"Decode URLs to inspect or modify their contents."}</li>
                            </ul>
                        </div>
                        <div class="content-section">
                            <h2>{"❓ Frequently Asked Questions"}</h2>
                            <div class="faq-item">
                                <h3>{"Q: What characters are encoded?"}</h3>
                                <p>{"A: All non-ASCII and reserved characters (such as spaces, /, ?, &, =) are percent-encoded."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Is this tool safe for sensitive data?"}</h3>
                                <p>{"A: Yes, all processing is done locally in your browser. No data is sent to any server."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Can I use this tool offline?"}</h3>
                                <p>{"A: Yes, it works entirely in your browser without an internet connection."}</p>
                            </div>
                        </div>
                        <div class="content-section">
                            <h2>{"🎯 Best Practices"}</h2>
                            <ul>
                                <li><strong>{"Always Encode URLs:"}</strong> {"Encode all URLs before sending them in HTTP requests or storing them in databases."}</li>
                                <li><strong>{"Decode for Readability:"}</strong> {"Decode URLs before displaying them to users or for debugging."}</li>
                                <li><strong>{"Test with Edge Cases:"}</strong> {"Test with special characters, Unicode, and long URLs."}</li>
                                <li><strong>{"Security Awareness:"}</strong> {"Never trust unvalidated URLs from untrusted sources."}</li>
                            </ul>
                        </div>
                        <div class="content-section">
                            <h2>{"🔗 Related Tools"}</h2>
                            <p>{"Explore more tools for developers:"}</p>
                            <ul>
                                <li><a href="/ascii/">{"ASCII Converter"}</a> {" - For converting text to ASCII codes and vice versa."}</li>
                                <li><a href="/base64/">{"Base64 Converter"}</a> {" - For encoding and decoding data in Base64 format."}</li>
                                <li><a href="/html/">{"HTML Entity Converter"}</a> {" - For encoding and decoding HTML entities."}</li>
                                <li><a href="/json/">{"JSON Formatter"}</a> {" - For formatting and validating JSON data."}</li>
                            </ul>
                        </div>
                    </div>
                    <div class="tool-container">
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
                        <div style="display: flex; align-items: center; padding-left: 20px; padding-right: 20px; margin-bottom: 10px; margin-top: 20px;">
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
            </>
        }
    }
}
