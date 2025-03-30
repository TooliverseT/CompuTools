use yew::prelude::*;
use html_escape::{encode_text, decode_html_entities, encode_unquoted_attribute};
use web_sys::{window, HtmlInputElement};
use wasm_bindgen_futures::JsFuture;
use log::info;
use regex::Regex;

pub struct ToolHtml {
    input_text: String,
    output_text: String,
    mode: String,
}

pub enum Msg {
    UpdateInput(String),
    CopyToClipboard(String),
    UpdateMode(String),
}

impl Component for ToolHtml {
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
                    self.output_text = encode_text(&self.input_text).to_string();
                } else if self.mode == "encode_w_unicode" {
                    let html_encoded = encode_text(&self.input_text);
                    self.output_text = encode_unicode_text(&html_encoded);
                } else {
                    self.output_text = decode_html_custom(&self.input_text);
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
                            { "HTML Converter" }
                        </h1>
                        <div class="tool-intro">
                            <p>
                                { "This tool helps you encode and decode HTML entities easily. HTML entities are special character sequences used to represent characters that might otherwise be interpreted as HTML markup." }
                            </p>
                            <p>{ "With this tool, you can:" }</p>
                            <ul>
                                <li>{ "Convert special characters to their corresponding HTML entities to ensure your content displays correctly on web pages." }</li>
                                <li>{ "Decode HTML entities back to their original characters for editing or display purposes." }</li>
                                <li>{ "Enable Unicode encoding to properly handle international characters and symbols across different character sets." }</li>
                            </ul>
                            <p>
                                { "This tool is especially useful for web developers, content creators, or anyone working with HTML content that contains special characters or international text." }
                            </p>
                            <p>{ "Note:" }</p>
                            <ul>
                                <li>{ "The Unicode encoding option converts non-ASCII characters to their hexadecimal entity representation (&#x...;)." }</li>
                                <li>{ "The decoder handles both named entities (like &amp;) and numeric entities (like &#x...;), ensuring complete conversion." }</li>
                            </ul>
                            <p>
                                { "Simplify your HTML content preparation with this straightforward encoder and decoder tool." }
                            </p>
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
                                    <option value="encode_w_unicode" selected={self.mode == "encode_w_unicode"}>{ "Encode with unicode" }</option>
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

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(window) = window() {
                let document = window.document();
                if let Some(doc) = document {
                    doc.set_title("HTML Converter | CompuTools");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "This tool helps you encode and decode HTML entities easily. HTML entities are special character sequences used to represent characters that might otherwise be interpreted as HTML markup. Convert special characters to their corresponding HTML entities to ensure your content displays correctly on web pages. This tool is especially useful for web developers, content creators, or anyone working with HTML content that contains special characters or international text.").unwrap();
                    }
                }
            }
        }
    }
}

fn encode_unicode_text(input: &str) -> String {
    input.chars().map(|c| {
        // 유니코드 문자일 경우 &#x<유니코드>로 변환
        if c.is_ascii() {
            c.to_string()
        } else {
            format!("&#x{:X};", c as u32)
        }
    }).collect()
}

fn decode_html_custom(input: &str) -> String {
    // 기본 HTML 엔티티를 먼저 처리 (공통 엔티티)
    let basic_entities = [
        ("&amp;", "&"),
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&apos;", "'"),
    ];
    
    let mut result = input.to_string();
    for (entity, replacement) in basic_entities.iter() {
        result = result.replace(entity, replacement);
    }
    
    // 유니코드 엔티티 처리 (&#x[0-9A-F]+; 형식)
    let re = Regex::new(r"&#x([0-9A-Fa-f]+);").unwrap();
    
    // 모든 유니코드 엔티티를 찾아서 처리
    while re.is_match(&result) {
        result = re.replace_all(&result, |caps: &regex::Captures| {
            let hex_str = &caps[1];
            if let Ok(code_point) = u32::from_str_radix(hex_str, 16) {
                if let Some(character) = char::from_u32(code_point) {
                    character.to_string()
                } else {
                    caps[0].to_string() // 유효하지 않은 코드 포인트는 원래 문자열 유지
                }
            } else {
                caps[0].to_string() // 16진수 파싱 실패시 원래 문자열 유지
            }
        }).to_string();
    }
    
    // 10진수 엔티티도 처리 (&#[0-9]+; 형식)
    let re_decimal = Regex::new(r"&#([0-9]+);").unwrap();
    
    result = re_decimal.replace_all(&result, |caps: &regex::Captures| {
        let decimal_str = &caps[1];
        if let Ok(code_point) = decimal_str.parse::<u32>() {
            if let Some(character) = char::from_u32(code_point) {
                character.to_string()
            } else {
                caps[0].to_string()
            }
        } else {
            caps[0].to_string()
        }
    }).to_string();
    
    result
}