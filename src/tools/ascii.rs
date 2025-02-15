use log::info;
use std::f64::consts::PI;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{window, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Quaternion {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, PartialEq)]
struct EulerAngles {
    roll: f64,
    pitch: f64,
    yaw: f64,
}

#[derive(Clone, PartialEq)]
pub enum AsciiMode {
    Decimal,
    Hex,
}

pub struct ToolAscii {
    input_ascii: String,
    output_text: String,
    input_text: String,
    output_ascii: String,
    convert: bool,
    mode: AsciiMode,
}

pub enum Msg {
    UpdateAscii(String),
    UpdateText(String),
    ModeChanged(AsciiMode),
    Convert,
    CopyToClipboard(String),
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
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateAscii(value) => {
                self.input_ascii = value.clone();

                if self.mode == AsciiMode::Decimal {
                    let input_bytes = self.parse_decimal_input(&value);
                    let parsed_bytes = match input_bytes {
                        Ok(bytes) => bytes,
                        Err(_) => Vec::new(),
                    };

                    if let Ok(text) = String::from_utf8(parsed_bytes) {
                        if text.len() > 0 {
                            self.output_text = text;
                            return true;
                        } else {
                            if self.input_ascii.len() == 0 {
                                self.output_text = "".to_string();
                                return true;
                            } else {
                                return false;
                            }
                        }
                    }
                } else {
                    let input_bytes = self.parse_hex_input(&value);
                    let parsed_bytes = match input_bytes {
                        Ok(bytes) => bytes,
                        Err(e) => Vec::new(),
                    };

                    if let Ok(text) = String::from_utf8(parsed_bytes) {
                        if text.len() > 0 {
                            self.output_text = text;
                            return true;
                        } else {
                            if self.input_ascii.len() == 0 {
                                self.output_text = "".to_string();
                                return true;
                            } else {
                                return false;
                            }
                        }
                    }
                }

                true
            }
            Msg::UpdateText(value) => {
                info!("update {}", value);
                self.input_text = value;

                let input_bytes = self.input_text.as_bytes().to_vec();

                if self.mode == AsciiMode::Decimal {
                    info!("decimal");
                    self.output_ascii = input_bytes
                        .iter()
                        .map(|byte| byte.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");
                } else {
                    info!("hex");
                    self.output_ascii = input_bytes
                        .iter()
                        .map(|byte| format!("0x{:02X}", byte))
                        .collect::<Vec<String>>()
                        .join(" ");
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
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let convert = self.convert.clone();
        let on_convert = _ctx.link().callback(|_| Msg::Convert);

        html! {
                <>
                    <div class="tool-wrapper">
                        <div>
                            <div class="tool-title">
                                { "ASCII Converter" }
                            </div>
                            <div class="tool-intro">
                                <p>
                                    {"This tool converts text to ASCII codes and vice versa, useful for analyzing or manipulating character data. ASCII (American Standard Code for Information Interchange) is widely used in computing and communication systems."}
                                </p>
                                <p> {"With this tool, you can:"} </p>
                                <ul>
                                    <li>{"Convert text to ASCII codes."}</li>
                                    <li>{"Decode ASCII codes into text."}</li>
                                </ul>
                                <p>
                                    {"Two modes are supported for ASCII codes:"}
                                </p>
                                <ul>
                                    <li><strong>{"Hex Mode:"}</strong> {" Uses hexadecimal format."}</li>
                                    <li><strong>{"Decimal Mode:"}</strong> {" Uses decimal format, with numbers separated by spaces (e.g., 65 66 67)."}</li>
                                </ul>
                                <p>
                                    {"Hexadecimal input formats include:"}
                                </p>
                                <ul>
                                    <li>{"0x41 \\x42 x43 44 45"}</li>
                                    <li>{"0x41\\x42x434445"}</li>
                                </ul>
                                <p>
                                    {"Ensure consistent input format for accurate conversions."}
                                </p>
                            </div>
                        </div>
                        <div class="tool-container">
                            <div style="display: flex; align-items: center; padding-left: 20px; padding-right: 20px; margin-bottom: 10px; margin-top: 5px;">
                                <div style="width: 90%;">
                                    if !convert {
                                        {"Text to ASCII"}
                                    } else {
                                        {"ASCII to Text"}
                                    }
                                </div>
                                <div onclick={on_convert} class="tool-change" style="width: 10%;">
                                    <i class="fa-solid fa-arrows-rotate"></i>
                                </div>
                            </div>
                            <div style="display: flex; align-items: center; padding-left: 20px; padding-right: 20px; margin-bottom: 10px; margin-top: 5px;">
                                <div style="width: 90%;">
                                    {"Input Method: "}
                                </div>
                                <select
                                    id="input-mode-select"
                                    onchange={_ctx.link().callback(|e: Event| {
                                        let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                        match value.as_str() {
                                            "decimal" => Msg::ModeChanged(AsciiMode::Decimal),
                                            "hex" => Msg::ModeChanged(AsciiMode::Hex),
                                            _ => unreachable!(),
                                        }
                                    })}>
                                    <option value="hex" selected={self.mode == AsciiMode::Hex}>{ "HEX" }</option>
                                    <option value="decimal" selected={self.mode == AsciiMode::Decimal}>{ "DECIMAL" }</option>
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
                                                if self.mode == AsciiMode::Hex {
                                                    "Enter ASCII code... (e.g., 0x41 \\x42 x43 44 45 or 0x41\\x42x434445)"
                                                } else {
                                                    "Enter ASCII code... (e.g., 65 66 67 with spaces)"
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
                        meta_tag.set_attribute("content", "This tool converts text to ASCII codes and vice versa, useful for analyzing or manipulating character data.").unwrap();
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
}
