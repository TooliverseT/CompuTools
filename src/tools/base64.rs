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
                        <h1 class="tool-title">
                            { "Base64 Converter" }
                        </h1>
                <div class="tool-wrapper">
                        <div class="tool-intro">
                        <div class="content-section">
                            <h2>{"🔤 What is Base64?"}</h2>
                            <p>{"Base64 is a binary-to-text encoding scheme that represents binary data in an ASCII string format by translating it into a radix-64 representation. It is commonly used to encode data for safe transmission over media that are designed to deal with textual data."}</p>
                            <p>{"Base64 encoding is widely used in email via MIME, storing complex data in XML or JSON, embedding images in HTML/CSS, and safely transmitting binary data over text-based protocols."}</p>
                        </div>

                        <div class="content-section">
                            <h2>{"⚙️ How This Base64 Converter Works"}</h2>
                            <p>{"This tool provides bidirectional conversion between plain text and Base64-encoded strings. It supports instant encoding and decoding, making it easy to work with Base64 in your daily workflow."}</p>
                            <h3>{"Supported Features:"}</h3>
                            <ul>
                                <li><strong>{"Text to Base64:"}</strong> {"Encode any text into a Base64 string instantly."}</li>
                                <li><strong>{"Base64 to Text:"}</strong> {"Decode Base64-encoded data back to readable text."}</li>
                                <li><strong>{"Real-time Conversion:"}</strong> {"Results update as you type."}</li>
                                <li><strong>{"Copy with Notification:"}</strong> {"Click any output field to copy results with visual feedback."}</li>
                                <li><strong>{"Robust Error Handling:"}</strong> {"Gracefully handles invalid Base64 input and displays clear error messages."}</li>
                            </ul>
                            <h3>{"Input Format Examples:"}</h3>
                            <div class="example-box">
                                <p><strong>{"Text input example:"}</strong></p>
                                <ul>
                                    <li>{"Hello World"}</li>
                                    <li>{"CompuTools 123!"}</li>
                                </ul>
                                <p><strong>{"Base64 input example:"}</strong></p>
                                <ul>
                                    <li>{"SGVsbG8gV29ybGQ="}</li>
                                    <li>{"Q29tcHVTb29scyAxMjMh"}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"💡 Common Use Cases"}</h2>
                            <div class="use-case">
                                <h3>{"1. Web Development & APIs"}</h3>
                                <ul>
                                    <li><strong>{"Data Embedding:"}</strong> {"Embed images or files directly in HTML, CSS, or JSON using Base64 strings."}</li>
                                    <li><strong>{"API Payloads:"}</strong> {"Transmit binary data (like files) in JSON or XML payloads."}</li>
                                    <li><strong>{"Authentication:"}</strong> {"Encode credentials for HTTP Basic Authentication headers."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"2. Email & Messaging"}</h3>
                                <ul>
                                    <li><strong>{"MIME Encoding:"}</strong> {"Send attachments and non-ASCII data in emails using Base64 encoding."}</li>
                                    <li><strong>{"Safe Text Transmission:"}</strong> {"Ensure binary data is not corrupted when sent over text-based protocols."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"3. Data Storage & Serialization"}</h3>
                                <ul>
                                    <li><strong>{"Database Storage:"}</strong> {"Store binary files or images as Base64 strings in databases."}</li>
                                    <li><strong>{"Configuration Files:"}</strong> {"Embed binary data in YAML, JSON, or XML configs."}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"📚 Step-by-Step Tutorial"}</h2>
                            <div class="tutorial-step">
                                <h3>{"Example 1: Encoding Text to Base64"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Convert the text 'Hello World' to Base64."}</p>
                                <ol>
                                    <li>{"Ensure the tool is in 'Text to Base64' mode (default)."}</li>
                                    <li>{"Enter 'Hello World' in the text input field."}</li>
                                    <li>{"View the Base64-encoded result instantly."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Input:"}</strong> {"Hello World"}</p>
                                    <p><strong>{"Base64 Output:"}</strong> {"SGVsbG8gV29ybGQ="}</p>
                                </div>
                                <p><strong>{"Explanation:"}</strong> {"Each character is converted to its binary representation, grouped into 6-bit chunks, and mapped to a Base64 alphabet."}</p>
                            </div>
                            <div class="tutorial-step">
                                <h3>{"Example 2: Decoding Base64 to Text"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Decode a Base64 string back to readable text."}</p>
                                <ol>
                                    <li>{"Switch to 'Base64 to Text' mode by clicking the rotate icon (⟲)."}</li>
                                    <li>{"Enter a Base64 string in the input field (e.g., 'SGVsbG8gV29ybGQ=')."}</li>
                                    <li>{"The decoded text appears automatically in the output field."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Input:"}</strong> {"SGVsbG8gV29ybGQ="}</p>
                                    <p><strong>{"Output:"}</strong> {"Hello World"}</p>
                                </div>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🔧 Technical Background"}</h2>
                            <h3>{"How Base64 Works"}</h3>
                            <p>{"Base64 encoding takes three bytes of binary data (24 bits) and splits them into four groups of six bits. Each 6-bit group is mapped to a character in the Base64 alphabet (A-Z, a-z, 0-9, +, /). Padding with '=' is used if the input is not a multiple of 3 bytes."}</p>
                            <div class="example-box">
                                <p><strong>{"Example for 'Man':"}</strong></p>
                                <ul>
                                    <li>{"ASCII: M = 77, a = 97, n = 110"}</li>
                                    <li>{"Binary: 01001101 01100001 01101110"}</li>
                                    <li>{"Base64: TWFu"}</li>
                                </ul>
                            </div>
                            <h3>{"Why Use Base64?"}</h3>
                            <ul>
                                <li>{"Ensures data integrity over text-only channels."}</li>
                                <li>{"Prevents data corruption in email, URLs, and config files."}</li>
                                <li>{"Universally supported and easy to implement."}</li>
                            </ul>
                            <h3>{"Performance & Implementation"}</h3>
                            <ul>
                                <li><strong>{"Efficient Encoding:"}</strong> {"Minimal overhead for encoding/decoding operations."}</li>
                                <li><strong>{"Browser Compatibility:"}</strong> {"Works in all modern browsers and environments."}</li>
                                <li><strong>{"Local Processing:"}</strong> {"All conversions happen in your browser for privacy and speed."}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"❓ Frequently Asked Questions"}</h2>
                            <div class="faq-item">
                                <h3>{"Q: What characters are used in Base64?"}</h3>
                                <p>{"A: The Base64 alphabet consists of uppercase A-Z, lowercase a-z, digits 0-9, and the symbols '+' and '/'. The '=' character is used for padding."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Why does my Base64 string end with '=' or '=='?"}</h3>
                                <p>{"A: Padding is added to ensure the output length is a multiple of 4. '=' or '==' at the end of a Base64 string is normal and should not be removed."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Can Base64 encode any type of data?"}</h3>
                                <p>{"A: Yes, Base64 can encode any binary data, including images, files, and text. However, it increases the data size by about 33%."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Is Base64 secure for encryption?"}</h3>
                                <p>{"A: No, Base64 is not an encryption method. It is an encoding scheme and should not be used for security purposes."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: What if I enter invalid Base64?"}</h3>
                                <p>{"A: The tool will display an error message if the input is not valid Base64. Always check your input for typos or missing padding."}</p>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🎯 Best Practices"}</h2>
                            <ul>
                                <li><strong>{"Validate Input:"}</strong> {"Always check that your Base64 input is properly padded and contains only valid characters."}</li>
                                <li><strong>{"Error Handling:"}</strong> {"Handle decoding errors gracefully in your applications."}</li>
                                <li><strong>{"Performance:"}</strong> {"Avoid unnecessary Base64 encoding/decoding in performance-critical paths."}</li>
                                <li><strong>{"Documentation:"}</strong> {"Document when and why Base64 is used in your codebase."}</li>
                                <li><strong>{"Testing:"}</strong> {"Test with edge cases, such as empty strings and non-ASCII data."}</li>
                                <li><strong>{"Data Size Awareness:"}</strong> {"Remember that Base64 increases data size by ~33%."}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"🔗 Related Tools"}</h2>
                            <p>{"Enhance your workflow with these related tools:"}</p>
                            <ul>
                                <li><a href="/ascii/">{"ASCII Converter"}</a> {" - For converting text to ASCII codes and vice versa."}</li>
                                <li><a href="/html/">{"HTML Entity Encoder"}</a> {" - For web-safe character encoding and HTML content."}</li>
                                <li><a href="/url/">{"URL Encoder/Decoder"}</a> {" - For URL-safe string encoding in web applications."}</li>
                                <li><a href="/json/">{"JSON Formatter"}</a> {" - For structured data formatting and validation."}</li>
                                <li><a href="/base/">{"Number Base Converter"}</a> {" - For converting between different number bases."}</li>
                            </ul>
                        </div>
                    </div>
                    <div class="tool-container">
                        <div style="display: flex; align-items: center; margin-bottom: 10px; margin-top: 5px;">
                            <div style="width: 90%;">
                                if !convert {
                                    {"Text to Base64"}
                                } else {
                                    {"Base64 to Text"}
                                }
                            </div>
                            <div onclick={on_convert} class="tool-change" style="width: 10%; display: flex; justify-content: center;">
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
