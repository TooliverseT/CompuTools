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
                <h1 class="tool-title">
                    { "HTML Converter" }
                </h1>
                <div class="tool-wrapper">
                    <div class="tool-intro">
                        <div class="content-section">
                            <h2>{"🔤 What is an HTML Entity?"}</h2>
                            <p>{"An HTML entity is a special sequence of characters used to represent reserved or invisible characters in HTML. Entities are used to display characters that would otherwise be interpreted as HTML markup, such as <, >, &, or non-breaking spaces."}</p>
                            <p>{"HTML entities are essential for ensuring that web content displays correctly and securely, especially when working with user-generated or international text."}</p>
                        </div>

                        <div class="content-section">
                            <h2>{"⚙️ How This HTML Converter Works"}</h2>
                            <p>{"This tool encodes and decodes HTML entities, supporting both standard and Unicode characters. It provides instant conversion and is ideal for web developers, content creators, and anyone working with HTML content."}</p>
                            <h3>{"Supported Features:"}</h3>
                            <ul>
                                <li><strong>{"Encode Special Characters:"}</strong> {"Convert <, >, &, \", ', and more to their HTML entity equivalents."}</li>
                                <li><strong>{"Decode Entities:"}</strong> {"Convert HTML entities back to their original characters."}</li>
                                <li><strong>{"Unicode Support:"}</strong> {"Encode non-ASCII characters as hexadecimal entities (&#x...;)."}</li>
                                <li><strong>{"Real-time Conversion:"}</strong> {"Results update as you type."}</li>
                                <li><strong>{"Copy with Notification:"}</strong> {"Click any output field to copy results with visual feedback."}</li>
                            </ul>
                            <h3>{"Input Format Examples:"}</h3>
                            <div class="example-box">
                                <p><strong>{"Text input example:"}</strong></p>
                                <ul>
                                    <li>{"<div class='greeting'>Hello & Welcome!</div>"}</li>
                                    <li>{"Café & Résumé"}</li>
                                </ul>
                                <p><strong>{"HTML entity input example:"}</strong></p>
                                <ul>
                                    <li>{"&lt;div class=&#x27;greeting&#x27;&gt;Hello &amp; Welcome!&lt;/div&gt;"}</li>
                                    <li>{"Caf&eacute; &amp; R&eacute;sum&eacute;"}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"💡 Common Use Cases"}</h2>
                            <div class="use-case">
                                <h3>{"1. Web Development"}</h3>
                                <ul>
                                    <li><strong>{"Safe Content Display:"}</strong> {"Prevent HTML injection by encoding user input."}</li>
                                    <li><strong>{"Template Rendering:"}</strong> {"Ensure dynamic content displays correctly in templates."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"2. Internationalization"}</h3>
                                <ul>
                                    <li><strong>{"Unicode Characters:"}</strong> {"Encode and display non-ASCII characters in HTML documents."}</li>
                                    <li><strong>{"Multilingual Content:"}</strong> {"Support for accented letters, symbols, and scripts from around the world."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"3. Data Exchange & APIs"}</h3>
                                <ul>
                                    <li><strong>{"XML/JSON Embedding:"}</strong> {"Safely embed HTML content in XML or JSON payloads."}</li>
                                    <li><strong>{"Email Templates:"}</strong> {"Prepare HTML for use in email clients that require entity encoding."}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"📚 Step-by-Step Tutorial"}</h2>
                            <div class="tutorial-step">
                                <h3>{"Example 1: Encoding HTML"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Convert special characters in a string to HTML entities."}</p>
                                <ol>
                                    <li>{"Set the mode to 'Encode'."}</li>
                                    <li>{"Enter text containing special characters (e.g., <, >, &, \", ')."}</li>
                                    <li>{"View the encoded HTML entities in the output field."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Input:"}</strong> {"<div>Hello & Welcome!</div>"}</p>
                                    <p><strong>{"Output:"}</strong> {"&lt;div&gt;Hello &amp; Welcome!&lt;/div&gt;"}</p>
                                </div>
                            </div>
                            <div class="tutorial-step">
                                <h3>{"Example 2: Decoding HTML Entities"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Convert HTML entities back to their original characters."}</p>
                                <ol>
                                    <li>{"Set the mode to 'Decode'."}</li>
                                    <li>{"Enter a string containing HTML entities (e.g., &amp;lt;, &amp;gt;, &amp;amp;)."}</li>
                                    <li>{"View the decoded text in the output field."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Input:"}</strong> {"&lt;div&gt;Hello &amp; Welcome!&lt;/div&gt;"}</p>
                                    <p><strong>{"Output:"}</strong> {"<div>Hello & Welcome!</div>"}</p>
                                </div>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🔧 Technical Background"}</h2>
                            <h3>{"How HTML Entities Work"}</h3>
                            <p>{"HTML entities use a special syntax: &amp;name; for named entities (e.g., &amp;lt; for <), and &amp;#xHEX; or &amp;#DEC; for numeric entities. Browsers automatically convert these entities to their corresponding characters when rendering HTML."}</p>
                            <div class="example-box">
                                <p><strong>{"Example for Unicode Character:"}</strong></p>
                                <ul>
                                    <li>{"Input: Café"}</li>
                                    <li>{"Encoded: Caf&eacute; or Caf&#xE9;"}</li>
                                </ul>
                            </div>
                            <h3>{"Why Use HTML Entities?"}</h3>
                            <ul>
                                <li>{"Prevent HTML injection and XSS attacks."}</li>
                                <li>{"Ensure correct display of special and international characters."}</li>
                                <li>{"Maintain compatibility across browsers and email clients."}</li>
                            </ul>
                            <h3>{"Performance & Implementation"}</h3>
                            <ul>
                                <li><strong>{"Fast Conversion:"}</strong> {"Instant encoding/decoding in your browser."}</li>
                                <li><strong>{"No Server Required:"}</strong> {"All processing happens locally for privacy and speed."}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"❓ Frequently Asked Questions"}</h2>
                            <div class="faq-item">
                                <h3>{"Q: What is the difference between encoding and decoding?"}</h3>
                                <p>{"A: Encoding converts special characters to HTML entities, while decoding converts entities back to their original characters."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Can this tool handle Unicode characters?"}</h3>
                                <p>{"A: Yes, use the 'Encode with unicode' mode to convert non-ASCII characters to hexadecimal entities."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Is this tool safe for sensitive data?"}</h3>
                                <p>{"A: Yes, all processing happens locally in your browser. No data is sent to any server."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Why do I need to encode HTML?"}</h3>
                                <p>{"A: Encoding prevents browsers from interpreting special characters as HTML markup, ensuring your content displays as intended."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: What if I enter invalid HTML entities?"}</h3>
                                <p>{"A: The tool will attempt to decode as much as possible. Invalid entities will remain unchanged in the output."}</p>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🎯 Best Practices"}</h2>
                            <ul>
                                <li><strong>{"Validate Input:"}</strong> {"Always check your input for unescaped special characters."}</li>
                                <li><strong>{"Error Handling:"}</strong> {"Handle invalid or incomplete entities gracefully in your applications."}</li>
                                <li><strong>{"Performance:"}</strong> {"Use local tools for instant conversion and privacy."}</li>
                                <li><strong>{"Documentation:"}</strong> {"Document when and why entity encoding is used in your codebase."}</li>
                                <li><strong>{"Testing:"}</strong> {"Test with a variety of characters, including Unicode and edge cases."}</li>
                                <li><strong>{"Security Awareness:"}</strong> {"Remember that encoding is essential for preventing XSS and injection attacks."}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"🔗 Related Tools"}</h2>
                            <p>{"Enhance your workflow with these related tools:"}</p>
                            <ul>
                                <li><a href="/base64/">{"Base64 Encoder/Decoder"}</a> {" - For binary-safe text encoding and data transmission."}</li>
                                <li><a href="/ascii/">{"ASCII Converter"}</a> {" - For converting text to ASCII codes and vice versa."}</li>
                                <li><a href="/url/">{"URL Encoder/Decoder"}</a> {" - For URL-safe string encoding in web applications."}</li>
                                <li><a href="/json/">{"JSON Formatter"}</a> {" - For structured data formatting and validation."}</li>
                                <li><a href="/base/">{"Number Base Converter"}</a> {" - For converting between different number bases."}</li>
                            </ul>
                        </div>
                    </div>
                    <div class="tool-container">
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