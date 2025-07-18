use log::info;
use std::f64::consts::PI;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{window, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum AsciiMode {
    Decimal,
    Hex,
    Binary, // 새로운 바이너리 모드 추가
}

pub struct ToolAscii {
    input_ascii: String,
    output_text: String,
    input_text: String,
    output_ascii: String,
    convert: bool,
    mode: AsciiMode,
    show_ascii_table: bool, // ASCII 테이블 표시 여부
}

pub enum Msg {
    UpdateAscii(String),
    UpdateText(String),
    ModeChanged(AsciiMode),
    Convert,
    CopyToClipboard(String),
    ToggleAsciiTable, // ASCII 테이블 토글
}

impl Component for ToolAscii {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_ascii: String::new(),
            output_text: String::new(),
            input_text: String::new(),
            output_ascii: String::new(),
            convert: false,
            mode: AsciiMode::Hex,
            show_ascii_table: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateAscii(value) => {
                self.input_ascii = value.clone();

                let parsed_bytes = match self.mode {
                    AsciiMode::Decimal => self.parse_decimal_input(&value),
                    AsciiMode::Hex => self.parse_hex_input(&value),
                    AsciiMode::Binary => self.parse_binary_input(&value),
                };

                let bytes = match parsed_bytes {
                        Ok(bytes) => bytes,
                        Err(_) => Vec::new(),
                    };

                if let Ok(text) = String::from_utf8(bytes) {
                            self.output_text = text;
                } else if self.input_ascii.is_empty() {
                    self.output_text = String::new();
                }

                true
            }
            Msg::UpdateText(value) => {
                self.input_text = value;

                let input_bytes = self.input_text.as_bytes().to_vec();

                match self.mode {
                    AsciiMode::Decimal => {
                    self.output_ascii = input_bytes
                        .iter()
                        .map(|byte| byte.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");
                    }
                    AsciiMode::Hex => {
                    self.output_ascii = input_bytes
                        .iter()
                        .map(|byte| format!("0x{:02X}", byte))
                        .collect::<Vec<String>>()
                        .join(" ");
                    }
                    AsciiMode::Binary => {
                        self.output_ascii = input_bytes
                            .iter()
                            .map(|byte| format!("{:08b}", byte))
                            .collect::<Vec<String>>()
                            .join(" ");
                    }
                }
                true
            }
            Msg::ModeChanged(mode) => {
                self.mode = mode;
                self.input_ascii = "".to_string();
                self.output_text = "".to_string();
                // let cb1 = _ctx.link().callback(|value| Msg::UpdateAscii(value));
                // cb1.emit(self.input_ascii.clone());

                let cb2 = _ctx.link().callback(|value| Msg::UpdateText(value));
                cb2.emit(self.input_text.clone());
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
            Msg::ToggleAsciiTable => {
                self.show_ascii_table = !self.show_ascii_table;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let convert = self.convert.clone();
        let on_convert = _ctx.link().callback(|_| Msg::Convert);

        html! {
                <>
                            <h1 class="tool-title">
                                { "ASCII Converter" }
                            </h1>
                    <div class="tool-wrapper">
                            <div class="tool-intro">
                            // 기존의 기본 설명 제거 - 새로운 상세 섹션으로 대체됨
                            
                            // 새로운 상세 설명 섹션들
                            <div class="content-section">
                                <h2>{"🔤 What is ASCII?"}</h2>
                                <p>
                                    {"ASCII (American Standard Code for Information Interchange) is a 7-bit character encoding standard that represents text in computers and electronic devices. Developed in the 1960s, ASCII assigns unique numerical values (0-127) to letters, digits, punctuation marks, and control characters."}
                                </p>
                                <p>
                                    {"Each ASCII character occupies exactly one byte of memory, making it extremely efficient for text storage and transmission. The ASCII table includes:"}
                                </p>
                                <ul>
                                    <li><strong>{"Control characters (0-31):"}</strong> {" Non-printable characters like newline, tab, carriage return"}</li>
                                    <li><strong>{"Printable characters (32-126):"}</strong> {" Letters, numbers, punctuation, and symbols"}</li>
                                    <li><strong>{"Extended ASCII (128-255):"}</strong> {" Additional characters in some implementations"}</li>
                                </ul>
                            </div>

                            <div class="content-section">
                                <h2>{"⚙️ How This ASCII Converter Works"}</h2>
                                <p>
                                    {"Our ASCII converter supports bidirectional conversion between text and ASCII codes with flexible input formats. The tool provides two modes: hexadecimal and decimal representation."}
                                </p>
                                
                                <h3>{"Supported Features:"}</h3>
                                <ul>
                                    <li><strong>{"Text to ASCII:"}</strong> {" Convert any text to ASCII codes instantly"}</li>
                                    <li><strong>{"ASCII to Text:"}</strong> {" Decode ASCII codes back to readable text"}</li>
                                    <li><strong>{"Triple Format Support:"}</strong> {" Choose between hexadecimal (0x41), decimal (65), or binary (01000001) representation"}</li>
                                    <li><strong>{"Flexible Input Parsing:"}</strong> {" Support for various hex formats, space-separated decimals, and 8-bit binary strings"}</li>
                                    <li><strong>{"ASCII Table Viewer:"}</strong> {" Interactive reference table showing all ASCII characters with their codes"}</li>
                                    <li><strong>{"Real-time Conversion:"}</strong> {" Instant results as you type"}</li>
                                    <li><strong>{"Copy with Notification:"}</strong> {" Click any output field to copy results with visual feedback"}</li>
                                </ul>

                                <h3>{"Input Format Examples:"}</h3>
                                <div class="example-box">
                                    <p><strong>{"Hexadecimal formats supported:"}</strong></p>
                                    <ul>
                                        <li>{"0x48 0x65 0x6C 0x6C 0x6F (standard prefix)"}</li>
                                        <li>{"\\x48\\x65\\x6C\\x6C\\x6F (escape sequence style)"}</li>
                                        <li>{"x48x65x6Cx6Cx6F (x prefix only)"}</li>
                                        <li>{"48656C6C6F (raw hex, parsed in pairs)"}</li>
                                    </ul>
                                    <p><strong>{"Decimal format:"}</strong></p>
                                    <ul>
                                        <li>{"72 101 108 108 111 (space-separated numbers)"}</li>
                                    </ul>
                                    <p><strong>{"Binary format:"}</strong></p>
                                    <ul>
                                        <li>{"01001000 01100101 01101100 01101100 01101111 (8-bit binary strings)"}</li>
                                        <li>{"0100100001100101011011000110110001101111 (continuous binary stream)"}</li>
                                    </ul>
                                </div>
                            </div>

                            <div class="content-section">
                                <h2>{"📋 ASCII Table Reference"}</h2>
                                <p>
                                    {"The complete ASCII character set contains 128 characters (0-127). This interactive table shows each character with its decimal, hexadecimal, binary values, and descriptions. Control characters (0-31) and DEL (127) are highlighted in gray."}
                                </p>
                                
                                <div style="margin: 20px 0;">
                                    <button 
                                        class="tool-btn"
                                        onclick={_ctx.link().callback(|_| Msg::ToggleAsciiTable)}
                                        style="padding: 10px 20px; background-color: var(--color-fourth); color: white; border: none; border-radius: 5px; cursor: pointer;"
                                    >
                                        if self.show_ascii_table {
                                            {"Hide ASCII Table"}
                                        } else {
                                            {"Show ASCII Table"}
                                        }
                                    </button>
                                </div>
                                
                                if self.show_ascii_table {
                                    <div style="max-height: 400px; overflow-y: auto; font-family: monospace; font-size: 12px; border: 1px solid #ddd; border-radius: 5px;">
                                        <table style="width: 100%; border-collapse: collapse;">
                                            <thead>
                                                <tr style="background-color: var(--color-fourth); color: white;">
                                                    <th style="padding: 8px; border: 1px solid #ddd;">{"Char"}</th>
                                                    <th style="padding: 8px; border: 1px solid #ddd;">{"Dec"}</th>
                                                    <th style="padding: 8px; border: 1px solid #ddd;">{"Hex"}</th>
                                                    <th style="padding: 8px; border: 1px solid #ddd;">{"Binary"}</th>
                                                    <th style="padding: 8px; border: 1px solid #ddd;">{"Description"}</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                { self.render_ascii_table() }
                                            </tbody>
                                        </table>
                                    </div>
                                }
                                
                                <p style="margin-top: 15px;">
                                    {"Understanding these character codes is essential for programming, data processing, and text manipulation. Each character has a unique code that computers use internally to represent text."}
                                </p>
                            </div>

                            <div class="content-section">
                                <h2>{"💡 Common Use Cases"}</h2>
                                <div class="use-case">
                                    <h3>{"1. Programming & Software Development"}</h3>
                                    <ul>
                                        <li><strong>{"Debugging:"}</strong> {" Inspect character data in applications to identify encoding issues"}</li>
                                        <li><strong>{"Protocol Implementation:"}</strong> {" Build communication protocols that transmit text as numerical data"}</li>
                                        <li><strong>{"Data Validation:"}</strong> {" Verify that input contains only valid ASCII characters"}</li>
                                        <li><strong>{"Escape Sequences:"}</strong> {" Create or parse escape sequences in strings"}</li>
                                        <li><strong>{"Binary File Analysis:"}</strong> {" Extract readable text from binary files"}</li>
                                    </ul>
                                </div>
                                
                                <div class="use-case">
                                    <h3>{"2. Cybersecurity & Forensics"}</h3>
                                    <ul>
                                        <li><strong>{"Payload Analysis:"}</strong> {" Decode suspicious data in security investigations"}</li>
                                        <li><strong>{"Log Analysis:"}</strong> {" Convert encoded log entries to readable text"}</li>
                                        <li><strong>{"Malware Research:"}</strong> {" Analyze obfuscated strings in malicious code"}</li>
                                        <li><strong>{"Network Traffic:"}</strong> {" Decode ASCII-encoded data in network packets"}</li>
                                        <li><strong>{"Forensic Investigation:"}</strong> {" Extract text from corrupted or encoded files"}</li>
                                    </ul>
                                </div>
                                
                                <div class="use-case">
                                    <h3>{"3. Data Processing & Analysis"}</h3>
                                    <ul>
                                        <li><strong>{"File Format Analysis:"}</strong> {" Examine binary files containing ASCII text"}</li>
                                        <li><strong>{"Legacy System Integration:"}</strong> {" Work with older systems that use ASCII encoding"}</li>
                                        <li><strong>{"Data Migration:"}</strong> {" Convert text data between different encoding formats"}</li>
                                        <li><strong>{"Quality Assurance:"}</strong> {" Verify text data integrity during transmission"}</li>
                                        <li><strong>{"Configuration Files:"}</strong> {" Parse or generate configuration files with ASCII data"}</li>
                                    </ul>
                                </div>
                            </div>

                            <div class="content-section">
                                <h2>{"📚 Step-by-Step Tutorial"}</h2>
                                
                                <div class="tutorial-step">
                                    <h3>{"Example 1: Converting Text to ASCII"}</h3>
                                    <p><strong>{"Goal:"}</strong> {" Convert the word 'Hello' to ASCII values"}</p>
                                    <ol>
                                        <li>{"Make sure the tool is in 'Text to ASCII' mode (default)"}</li>
                                        <li>{"Enter 'Hello' in the text input field"}</li>
                                        <li>{"Select your preferred output format using the dropdown:"}</li>
                                        <ul>
                                            <li>{"Choose 'HEX' for hexadecimal output"}</li>
                                            <li>{"Choose 'DECIMAL' for decimal output"}</li>
                                        </ul>
                                        <li>{"View the automatic conversion results"}</li>
                                    </ol>
                                    <div class="example-box">
                                        <p><strong>{"Input:"}</strong> {" Hello"}</p>
                                        <p><strong>{"Decimal Output:"}</strong> {" 72 101 108 108 111"}</p>
                                        <p><strong>{"Hex Output:"}</strong> {" 0x48 0x65 0x6C 0x6C 0x6F"}</p>
                                    </div>
                                    <p><strong>{"Explanation:"}</strong> {" Each character maps to its ASCII value: H=72, e=101, l=108, l=108, o=111"}</p>
                                </div>

                                <div class="tutorial-step">
                                    <h3>{"Example 2: Converting ASCII to Text"}</h3>
                                    <p><strong>{"Goal:"}</strong> {" Decode ASCII values back to readable text"}</p>
                                    <ol>
                                        <li>{"Click the rotation arrow (⟲) to switch to 'ASCII to Text' mode"}</li>
                                        <li>{"Enter ASCII values in the input field:"}</li>
                                        <ul>
                                            <li>{"For decimal: '87 111 114 108 100' (space-separated)"}</li>
                                            <li>{"For hex: '0x57 0x6F 0x72 0x6C 0x64' (any supported format)"}</li>
                                        </ul>
                                        <li>{"The converted text appears automatically in the output field"}</li>
                                        <li>{"Click the output field to copy the result to clipboard"}</li>
                                    </ol>
                                    <div class="example-box">
                                        <p><strong>{"Input:"}</strong> {" 87 111 114 108 100"}</p>
                                        <p><strong>{"Output:"}</strong> {" World"}</p>
                                    </div>
                                </div>

                                <div class="tutorial-step">
                                    <h3>{"Example 3: Working with Mixed Hex Formats"}</h3>
                                    <p><strong>{"Goal:"}</strong> {" Handle different hex input formats in the same conversion"}</p>
                                    <p>{"Our tool intelligently parses various hexadecimal input formats:"}</p>
                                    <div class="example-box">
                                        <p><strong>{"Input variations (all produce 'Hello'):"}</strong></p>
                                        <ul>
                                            <li>{"0x48 0x65 0x6C 0x6C 0x6F"}</li>
                                            <li>{"\\x48\\x65\\x6C\\x6C\\x6F"}</li>
                                            <li>{"x48x65x6Cx6Cx6F"}</li>
                                            <li>{"48656C6C6F"}</li>
                                            <li>{"0x48 \\x65 x6C 6C 6F (mixed formats)"}</li>
                                        </ul>
                                    </div>
                                    <p><strong>{"Pro Tip:"}</strong> {" You can mix different hex formats in the same input. The parser automatically detects and handles each format correctly."}</p>
                                </div>
                            </div>

                            <div class="content-section">
                                <h2>{"🔧 Technical Background"}</h2>
                                
                                <h3>{"ASCII vs. Unicode"}</h3>
                                <p>
                                    {"While ASCII is limited to 128 characters, Unicode extends this to support virtually all writing systems worldwide. ASCII remains important because:"}
                                </p>
                                <ul>
                                    <li>{"It's a subset of Unicode (first 128 characters are identical)"}</li>
                                    <li>{"It's extremely efficient for English text and programming code"}</li>
                                    <li>{"It's universally supported across all systems and platforms"}</li>
                                    <li>{"It's the foundation for many other encoding schemes"}</li>
                                    <li>{"It requires minimal processing power and memory"}</li>
                                </ul>

                                <h3>{"Binary Representation"}</h3>
                                <p>
                                    {"ASCII values are stored as 7-bit binary numbers, though typically padded to 8 bits (1 byte). Understanding the binary representation helps with low-level programming:"}
                                </p>
                                <div class="example-box">
                                    <p><strong>{"Character 'A' breakdown:"}</strong></p>
                                    <ul>
                                        <li>{"ASCII value: 65"}</li>
                                        <li>{"Binary: 01000001"}</li>
                                        <li>{"Hexadecimal: 0x41"}</li>
                                        <li>{"Bit positions: 64 + 1 = 65"}</li>
                                    </ul>
                                </div>

                                <h3>{"Performance & Implementation"}</h3>
                                <p>
                                    {"Our ASCII converter is built with Rust and WebAssembly for optimal performance:"}
                                </p>
                                <ul>
                                    <li><strong>{"Zero-cost abstractions:"}</strong> {" No performance overhead from safety features"}</li>
                                    <li><strong>{"Memory efficiency:"}</strong> {" Direct byte manipulation without unnecessary allocations"}</li>
                                    <li><strong>{"Browser compatibility:"}</strong> {" Runs at near-native speed in all modern browsers"}</li>
                                    <li><strong>{"Instant conversion:"}</strong> {" Real-time processing as you type with no lag"}</li>
                                    <li><strong>{"Local processing:"}</strong> {" All conversion happens in your browser - no server round trips"}</li>
                                </ul>
                            </div>

                            <div class="content-section">
                                <h2>{"❓ Frequently Asked Questions"}</h2>
                                
                                <div class="faq-item">
                                    <h3>{"Q: What's the difference between ASCII and UTF-8?"}</h3>
                                    <p>
                                        {"A: ASCII is a 7-bit encoding that supports 128 characters, primarily English letters, numbers, and basic symbols. UTF-8 is a variable-length encoding that can represent all Unicode characters while being backward-compatible with ASCII. The first 128 UTF-8 characters are identical to ASCII, so this tool works for UTF-8 text containing only ASCII characters."}
                                    </p>
                                </div>

                                <div class="faq-item">
                                    <h3>{"Q: Why do I get different results in hex vs decimal mode?"}</h3>
                                    <p>
                                        {"A: The underlying ASCII values are identical; only the display format changes. Decimal shows numbers in base 10 (0-9), while hexadecimal uses base 16 (0-9, A-F). For example, the letter 'A' is 65 in decimal and 0x41 in hexadecimal - both represent the same value."}
                                    </p>
                                </div>

                                <div class="faq-item">
                                    <h3>{"Q: Can this tool handle non-English characters?"}</h3>
                                    <p>
                                        {"A: This tool focuses on standard ASCII (0-127). Characters outside this range (like accented letters, emojis, or non-Latin scripts) may not convert correctly. For international text, consider using Unicode tools or UTF-8 converters that can handle extended character sets."}
                                    </p>
                                </div>

                                <div class="faq-item">
                                    <h3>{"Q: How do I handle control characters?"}</h3>
                                    <p>
                                        {"A: Control characters (0-31) like newline (10), tab (9), and carriage return (13) can be converted but may not display visibly in the text output. They're essential for formatting and control in programming and data processing. You'll see their ASCII values but not visual representation."}
                                    </p>
                                </div>

                                <div class="faq-item">
                                    <h3>{"Q: Is my data safe when using this tool?"}</h3>
                                    <p>
                                        {"A: Absolutely! All conversion happens locally in your browser using WebAssembly. Your data never leaves your device, ensuring complete privacy and security. No information is transmitted to our servers, and nothing is stored or logged."}
                                    </p>
                                </div>

                                <div class="faq-item">
                                    <h3>{"Q: What if I enter invalid ASCII codes?"}</h3>
                                    <p>
                                        {"A: The tool validates input and handles errors gracefully. Invalid ASCII values (above 127 or below 0) are ignored, and malformed hex inputs are parsed as best as possible. Always ensure your ASCII values are in the valid range for reliable conversion."}
                                    </p>
                                </div>
                            </div>

                            <div class="content-section">
                                <h2>{"🎯 Best Practices"}</h2>
                                <ul>
                                    <li><strong>{"Input validation:"}</strong> {" Always validate ASCII input ranges (0-127) in your applications"}</li>
                                    <li><strong>{"Error handling:"}</strong> {" Implement proper error handling for invalid character codes"}</li>
                                    <li><strong>{"Performance:"}</strong> {" For large datasets, consider batch processing rather than character-by-character conversion"}</li>
                                    <li><strong>{"Documentation:"}</strong> {" Clearly document the expected encoding format in your code and APIs"}</li>
                                    <li><strong>{"Testing:"}</strong> {" Test with edge cases like control characters and boundary values (0, 127)"}</li>
                                    <li><strong>{"Consistency:"}</strong> {" Choose one hex format style and stick with it throughout your project"}</li>
                                    <li><strong>{"Backup data:"}</strong> {" Always keep original data when performing encoding conversions"}</li>
                                </ul>
                            </div>

                            <div class="content-section">
                                <h2>{"🔗 Related Tools"}</h2>
                                <p>{"Enhance your text processing workflow with these complementary tools:"}</p>
                                <ul>
                                    <li><a href="/base64/">{"Base64 Encoder/Decoder"}</a> {" - For binary-safe text encoding and data transmission"}</li>
                                    <li><a href="/html/">{"HTML Entity Encoder"}</a> {" - For web-safe character encoding and HTML content"}</li>
                                    <li><a href="/url/">{"URL Encoder/Decoder"}</a> {" - For URL-safe string encoding in web applications"}</li>
                                    <li><a href="/json/">{"JSON Formatter"}</a> {" - For structured data formatting and validation"}</li>
                                    <li><a href="/base/">{"Number Base Converter"}</a> {" - For converting between different number bases"}</li>
                                </ul>
                            </div>
                        </div>
                        <div class="tool-container">
                            <div style="display: flex; align-items: center; margin-bottom: 10px; margin-top: 5px;">
                                <div style="width: 90%;">
                                    if !convert {
                                        {"Text to ASCII"}
                                    } else {
                                        {"ASCII to Text"}
                                    }
                                </div>
                                <div onclick={on_convert} class="tool-change" style="width: 10%; display: flex; justify-content: center;">
                                    <i class="fa-solid fa-arrows-rotate"></i>
                                </div>
                            </div>
                            <div style="display: flex; align-items: center; margin-bottom: 10px; margin-top: 5px;">
                                <div style="width: 70%;">
                                    {"Output Format: "}
                                </div>
                                <select
                                    id="input-mode-select"
                                    style="width: 30%;"
                                    onchange={_ctx.link().callback(|e: Event| {
                                        let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                        match value.as_str() {
                                            "decimal" => Msg::ModeChanged(AsciiMode::Decimal),
                                            "hex" => Msg::ModeChanged(AsciiMode::Hex),
                                            "binary" => Msg::ModeChanged(AsciiMode::Binary),
                                            _ => unreachable!(),
                                        }
                                    })}>
                                    <option value="hex" selected={self.mode == AsciiMode::Hex}>{ "HEX" }</option>
                                    <option value="decimal" selected={self.mode == AsciiMode::Decimal}>{ "DECIMAL" }</option>
                                    <option value="binary" selected={self.mode == AsciiMode::Binary}>{ "BINARY" }</option>
                                </select>
                            </div>
                            if !convert {
                                <div class="tool-inner">
                                    <div>
                                        <div class="tool-subtitle" style="margin-bottom: 5px;">{ "Text" }</div>
                                        <textarea
                                            type="text"
                                            style="overflow: auto;"
                                            value={self.input_text.clone()}
                                            placeholder={ "Enter text..."}
                                            oninput={_ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                Msg::UpdateText(input.value())
                                            })}
                                        />
                                    </div>
                                </div>
                                <div class="tool-inner" style="margin-top: 10px;">
                                    <div>
                                        <div class="tool-subtitle">{ "ASCII" }</div>
                                        <textarea
                                            type="text"
                                            readonly=true
                                            style="overflow: auto; cursor: pointer;"
                                            value={self.output_ascii.clone()}
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
                                        <div class="tool-subtitle" style="margin-bottom: 5px;">{ "ASCII" }</div>
                                        <textarea
                                            type="text"
                                            style="overflow: auto;"
                                            value={self.input_ascii.clone()}
                                            placeholder={
                                                match self.mode {
                                                    AsciiMode::Hex => "Enter ASCII code... (e.g., 0x41 \\x42 x43 44 45 or 0x41\\x42x434445)",
                                                    AsciiMode::Decimal => "Enter ASCII code... (e.g., 65 66 67 with spaces)",
                                                    AsciiMode::Binary => "Enter ASCII code... (e.g., 01000001 01000010 01000011)",
                                                }
                                            }
                                            oninput={_ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                Msg::UpdateAscii(input.value())
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
                                            value={self.output_text.clone()}
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
                    doc.set_title("ASCII Converter | CompuTools");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "Free ASCII Converter tool for developers and engineers. Convert text to ASCII codes (decimal/hex) and vice versa. Includes comprehensive tutorials, use cases, and technical background. Support for multiple hex formats. Built with Rust and WebAssembly for optimal performance. Essential tool for programming, cybersecurity, data analysis, and debugging.").unwrap();
                    }
                }
            }
        }
    }
}

impl ToolAscii {
    fn parse_hex_input(&self, input: &str) -> Result<Vec<u8>, String> {
        let mut result = Vec::new();
        let mut current_number = String::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                // 공백 문자 처리
                ' ' | '\n' | '\t' | '\r' => {
                    if !current_number.is_empty() {
                        result.push(self.parse_hex_string(&current_number)?);
                        current_number.clear();
                    }
                    chars.next();
                }
                // "0x" 또는 "\x" 접두사 처리
                '0' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == 'x' || next == 'X' {
                            if !current_number.is_empty() {
                                result.push(self.parse_hex_string(&current_number)?);
                                current_number.clear();
                            }
                            chars.next(); // 'x' 건너뛰기
                            current_number = self.collect_hex_digits(&mut chars)?;
                            if !current_number.is_empty() {
                                result.push(self.parse_hex_string(&current_number)?);
                                current_number.clear();
                            }
                        } else {
                            current_number.push('0');
                        }
                    } else {
                        current_number.push('0');
                    }
                }
                '\\' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == 'x' || next == 'X' {
                            if !current_number.is_empty() {
                                result.push(self.parse_hex_string(&current_number)?);
                                current_number.clear();
                            }
                            chars.next(); // 'x' 건너뛰기
                            current_number = self.collect_hex_digits(&mut chars)?;
                            if !current_number.is_empty() {
                                result.push(self.parse_hex_string(&current_number)?);
                                current_number.clear();
                            }
                        } else {
                            return Err("Invalid hex format: expected 'x' after '\\'".to_string());
                        }
                    } else {
                        return Err("Unexpected end of input after '\\'".to_string());
                    }
                }
                'x' | 'X' => {
                    if !current_number.is_empty() {
                        result.push(self.parse_hex_string(&current_number)?);
                        current_number.clear();
                    }
                    chars.next(); // 'x' 건너뛰기
                    current_number = self.collect_hex_digits(&mut chars)?;
                    if !current_number.is_empty() {
                        result.push(self.parse_hex_string(&current_number)?);
                        current_number.clear();
                    }
                }
                // 16진수 숫자 수집
                _ => {
                    if c.is_ascii_hexdigit() {
                        current_number.push(chars.next().unwrap());
                    } else {
                        chars.next(); // 무시할 문자 건너뛰기
                        continue;
                    }

                    // 두 자리가 모이면 바이트로 변환
                    if current_number.len() == 2 {
                        result.push(self.parse_hex_string(&current_number)?);
                        current_number.clear();
                    }
                }
            }
        }

        // 남은 숫자 처리
        if !current_number.is_empty() {
            // 한 자리 숫자인 경우 앞에 0을 붙임
            if current_number.len() == 1 {
                current_number.insert(0, '0');
            }
            result.push(self.parse_hex_string(&current_number)?);
        }

        if result.is_empty() {
            return Err("No valid hex values found".to_string());
        }

        Ok(result)
    }

    fn collect_hex_digits(
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Result<String, String> {
        let mut hex_str = String::new();

        while let Some(&c) = chars.peek() {
            if c.is_ascii_hexdigit() {
                hex_str.push(chars.next().unwrap());
                if hex_str.len() == 2 {
                    break;
                }
            } else {
                break;
            }
        }

        if hex_str.is_empty() {
            return Err("Expected hex digits".to_string());
        }

        // 한 자리 숫자인 경우 앞에 0을 붙임
        if hex_str.len() == 1 {
            hex_str.insert(0, '0');
        }

        Ok(hex_str)
    }

    fn parse_hex_string(&self, hex_str: &str) -> Result<u8, String> {
        u8::from_str_radix(hex_str, 16).map_err(|_| format!("Invalid hex value: {}", hex_str))
    }

    fn parse_decimal_input(&self, input: &str) -> Result<Vec<u8>, String> {
        input
            .split_whitespace()
            .map(|s| {
                s.parse::<u8>()
                    .map_err(|e| format!("Invalid number '{}': {}", s, e))
            })
            .collect()
    }

    fn parse_binary_input(&self, input: &str) -> Result<Vec<u8>, String> {
        let mut result = Vec::new();
        let mut current_number = String::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                // 공백 문자 처리
                ' ' | '\n' | '\t' | '\r' => {
                    if !current_number.is_empty() {
                        result.push(self.parse_binary_string(&current_number)?);
                        current_number.clear();
                    }
                    chars.next();
                }
                // 이진수 숫자 수집
                '0' | '1' => {
                    current_number.push(chars.next().unwrap());
                    // 8비트가 모이면 바이트로 변환
                    if current_number.len() == 8 {
                        result.push(self.parse_binary_string(&current_number)?);
                        current_number.clear();
                    }
                }
                _ => {
                    chars.next(); // 무시할 문자 건너뛰기
                    continue;
                }
            }
        }

        // 남은 숫자 처리
        if !current_number.is_empty() {
            result.push(self.parse_binary_string(&current_number)?);
        }

        if result.is_empty() {
            return Err("No valid binary values found".to_string());
        }

        Ok(result)
    }

    fn parse_binary_string(&self, binary_str: &str) -> Result<u8, String> {
        u8::from_str_radix(binary_str, 2).map_err(|_| format!("Invalid binary value: {}", binary_str))
    }

    fn render_ascii_table(&self) -> Html {
        let mut rows = Vec::new();
        
        for i in 0..128 {
            let char_display = if i < 32 {
                // 제어 문자들의 설명
                match i {
                    0 => "NUL".to_string(),
                    1 => "SOH".to_string(),
                    2 => "STX".to_string(),
                    3 => "ETX".to_string(),
                    4 => "EOT".to_string(),
                    5 => "ENQ".to_string(),
                    6 => "ACK".to_string(),
                    7 => "BEL".to_string(),
                    8 => "BS".to_string(),
                    9 => "TAB".to_string(),
                    10 => "LF".to_string(),
                    11 => "VT".to_string(),
                    12 => "FF".to_string(),
                    13 => "CR".to_string(),
                    14 => "SO".to_string(),
                    15 => "SI".to_string(),
                    16 => "DLE".to_string(),
                    17 => "DC1".to_string(),
                    18 => "DC2".to_string(),
                    19 => "DC3".to_string(),
                    20 => "DC4".to_string(),
                    21 => "NAK".to_string(),
                    22 => "SYN".to_string(),
                    23 => "ETB".to_string(),
                    24 => "CAN".to_string(),
                    25 => "EM".to_string(),
                    26 => "SUB".to_string(),
                    27 => "ESC".to_string(),
                    28 => "FS".to_string(),
                    29 => "GS".to_string(),
                    30 => "RS".to_string(),
                    31 => "US".to_string(),
                    _ => "CTRL".to_string(),
                }
            } else if i == 127 {
                "DEL".to_string()
            } else {
                format!("{}", char::from(i as u8))
            };

            let description = match i {
                0 => "Null character",
                9 => "Horizontal tab",
                10 => "Line feed (LF)",
                13 => "Carriage return (CR)",
                32 => "Space",
                127 => "Delete",
                _ if i < 32 => "Control character",
                _ if i >= 33 && i <= 126 => "Printable character",
                _ => "",
            };

            let row_style = if i < 32 || i == 127 {
                "background-color: var(--color-third)"
            } else {
                ""
            };

            rows.push(html! {
                <tr style={row_style}>
                    <td style="padding: 3px; border: 1px solid #ddd; text-align: center; font-weight: bold;">
                        { char_display }
                    </td>
                    <td style="padding: 3px; border: 1px solid #ddd; text-align: center;">
                        { i }
                    </td>
                    <td style="padding: 3px; border: 1px solid #ddd; text-align: center;">
                        { format!("0x{:02X}", i) }
                    </td>
                    <td style="padding: 3px; border: 1px solid #ddd; text-align: center;">
                        { format!("{:08b}", i) }
                    </td>
                    <td style="padding: 3px; border: 1px solid #ddd; text-align: left; font-size: 11px;">
                        { description }
                    </td>
                </tr>
            });
        }

        html! {
            <>
                { for rows }
            </>
        }
    }
}
