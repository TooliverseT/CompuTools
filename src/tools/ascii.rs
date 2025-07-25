use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlInputElement, Storage};
use yew::prelude::*;
use crate::components::tool_category::ToolCategoryManager;

#[derive(Clone, PartialEq)]
pub enum AsciiMode {
    Decimal,
    Hex,
    Binary, // 새로운 바이너리 모드 추가
    Octal, // 8진수 모드 추가
}

#[derive(Clone, PartialEq)]
pub enum HexStyle {
    WithPrefix,     // 0x48
    ShortPrefix,    // x48
    NoPrefix,       // 48
    EscapeSequence, // \x48
}

#[derive(Clone, PartialEq)]
pub enum BinaryStyle {
    WithPrefix,    // 0b01001000
    ShortPrefix,   // b01001000
    NoPrefix,      // 01001000
}

#[derive(Clone, PartialEq)]
pub enum OctalStyle {
    WithPrefix,     // 0o110
    ShortPrefix,    // o110
    NoPrefix,       // 110
    EscapeSequence, // \110
}

pub struct ToolAscii {
    input_ascii: String,
    output_text: String,
    input_text: String,
    output_ascii: String,
    convert: bool,
    mode: AsciiMode,
    hex_style: HexStyle,
    binary_style: BinaryStyle,
    octal_style: OctalStyle,
    show_ascii_table: bool, // ASCII 테이블 표시 여부
    error_message: Option<String>, // 에러 메시지 추가
}

pub enum Msg {
    UpdateAscii(String),
    UpdateText(String),
    ModeChanged(AsciiMode),
    HexStyleChanged(HexStyle),
    BinaryStyleChanged(BinaryStyle),
    OctalStyleChanged(OctalStyle),
    Convert,
    CopyToClipboard(String),
    ToggleAsciiTable, // ASCII 테이블 토글
}

impl Component for ToolAscii {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::load_from_storage()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateAscii(value) => {
                self.input_ascii = value.clone();
                self.error_message = None; // 에러 메시지 초기화

                // 입력값이 비어있으면 출력도 초기화
                if value.is_empty() {
                    self.output_text = String::new();
                    return true;
                }

                let parsed_bytes = match self.mode {
                    AsciiMode::Decimal => self.parse_decimal_input(&value),
                    AsciiMode::Hex => self.parse_hex_input(&value),
                    AsciiMode::Binary => self.parse_binary_input(&value),
                    AsciiMode::Octal => self.parse_octal_input(&value),
                };

                match parsed_bytes {
                    Ok(bytes) => {
                        // Extended ASCII 지원 (0-255)
                        match String::from_utf8(bytes.clone()) {
                            Ok(text) => {
                            self.output_text = text;
                            }
                            Err(_) => {
                                // UTF-8로 변환할 수 없는 경우, 각 바이트를 문자로 변환 시도
                                let mut result = String::new();
                                for byte in bytes {
                                    if byte <= 127 {
                                        result.push(char::from(byte));
                        } else {
                                        // Extended ASCII (128-255)는 물음표로 표시
                                        result.push('?');
                                    }
                                }
                                self.output_text = result;
                            }
                        }
                    }
                    Err(err) => {
                        self.error_message = Some(err);
                        self.output_text = String::new();
                    }
                }

                true
            }
            Msg::UpdateText(value) => {
                self.input_text = value;
                self.error_message = None;

                let input_bytes = self.input_text.as_bytes().to_vec();
                self.output_ascii = self.convert_text_to_ascii(&input_bytes);
                true
            }
            Msg::ModeChanged(mode) => {
                self.mode = mode;
                self.input_ascii = "".to_string();
                self.output_text = "".to_string();
                self.error_message = None;

                // Text to ASCII 모드일 때, 기존 텍스트가 있으면 새 포맷으로 다시 변환
                if !self.convert && !self.input_text.is_empty() {
                    let input_bytes = self.input_text.as_bytes().to_vec();
                    self.output_ascii = self.convert_text_to_ascii(&input_bytes);
                }

                self.save_to_storage();
                true
            }
            Msg::HexStyleChanged(style) => {
                self.hex_style = style;
                // HEX 모드이고 Text to ASCII 모드일 때만 즉시 업데이트
                if self.mode == AsciiMode::Hex && !self.convert && !self.input_text.is_empty() {
                    let input_bytes = self.input_text.as_bytes().to_vec();
                    self.output_ascii = self.convert_text_to_ascii(&input_bytes);
                }
                self.save_to_storage();
                true
            }
            Msg::BinaryStyleChanged(style) => {
                self.binary_style = style;
                // BINARY 모드이고 Text to ASCII 모드일 때만 즉시 업데이트
                if self.mode == AsciiMode::Binary && !self.convert && !self.input_text.is_empty() {
                    let input_bytes = self.input_text.as_bytes().to_vec();
                    self.output_ascii = self.convert_text_to_ascii(&input_bytes);
                }
                self.save_to_storage();
                true
            }
            Msg::OctalStyleChanged(style) => {
                self.octal_style = style;
                // OCTAL 모드이고 Text to ASCII 모드일 때만 즉시 업데이트
                if self.mode == AsciiMode::Octal && !self.convert && !self.input_text.is_empty() {
                    let input_bytes = self.input_text.as_bytes().to_vec();
                    self.output_ascii = self.convert_text_to_ascii(&input_bytes);
                }
                self.save_to_storage();
                true
            }
            Msg::Convert => {
                self.convert = !self.convert;
                self.error_message = None;
                self.save_to_storage();
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
                                    {"Our ASCII converter supports bidirectional conversion between text and ASCII codes with flexible input formats and comprehensive error handling. The tool provides three output modes: hexadecimal, decimal, and binary representation."}
                                </p>
                                
                                <h3>{"🔥 Advanced Features:"}</h3>
                                <ul>
                                    <li><strong>{"Bidirectional Conversion:"}</strong> {" Convert text to ASCII codes and vice versa instantly"}</li>
                                    <li><strong>{"Quad Format Support:"}</strong> {" Choose between hexadecimal, decimal, binary, and octal representation"}</li>
                                    <li><strong>{"Customizable Output Styles:"}</strong> {" Select prefix styles for each format (0x vs x vs \\x for hex, 0b vs b for binary, 0o vs o vs \\ for octal)"}</li>
                                    <li><strong>{"Flexible Input Parsing:"}</strong> {" Support for various hex formats, space-separated decimals, and 8-bit binary strings"}</li>
                                    <li><strong>{"Real-time Error Feedback:"}</strong> {" Instant validation with detailed error messages and visual indicators"}</li>
                                    <li><strong>{"ASCII Table Viewer:"}</strong> {" Interactive reference table showing all ASCII characters with their codes"}</li>
                                    <li><strong>{"Extended ASCII Support:"}</strong> {" Handle characters in the 128-255 range with appropriate fallbacks"}</li>
                                    <li><strong>{"Smart Input Recognition:"}</strong> {" Automatically detect and parse mixed hex formats"}</li>
                                    <li><strong>{"Copy with Feedback:"}</strong> {" Click any output field to copy results with visual confirmation"}</li>
                                </ul>

                                <h3>{"📊 Supported Input Formats:"}</h3>
                                <div class="example-box">
                                    <p><strong>{"Hexadecimal formats supported:"}</strong></p>
                                    <ul>
                                        <li>{"0x48 0x65 0x6C 0x6C 0x6F (standard prefix)"}</li>
                                        <li>{"\\x48\\x65\\x6C\\x6C\\x6F (escape sequence style)"}</li>
                                        <li>{"x48x65x6Cx6Cx6F (x prefix only)"}</li>
                                        <li>{"48656C6C6F (raw hex, parsed in pairs)"}</li>
                                        <li>{"0x48 \\x65 x6C 6F (mixed formats)"}</li>
                                    </ul>
                                    <p><strong>{"Decimal format:"}</strong></p>
                                    <ul>
                                        <li>{"72 101 108 108 111 (space-separated numbers)"}</li>
                                        <li>{"Valid range: 0-255 (Extended ASCII)"}</li>
                                    </ul>
                                    <p><strong>{"Binary format:"}</strong></p>
                                    <ul>
                                        <li>{"0b01001000 0b01100101 (standard prefix)"}</li>
                                        <li>{"b01001000 b01100101 (short prefix)"}</li>
                                        <li>{"01001000 01100101 01101100 01101100 01101111 (8-bit binary strings)"}</li>
                                        <li>{"0100100001100101011011000110110001101111 (continuous binary stream)"}</li>
                                    </ul>
                                    <p><strong>{"Octal format:"}</strong></p>
                                    <ul>
                                        <li>{"0o110 0o145 0o154 (standard prefix)"}</li>
                                        <li>{"o110 o145 o154 (short prefix)"}</li>
                                        <li>{"110 145 154 154 157 (3-digit octal numbers)"}</li>
                                        <li>{"\\110\\145\\154 (escape sequence style)"}</li>
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
                                    <div style="max-height: 400px; overflow-y: auto; overflow-x: auto; font-family: monospace; font-size: 12px; border: 1px solid #ddd; border-radius: 5px;">
                                        <table style="width: 100%; border-collapse: collapse; min-width: 600px;">
                                            <thead>
                                                <tr style="background-color: var(--color-fourth); color: white; position: sticky; top: 0;">
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
                                <h2>{"💼 Professional Use Cases & Applications"}</h2>
                                <div class="use-case">
                                    <h3>{"1. Programming & Software Development"}</h3>
                                    <ul>
                                        <li><strong>{"Debugging Character Issues:"}</strong> {" Inspect character data in applications to identify encoding problems and invisible characters"}</li>
                                        <li><strong>{"Protocol Implementation:"}</strong> {" Build communication protocols that transmit text as numerical data"}</li>
                                        <li><strong>{"Data Validation:"}</strong> {" Verify that input contains only valid ASCII characters for compatibility"}</li>
                                        <li><strong>{"Escape Sequence Processing:"}</strong> {" Create or parse escape sequences in strings and configuration files"}</li>
                                        <li><strong>{"Binary File Analysis:"}</strong> {" Extract readable text from binary files and data streams"}</li>
                                        <li><strong>{"Configuration File Parsing:"}</strong> {" Handle special characters in config files and command-line arguments"}</li>
                                </ul>
                                </div>
                                
                                <div class="use-case">
                                    <h3>{"2. Cybersecurity & Forensics"}</h3>
                                    <ul>
                                        <li><strong>{"Payload Analysis:"}</strong> {" Decode suspicious data in security investigations and incident response"}</li>
                                        <li><strong>{"Log Analysis:"}</strong> {" Convert encoded log entries to readable text for threat hunting"}</li>
                                        <li><strong>{"Malware Research:"}</strong> {" Analyze obfuscated strings in malicious code and reverse engineering"}</li>
                                        <li><strong>{"Network Traffic Inspection:"}</strong> {" Decode ASCII-encoded data in network packets and protocols"}</li>
                                        <li><strong>{"Digital Forensics:"}</strong> {" Extract text from corrupted or encoded files in forensic investigations"}</li>
                                        <li><strong>{"Steganography Detection:"}</strong> {" Analyze hidden messages in text-based steganographic techniques"}</li>
                                    </ul>
                                </div>
                                
                                <div class="use-case">
                                    <h3>{"3. Data Processing & Integration"}</h3>
                                    <ul>
                                        <li><strong>{"File Format Analysis:"}</strong> {" Examine binary files containing ASCII text headers and metadata"}</li>
                                        <li><strong>{"Legacy System Integration:"}</strong> {" Work with older systems that use ASCII encoding for data exchange"}</li>
                                        <li><strong>{"Data Migration Projects:"}</strong> {" Convert text data between different encoding formats during system upgrades"}</li>
                                        <li><strong>{"Quality Assurance Testing:"}</strong> {" Verify text data integrity during transmission and storage"}</li>
                                        <li><strong>{"Database Import/Export:"}</strong> {" Handle special characters in CSV files and database dumps"}</li>
                                        <li><strong>{"API Development:"}</strong> {" Debug character encoding issues in REST APIs and web services"}</li>
                                    </ul>
                                </div>
                            </div>

                            <div class="content-section">
                                <h2>{"📚 Step-by-Step Tutorial"}</h2>
                                
                                <div class="tutorial-step">
                                    <h3>{"Example 1: Converting Text to ASCII with Style Options"}</h3>
                                    <p><strong>{"Goal:"}</strong> {" Convert the word 'Hello' to ASCII values with different output styles"}</p>
                                    <ol>
                                        <li>{"Make sure the tool is in 'Text to ASCII' mode (default)"}</li>
                                        <li>{"Enter 'Hello' in the text input field"}</li>
                                        <li>{"Select your preferred output format using the first dropdown:"}</li>
                                        <ul>
                                            <li>{"Choose 'HEX' for hexadecimal output"}</li>
                                            <li>{"Choose 'DECIMAL' for decimal output"}</li>
                                            <li>{"Choose 'BINARY' for binary output"}</li>
                                            <li>{"Choose 'OCTAL' for octal output"}</li>
                                        </ul>
                                        <li>{"Use the second dropdown to select your preferred style (when not in decimal mode)"}</li>
                                        <li>{"View the automatic conversion results"}</li>
                                    </ol>
                                    <div class="example-box">
                                        <p><strong>{"Input:"}</strong> {" Hello"}</p>
                                        <p><strong>{"Decimal Output:"}</strong> {" 72 101 108 108 111"}</p>
                                        <p><strong>{"Hex Output Styles:"}</strong></p>
                                        <ul>
                                            <li>{"0x48 0x65 0x6C 0x6C 0x6F (with 0x prefix)"}</li>
                                            <li>{"x48 x65 x6C x6C x6F (with x prefix)"}</li>
                                            <li>{"48 65 6C 6C 6F (no prefix)"}</li>
                                            <li>{"\\x48\\x65\\x6C\\x6C\\x6F (escape sequence)"}</li>
                                        </ul>
                                        <p><strong>{"Binary Output Styles:"}</strong></p>
                                        <ul>
                                            <li>{"0b01001000 0b01100101 0b01101100 0b01101100 0b01101111 (with 0b prefix)"}</li>
                                            <li>{"b01001000 b01100101 b01101100 b01101100 b01101111 (with b prefix)"}</li>
                                            <li>{"01001000 01100101 01101100 01101100 01101111 (no prefix)"}</li>
                                        </ul>
                                        <p><strong>{"Octal Output Styles:"}</strong></p>
                                        <ul>
                                            <li>{"0o110 0o145 0o154 0o154 0o157 (with 0o prefix)"}</li>
                                            <li>{"o110 o145 o154 o154 o157 (with o prefix)"}</li>
                                            <li>{"110 145 154 154 157 (no prefix)"}</li>
                                            <li>{"\\110\\145\\154\\154\\157 (escape sequence)"}</li>
                                        </ul>
                                    </div>
                                    <p><strong>{"Explanation:"}</strong> {" Each character maps to its ASCII value: H=72, e=101, l=108, l=108, o=111. The style dropdown lets you choose how the numbers are formatted."}</p>
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
                                            <li>{"For binary: '01010111 01101111 01110010 01101100 01100100'"}</li>
                                            <li>{"For octal: '0o127 0o157 0o162 0o154 0o144' or '\\127\\157\\162\\154\\144'"}</li>
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
                                            <li>{"0x48 0x65 0x6C 0x6C 0x6F (hex with 0x prefix)"}</li>
                                            <li>{"\\x48\\x65\\x6C\\x6C\\x6F (hex with escape sequence)"}</li>
                                            <li>{"x48x65x6Cx6Cx6F (hex with x prefix)"}</li>
                                            <li>{"48656C6C6F (raw hex)"}</li>
                                            <li>{"0b01001000 0b01100101 0b01101100 0b01101100 0b01101111 (binary)"}</li>
                                            <li>{"0o110 0o145 0o154 0o154 0o157 (octal)"}</li>
                                            <li>{"\\110\\145\\154\\154\\157 (octal escape sequence)"}</li>
                                            <li>{"0x48 \\x65 x6C 6F (mixed formats)"}</li>
                                        </ul>
                                    </div>
                                    <p><strong>{"Pro Tip:"}</strong> {" You can mix different hex formats in the same input. The parser automatically detects and handles each format correctly."}</p>
                                </div>

                                <div class="tutorial-step">
                                    <h3>{"Example 4: Handling Control Characters"}</h3>
                                    <p><strong>{"Goal:"}</strong> {" Work with non-printable control characters"}</p>
                                    <div class="example-box">
                                        <p><strong>{"Common control characters:"}</strong></p>
                                        <ul>
                                            <li>{"Tab: 9 (0x09)"}</li>
                                            <li>{"Newline: 10 (0x0A)"}</li>
                                            <li>{"Carriage Return: 13 (0x0D)"}</li>
                                            <li>{"Escape: 27 (0x1B)"}</li>
                                        </ul>
                                        <p><strong>{"Example input:"}</strong> {" 72 101 108 108 111 9 87 111 114 108 100"}</p>
                                        <p><strong>{"Output:"}</strong> {" Hello[TAB]World (tab character between words)"}</p>
                                    </div>
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
                                        {"A: This tool focuses on standard ASCII (0-127) and basic Extended ASCII (128-255). Characters outside this range (like accented letters, emojis, or non-Latin scripts) may not convert correctly. For international text, consider using Unicode tools or UTF-8 converters that can handle extended character sets."}
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
                                        {"A: The tool validates input and provides detailed error messages. Invalid ASCII values or malformed inputs are highlighted with clear explanations. Extended ASCII values (128-255) are supported but may display as placeholder characters if not properly encoded."}
                                    </p>
                                </div>
                            </div>

                            <div class="content-section">
                                <h2>{"🎯 Best Practices"}</h2>
                                <ul>
                                    <li><strong>{"Input validation:"}</strong> {" Always validate ASCII input ranges (0-127 for standard ASCII, 0-255 for extended) in your applications"}</li>
                                    <li><strong>{"Error handling:"}</strong> {" Implement proper error handling for invalid character codes and encoding issues"}</li>
                                    <li><strong>{"Performance:"}</strong> {" For large datasets, consider batch processing rather than character-by-character conversion"}</li>
                                    <li><strong>{"Documentation:"}</strong> {" Clearly document the expected encoding format in your code and APIs"}</li>
                                    <li><strong>{"Testing:"}</strong> {" Test with edge cases like control characters and boundary values (0, 127, 255)"}</li>
                                    <li><strong>{"Consistency:"}</strong> {" Choose one hex format style and stick with it throughout your project"}</li>
                                    <li><strong>{"Backup data:"}</strong> {" Always keep original data when performing encoding conversions"}</li>
                                    <li><strong>{"Security awareness:"}</strong> {" Be cautious with user input that may contain control characters or escape sequences"}</li>
                                </ul>
                            </div>

                            <div class="content-section">
                                <h2>{"🔗 Related Tools"}</h2>
                                <ul>
                                    {
                                        ToolCategoryManager::get_related_tools("ascii")
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
                                    if !convert {
                                        {"Output Format: "}
                                    } else {
                                        {"Input Format: "}
                                    }
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
                                            "octal" => Msg::ModeChanged(AsciiMode::Octal),
                                            _ => unreachable!(),
                                        }
                                    })}>
                                    <option value="hex" selected={self.mode == AsciiMode::Hex}>{ "HEX" }</option>
                                    <option value="decimal" selected={self.mode == AsciiMode::Decimal}>{ "DECIMAL" }</option>
                                    <option value="binary" selected={self.mode == AsciiMode::Binary}>{ "BINARY" }</option>
                                    <option value="octal" selected={self.mode == AsciiMode::Octal}>{ "OCTAL" }</option>
                                </select>
                            </div>
                            
                            // 스타일 선택 드롭다운 (Text to ASCII 모드일 때만 표시)
                            if !convert && self.mode == AsciiMode::Hex {
                                <div style="display: flex; align-items: center; margin-bottom: 10px;">
                                    <div style="width: 70%;">
                                        {"Hex Style: "}
                                    </div>
                                    <select
                                        style="width: 30%;"
                                        onchange={_ctx.link().callback(|e: Event| {
                                            let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                            match value.as_str() {
                                                "with_prefix" => Msg::HexStyleChanged(HexStyle::WithPrefix),
                                                "short_prefix" => Msg::HexStyleChanged(HexStyle::ShortPrefix),
                                                "no_prefix" => Msg::HexStyleChanged(HexStyle::NoPrefix),
                                                "escape_sequence" => Msg::HexStyleChanged(HexStyle::EscapeSequence),
                                                _ => unreachable!(),
                                            }
                                        })}>
                                        <option value="with_prefix" selected={self.hex_style == HexStyle::WithPrefix}>{ "0x48" }</option>
                                        <option value="short_prefix" selected={self.hex_style == HexStyle::ShortPrefix}>{ "x48" }</option>
                                        <option value="no_prefix" selected={self.hex_style == HexStyle::NoPrefix}>{ "48" }</option>
                                        <option value="escape_sequence" selected={self.hex_style == HexStyle::EscapeSequence}>{ "\\x48" }</option>
                                    </select>
                                </div>
                            }
                            
                            if !convert && self.mode == AsciiMode::Binary {
                                <div style="display: flex; align-items: center; margin-bottom: 10px;">
                                    <div style="width: 70%;">
                                        {"Binary Style: "}
                                    </div>
                                    <select
                                        style="width: 30%;"
                                        onchange={_ctx.link().callback(|e: Event| {
                                            let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                            match value.as_str() {
                                                "with_prefix" => Msg::BinaryStyleChanged(BinaryStyle::WithPrefix),
                                                "short_prefix" => Msg::BinaryStyleChanged(BinaryStyle::ShortPrefix),
                                                "no_prefix" => Msg::BinaryStyleChanged(BinaryStyle::NoPrefix),
                                                _ => unreachable!(),
                                            }
                                        })}>
                                        <option value="with_prefix" selected={self.binary_style == BinaryStyle::WithPrefix}>{ "0b01000001" }</option>
                                        <option value="short_prefix" selected={self.binary_style == BinaryStyle::ShortPrefix}>{ "b01000001" }</option>
                                        <option value="no_prefix" selected={self.binary_style == BinaryStyle::NoPrefix}>{ "01000001" }</option>
                                    </select>
                                </div>
                            }
                            
                            if !convert && self.mode == AsciiMode::Octal {
                                <div style="display: flex; align-items: center; margin-bottom: 10px;">
                                    <div style="width: 70%;">
                                        {"Octal Style: "}
                                    </div>
                                    <select
                                        style="width: 30%;"
                                        onchange={_ctx.link().callback(|e: Event| {
                                            let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                            match value.as_str() {
                                                "with_prefix" => Msg::OctalStyleChanged(OctalStyle::WithPrefix),
                                                "short_prefix" => Msg::OctalStyleChanged(OctalStyle::ShortPrefix),
                                                "no_prefix" => Msg::OctalStyleChanged(OctalStyle::NoPrefix),
                                                "escape_sequence" => Msg::OctalStyleChanged(OctalStyle::EscapeSequence),
                                                _ => unreachable!(),
                                            }
                                        })}>
                                        <option value="with_prefix" selected={self.octal_style == OctalStyle::WithPrefix}>{ "0o101" }</option>
                                        <option value="short_prefix" selected={self.octal_style == OctalStyle::ShortPrefix}>{ "o101" }</option>
                                        <option value="no_prefix" selected={self.octal_style == OctalStyle::NoPrefix}>{ "101" }</option>
                                        <option value="escape_sequence" selected={self.octal_style == OctalStyle::EscapeSequence}>{ "\\101" }</option>
                                    </select>
                                </div>
                            }
                            if !convert {
                                <div class="tool-inner">
                                    <div>
                                        <div class="tool-subtitle" style="margin-bottom: 5px;">{ "Text" }</div>
                                        <textarea
                                            type="text"
                                            style="overflow: auto;"
                                            value={self.input_text.clone()}
                                            placeholder={ "Enter text to convert to ASCII codes..."}
                                            oninput={_ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                Msg::UpdateText(input.value())
                                            })}
                                        />
                                    </div>
                                </div>
                                <div class="tool-inner" style="margin-top: 10px;">
                                    <div>
                                        <div class="tool-subtitle">{ "ASCII Codes" }</div>
                                        <textarea
                                            type="text"
                                            readonly=true
                                            style="overflow: auto; cursor: pointer;"
                                            value={self.output_ascii.clone()}
                                            placeholder="ASCII codes will appear here..."
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
                                        <div class="tool-subtitle" style="margin-bottom: 5px;">{ "ASCII Codes" }</div>
                                        <textarea
                                            type="text"
                                            style={if self.error_message.is_some() { 
                                                "overflow: auto; border: 2px solid var(--color-error);" 
                                            } else { 
                                                "overflow: auto;" 
                                            }}
                                            value={self.input_ascii.clone()}
                                            placeholder={
                                                match self.mode {
                                                    AsciiMode::Hex => "Enter ASCII codes... (e.g., 0x41 \\x42 x43 44)",
                                                    AsciiMode::Decimal => "Enter ASCII codes... (e.g., 65 66 67 with spaces)",
                                                    AsciiMode::Binary => "Enter ASCII codes... (e.g., 0b01000001 b01000010 01000011)",
                                                    AsciiMode::Octal => "Enter ASCII codes... (e.g., 0o101 o102 103 \\104)",
                                                }
                                            }
                                            oninput={_ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                Msg::UpdateAscii(input.value())
                                            })}
                                        />
                                        if let Some(error_msg) = &self.error_message {
                                            <div style="color: var(--color-error); font-size: 12px; margin-top: 4px; line-height: 1.3;">
                                                { error_msg }
                                            </div>
                                        }
                                        <div style="color: var(--color-subfont); font-size: 11px; margin-top: 2px;">
                                            { match self.mode {
                                                AsciiMode::Hex => "Supports: 0x41, \\x41, x41, 41 formats",
                                                AsciiMode::Decimal => "Valid range: 0-255 (space-separated)",
                                                AsciiMode::Binary => "Supports: 0b01000001, b01000001, 01000001 formats",
                                                AsciiMode::Octal => "Supports: 0o101, o101, 101, \\101 formats",
                                            }}
                                        </div>
                                    </div>
                                </div>
                                <div class="tool-inner" style="margin-top: 10px;">
                                    <div>
                                        <div class="tool-subtitle">{ "Converted Text" }</div>
                                        <textarea
                                            type="text"
                                            readonly=true
                                            style="overflow: auto; cursor: pointer;"
                                            value={self.output_text.clone()}
                                            placeholder="Converted text will appear here..."
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
                        meta_tag.set_attribute("content", "Advanced ASCII Converter with customizable output styles and real-time error feedback. Support for hex (0x/x), decimal, binary (0b/b), and octal (0o/o) formats with flexible prefix options. Flexible input parsing supports mixed formats and prefixes. Convert text to ASCII codes and vice versa instantly. Features comprehensive tutorials, professional use cases, interactive ASCII table, and robust input validation. Essential tool for programming, cybersecurity, debugging, and data analysis.").unwrap();
                    }
                }
            }
        }
    }
}

impl ToolAscii {
    // Local Storage 키 상수들
    const STORAGE_KEY_MODE: &'static str = "ascii_mode";
    const STORAGE_KEY_HEX_STYLE: &'static str = "ascii_hex_style";
    const STORAGE_KEY_BINARY_STYLE: &'static str = "ascii_binary_style";
    const STORAGE_KEY_OCTAL_STYLE: &'static str = "ascii_octal_style";
    const STORAGE_KEY_CONVERT: &'static str = "ascii_convert";

    fn get_local_storage() -> Option<Storage> {
        window()?.local_storage().ok()?
    }

    fn load_from_storage() -> Self {
        let storage = Self::get_local_storage();
        
        let mode = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_MODE).ok().flatten())
            .and_then(|s| match s.as_str() {
                "decimal" => Some(AsciiMode::Decimal),
                "hex" => Some(AsciiMode::Hex),
                "binary" => Some(AsciiMode::Binary),
                "octal" => Some(AsciiMode::Octal),
                _ => None,
            })
            .unwrap_or(AsciiMode::Hex);

        let hex_style = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_HEX_STYLE).ok().flatten())
            .and_then(|s| match s.as_str() {
                "with_prefix" => Some(HexStyle::WithPrefix),
                "short_prefix" => Some(HexStyle::ShortPrefix),
                "no_prefix" => Some(HexStyle::NoPrefix),
                "escape_sequence" => Some(HexStyle::EscapeSequence),
                _ => None,
            })
            .unwrap_or(HexStyle::WithPrefix);

        let binary_style = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_BINARY_STYLE).ok().flatten())
            .and_then(|s| match s.as_str() {
                "with_prefix" => Some(BinaryStyle::WithPrefix),
                "short_prefix" => Some(BinaryStyle::ShortPrefix),
                "no_prefix" => Some(BinaryStyle::NoPrefix),
                _ => None,
            })
            .unwrap_or(BinaryStyle::WithPrefix);

        let octal_style = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_OCTAL_STYLE).ok().flatten())
            .and_then(|s| match s.as_str() {
                "with_prefix" => Some(OctalStyle::WithPrefix),
                "short_prefix" => Some(OctalStyle::ShortPrefix),
                "no_prefix" => Some(OctalStyle::NoPrefix),
                "escape_sequence" => Some(OctalStyle::EscapeSequence),
                _ => None,
            })
            .unwrap_or(OctalStyle::WithPrefix);

        let convert = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_CONVERT).ok().flatten())
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(false);

        Self {
            input_ascii: String::new(),
            output_text: String::new(),
            input_text: String::new(),
            output_ascii: String::new(),
            convert,
            mode,
            hex_style,
            binary_style,
            octal_style,
            show_ascii_table: false,
            error_message: None,
        }
    }

    fn save_to_storage(&self) {
        if let Some(storage) = Self::get_local_storage() {
            let mode_str = match self.mode {
                AsciiMode::Decimal => "decimal",
                AsciiMode::Hex => "hex",
                AsciiMode::Binary => "binary",
                AsciiMode::Octal => "octal",
            };
            let _ = storage.set_item(Self::STORAGE_KEY_MODE, mode_str);

            let hex_style_str = match self.hex_style {
                HexStyle::WithPrefix => "with_prefix",
                HexStyle::ShortPrefix => "short_prefix",
                HexStyle::NoPrefix => "no_prefix",
                HexStyle::EscapeSequence => "escape_sequence",
            };
            let _ = storage.set_item(Self::STORAGE_KEY_HEX_STYLE, hex_style_str);

            let binary_style_str = match self.binary_style {
                BinaryStyle::WithPrefix => "with_prefix",
                BinaryStyle::ShortPrefix => "short_prefix",
                BinaryStyle::NoPrefix => "no_prefix",
            };
            let _ = storage.set_item(Self::STORAGE_KEY_BINARY_STYLE, binary_style_str);

            let octal_style_str = match self.octal_style {
                OctalStyle::WithPrefix => "with_prefix",
                OctalStyle::ShortPrefix => "short_prefix",
                OctalStyle::NoPrefix => "no_prefix",
                OctalStyle::EscapeSequence => "escape_sequence",
            };
            let _ = storage.set_item(Self::STORAGE_KEY_OCTAL_STYLE, octal_style_str);

            let _ = storage.set_item(Self::STORAGE_KEY_CONVERT, &self.convert.to_string());
        }
    }

    fn convert_text_to_ascii(&self, input_bytes: &[u8]) -> String {
        match self.mode {
            AsciiMode::Decimal => {
                input_bytes
                    .iter()
                    .map(|byte| byte.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            }
            AsciiMode::Hex => {
                input_bytes
                    .iter()
                    .map(|byte| match self.hex_style {
                        HexStyle::WithPrefix => format!("0x{:02X}", byte),
                        HexStyle::ShortPrefix => format!("x{:02X}", byte),
                        HexStyle::NoPrefix => format!("{:02X}", byte),
                        HexStyle::EscapeSequence => format!("\\x{:02X}", byte),
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            }
            AsciiMode::Binary => {
                input_bytes
                    .iter()
                    .map(|byte| match self.binary_style {
                        BinaryStyle::WithPrefix => format!("0b{:08b}", byte),
                        BinaryStyle::ShortPrefix => format!("b{:08b}", byte),
                        BinaryStyle::NoPrefix => format!("{:08b}", byte),
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            }
            AsciiMode::Octal => {
                input_bytes
                    .iter()
                    .map(|byte| match self.octal_style {
                        OctalStyle::WithPrefix => format!("0o{:03o}", byte),
                        OctalStyle::ShortPrefix => format!("o{:03o}", byte),
                        OctalStyle::NoPrefix => format!("{:03o}", byte),
                        OctalStyle::EscapeSequence => format!("\\{:03o}", byte),
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            }
        }
    }

    fn parse_hex_input(&self, input: &str) -> Result<Vec<u8>, String> {
        if input.trim().is_empty() {
            return Ok(Vec::new());
        }

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
                            return Err("Invalid escape sequence: expected 'x' after '\\'".to_string());
                        }
                    } else {
                        return Err("Incomplete escape sequence: unexpected end of input after '\\'".to_string());
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
                        return Err(format!("Invalid character '{}' in hexadecimal input", c));
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
            return Err("No valid hex values found in input".to_string());
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
            return Err("Expected hexadecimal digits after prefix".to_string());
        }

        // 한 자리 숫자인 경우 앞에 0을 붙임
        if hex_str.len() == 1 {
            hex_str.insert(0, '0');
        }

        Ok(hex_str)
    }

    fn parse_hex_string(&self, hex_str: &str) -> Result<u8, String> {
        u8::from_str_radix(hex_str, 16).map_err(|_| format!("Invalid hexadecimal value: '{}'", hex_str))
    }

    fn parse_decimal_input(&self, input: &str) -> Result<Vec<u8>, String> {
        if input.trim().is_empty() {
            return Ok(Vec::new());
        }

        input
            .split_whitespace()
            .map(|s| {
                s.parse::<u16>()
                    .map_err(|_| format!("Invalid decimal number: '{}'", s))
                    .and_then(|num| {
                        if num > 255 {
                            Err(format!("Decimal value {} exceeds maximum ASCII range (0-255)", num))
                        } else {
                            Ok(num as u8)
                        }
                    })
            })
            .collect()
    }

    fn parse_binary_input(&self, input: &str) -> Result<Vec<u8>, String> {
        if input.trim().is_empty() {
            return Ok(Vec::new());
        }

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
                // "0b" 접두사 처리
                '0' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == 'b' || next == 'B' {
                            if !current_number.is_empty() {
                                result.push(self.parse_binary_string(&current_number)?);
                                current_number.clear();
                            }
                            chars.next(); // 'b' 건너뛰기
                            current_number = self.collect_binary_digits(&mut chars)?;
                            if !current_number.is_empty() {
                                result.push(self.parse_binary_string(&current_number)?);
                                current_number.clear();
                            }
                        } else {
                            // 일반 '0' 이진수 숫자
                            current_number.push('0');
                            if current_number.len() == 8 {
                                result.push(self.parse_binary_string(&current_number)?);
                                current_number.clear();
                            }
                        }
                    } else {
                        // 마지막 문자가 '0'인 경우
                        current_number.push('0');
                        if current_number.len() == 8 {
                            result.push(self.parse_binary_string(&current_number)?);
                            current_number.clear();
                        }
                    }
                }
                'b' | 'B' => {
                    if !current_number.is_empty() {
                        result.push(self.parse_binary_string(&current_number)?);
                        current_number.clear();
                    }
                    chars.next(); // 'b' 건너뛰기
                    current_number = self.collect_binary_digits(&mut chars)?;
                    if !current_number.is_empty() {
                        result.push(self.parse_binary_string(&current_number)?);
                        current_number.clear();
                    }
                }
                // 이진수 숫자 수집
                '1' => {
                    current_number.push(chars.next().unwrap());
                    // 8비트가 모이면 바이트로 변환
                    if current_number.len() == 8 {
                        result.push(self.parse_binary_string(&current_number)?);
                        current_number.clear();
                    }
                }
                _ => {
                    return Err(format!("Invalid character '{}' in binary input. Only 0 and 1 are allowed.", c));
                }
            }
        }

        // 남은 숫자 처리
        if !current_number.is_empty() {
            if current_number.len() > 8 {
                return Err(format!("Binary sequence '{}' is longer than 8 bits", current_number));
            }
            result.push(self.parse_binary_string(&current_number)?);
        }

        if result.is_empty() {
            return Err("No valid binary values found in input".to_string());
        }

        Ok(result)
    }

    fn collect_binary_digits(
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Result<String, String> {
        let mut binary_str = String::new();

        while let Some(&c) = chars.peek() {
            if c == '0' || c == '1' {
                binary_str.push(chars.next().unwrap());
                if binary_str.len() == 8 {
                    break;
                }
            } else {
                break;
            }
        }

        if binary_str.is_empty() {
            return Err("Expected binary digits after prefix".to_string());
        }

        Ok(binary_str)
    }

    fn parse_binary_string(&self, binary_str: &str) -> Result<u8, String> {
        if binary_str.len() > 8 {
            return Err(format!("Binary sequence '{}' exceeds 8 bits", binary_str));
        }
        u8::from_str_radix(binary_str, 2).map_err(|_| format!("Invalid binary value: '{}'", binary_str))
    }

    fn parse_octal_input(&self, input: &str) -> Result<Vec<u8>, String> {
        if input.trim().is_empty() {
            return Ok(Vec::new());
        }

        let mut result = Vec::new();
        let mut current_number = String::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                // 공백 문자 처리
                ' ' | '\n' | '\t' | '\r' => {
                    if !current_number.is_empty() {
                        result.push(self.parse_octal_string(&current_number)?);
                        current_number.clear();
                    }
                    chars.next();
                }
                // "\NNN" escape sequence 처리
                '\\' => {
                    if !current_number.is_empty() {
                        result.push(self.parse_octal_string(&current_number)?);
                        current_number.clear();
                    }
                    chars.next(); // '\' 건너뛰기
                    current_number = self.collect_octal_digits(&mut chars)?;
                    if !current_number.is_empty() {
                        result.push(self.parse_octal_string(&current_number)?);
                        current_number.clear();
                    }
                }
                // "0o" 접두사 처리
                '0' => {
                    chars.next();
                    if let Some(&next) = chars.peek() {
                        if next == 'o' || next == 'O' {
                            if !current_number.is_empty() {
                                result.push(self.parse_octal_string(&current_number)?);
                                current_number.clear();
                            }
                            chars.next(); // 'o' 건너뛰기
                            current_number = self.collect_octal_digits(&mut chars)?;
                            if !current_number.is_empty() {
                                result.push(self.parse_octal_string(&current_number)?);
                                current_number.clear();
                            }
                        } else {
                            current_number.push('0');
                        }
                    } else {
                        current_number.push('0');
                    }
                }
                'o' | 'O' => {
                    if !current_number.is_empty() {
                        result.push(self.parse_octal_string(&current_number)?);
                        current_number.clear();
                    }
                    chars.next(); // 'o' 건너뛰기
                    current_number = self.collect_octal_digits(&mut chars)?;
                    if !current_number.is_empty() {
                        result.push(self.parse_octal_string(&current_number)?);
                        current_number.clear();
                    }
                }
                // 8진수 숫자 수집
                '0'..='7' => {
                    current_number.push(chars.next().unwrap());
                    // 3자리가 모이면 바이트로 변환 (8진수 377 = 255)
                    if current_number.len() == 3 {
                        result.push(self.parse_octal_string(&current_number)?);
                        current_number.clear();
                    }
                }
                _ => {
                    return Err(format!("Invalid character '{}' in octal input. Only 0-7 are allowed.", c));
                }
            }
        }

        // 남은 숫자 처리
        if !current_number.is_empty() {
            if current_number.len() > 3 {
                return Err(format!("Octal sequence '{}' is longer than 3 digits", current_number));
            }
            result.push(self.parse_octal_string(&current_number)?);
        }

        if result.is_empty() {
            return Err("No valid octal values found in input".to_string());
        }

        Ok(result)
    }

    fn collect_octal_digits(
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Result<String, String> {
        let mut octal_str = String::new();

        while let Some(&c) = chars.peek() {
            if c >= '0' && c <= '7' {
                octal_str.push(chars.next().unwrap());
                if octal_str.len() == 3 {
                    break;
                }
            } else {
                break;
            }
        }

        if octal_str.is_empty() {
            return Err("Expected octal digits after prefix".to_string());
        }

        Ok(octal_str)
    }

    fn parse_octal_string(&self, octal_str: &str) -> Result<u8, String> {
        if octal_str.len() > 3 {
            return Err(format!("Octal sequence '{}' exceeds 3 digits", octal_str));
        }
        u8::from_str_radix(octal_str, 8).map_err(|_| format!("Invalid octal value: '{}'", octal_str))
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
                1 => "Start of Heading",
                2 => "Start of Text",
                3 => "End of Text",
                4 => "End of Transmission",
                5 => "Enquiry",
                6 => "Acknowledge",
                7 => "Bell",
                8 => "Backspace",
                9 => "Horizontal tab",
                10 => "Line feed (LF)",
                11 => "Vertical tab",
                12 => "Form feed",
                13 => "Carriage return (CR)",
                14 => "Shift Out",
                15 => "Shift In",
                16 => "Data Link Escape",
                17 => "Device Control 1",
                18 => "Device Control 2",
                19 => "Device Control 3",
                20 => "Device Control 4",
                21 => "Negative Acknowledge",
                22 => "Synchronous Idle",
                23 => "End of Transmission Block",
                24 => "Cancel",
                25 => "End of Medium",
                26 => "Substitute",
                27 => "Escape",
                28 => "File Separator",
                29 => "Group Separator",
                30 => "Record Separator",
                31 => "Unit Separator",
                32 => "Space",
                127 => "Delete",
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
