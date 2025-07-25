use gloo_timers::callback::Timeout;
use log::info;
use std::collections::{HashMap, BTreeMap};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;
use crate::components::tool_category::ToolCategoryManager;

pub struct ToolBase {
    bases: BTreeMap<u32, String>,
    custom_bases: Vec<u32>, // 동적으로 추가된 진수 목록
    error_messages: BTreeMap<u32, Option<String>>, // 각 진수별 에러 메시지
    decimal_precision: u32, // 소수점 정밀도 (기본값: 6)
    supports_float: bool, // 부동소수점 모드 활성화 여부
}

pub enum Msg {
    UpdateBase(u32, String),
    AddCustomBase(u32),
    RemoveCustomBase(u32),
    SetPrecision(u32),
    ToggleFloatMode,
}

impl Component for ToolBase {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut bases = BTreeMap::new();
        let mut error_messages = BTreeMap::new();
        for base in 2..=36 {
            bases.insert(base, String::new());
            error_messages.insert(base, None);
        }
        Self { bases, custom_bases: vec![], error_messages, decimal_precision: 6, supports_float: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateBase(base, value) => {
                // 현재 입력 중인 필드의 값 업데이트
                if let Some(current_field) = self.bases.get_mut(&base) {
                    *current_field = value.clone();
                }

                // 입력값이 비어있으면 모든 다른 필드 초기화
                if value.is_empty() {
                    for (other_base, val) in self.bases.iter_mut() {
                        if *other_base != base {
                            val.clear();
                        }
                    }
                    for (_, err) in self.error_messages.iter_mut() {
                        *err = None;
                    }
                    return true;
                }

                // 입력 검증
                match self.validate_input(&value, base) {
                    Ok(_) => {
                        // 유효한 입력인 경우
                        self.error_messages.insert(base, None);
                        
                        if self.supports_float {
                            // 부동소수점 모드
                            match self.parse_float_input(&value, base) {
                                Ok(float_value) => {
                                    self.update_all_float_except(float_value, Some(base));
                                }
                                Err(_) => {
                                    // 파싱 실패 시 다른 필드들만 초기화
                                    for (other_base, val) in self.bases.iter_mut() {
                                        if *other_base != base {
                                            val.clear();
                                        }
                                    }
                                }
                            }
                        } else {
                            // 정수 모드 (기존 로직)
                            // 음수 부호 처리
                            let (is_negative, number_part) = if value.trim().starts_with('-') {
                                (true, &value.trim()[1..])
                            } else {
                                (false, value.trim())
                            };

                            // 유연한 포맷 파싱
                            if let Ok(cleaned_input) = self.parse_flexible_format(number_part, base) {
                                // prefix만 입력된 경우 (예: "0b", "0x") - 다른 필드들만 초기화
                                if cleaned_input.is_empty() {
                                    for (other_base, val) in self.bases.iter_mut() {
                                        if *other_base != base {
                                            val.clear();
                                        }
                                    }
                                } else if let Ok(mut num) = i64::from_str_radix(&cleaned_input, base) {
                                    if is_negative {
                                        num = -num;
                                    }
                                    self.update_all_except(num, Some(base));
                                }
                            }
                        }
                    }
                    Err(error_msg) => {
                        // 잘못된 입력인 경우
                        self.error_messages.insert(base, Some(error_msg));
                        
                        // 다른 필드들 초기화
                        for (other_base, val) in self.bases.iter_mut() {
                            if *other_base != base {
                        val.clear();
                            }
                        }
                        for (other_base, err) in self.error_messages.iter_mut() {
                            if *other_base != base {
                                *err = None;
                            }
                        }
                    }
                }
                true
            }
            Msg::AddCustomBase(base) => {
                if !self.custom_bases.contains(&base) {
                    self.custom_bases.push(base);
                }
                true
            }
            Msg::RemoveCustomBase(base) => {
                self.custom_bases.retain(|&b| b != base);
                true
            }
            Msg::SetPrecision(precision) => {
                self.decimal_precision = precision;
                true
            }
            Msg::ToggleFloatMode => {
                self.supports_float = !self.supports_float;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let common_bases = [10, 2, 8, 16];
        let all_bases: Vec<u32> = (2..=36).collect();
        let available_bases: Vec<u32> = all_bases.iter().cloned().filter(|b| !common_bases.contains(b) && !self.custom_bases.contains(b)).collect();
        let on_add = ctx.link().callback(|e: Event| {
            let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            let value = select.value();
            if let Ok(base) = value.parse::<u32>() {
                Msg::AddCustomBase(base)
            } else {
                Msg::AddCustomBase(2) // fallback, should not happen
            }
        });
        html! {
            <>
                <h1 class="tool-title">{ "Number Base Converter" }</h1>
                <div class="tool-wrapper">
                        <div class="tool-intro">
                        <div class="content-section">
                            <h2>{"🔢 What is Base Conversion?"}</h2>
                            <p>{"Base conversion is the process of translating numbers from one numeral system (base) to another. Each base uses a different set of digits and follows specific place value rules. Understanding base conversion is fundamental in computer science, mathematics, and digital electronics."}</p>
                            
                            <h3>{"Common Number Bases:"}</h3>
                            <ul>
                                <li><strong>{"Binary (Base 2):"}</strong> {" Uses digits 0-1. Foundation of all digital systems."}</li>
                                <li><strong>{"Octal (Base 8):"}</strong> {" Uses digits 0-7. Common in Unix file permissions."}</li>
                                <li><strong>{"Decimal (Base 10):"}</strong> {" Uses digits 0-9. Our everyday number system."}</li>
                                <li><strong>{"Hexadecimal (Base 16):"}</strong> {" Uses 0-9, A-F. Essential for programming and memory addresses."}</li>
                            </ul>

                            <div class="example-box">
                                <p><strong>{"The same value in different bases:"}</strong></p>
                                <ul>
                                    <li>{"Decimal: 42"}</li>
                                    <li>{"Binary: 101010 (1×32 + 0×16 + 1×8 + 0×4 + 1×2 + 0×1)"}</li>
                                    <li>{"Octal: 52 (5×8¹ + 2×8⁰)"}</li>
                                    <li>{"Hexadecimal: 2A (2×16¹ + 10×16⁰)"}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"⚙️ How This Number Base Converter Works"}</h2>
                            <p>{"Our converter supports both integer and floating-point conversions with advanced features for professional and educational use."}</p>
                            
                            <h3>{"🔥 Advanced Features:"}</h3>
                            <ul>
                                <li><strong>{"Dual Mode Operation:"}</strong> {" Switch between integer and floating-point conversion modes."}</li>
                                <li><strong>{"Flexible Input Formats:"}</strong> {" Supports 0x, 0b, 0o prefixes and escape sequences (\\x) for convenient input."}</li>
                                <li><strong>{"Complete Base Support:"}</strong> {" Convert between any base from 2 to 36, including uncommon bases."}</li>
                                <li><strong>{"Floating-Point Precision:"}</strong> {" Configurable decimal precision (3-12 digits) with repeating decimal detection."}</li>
                                <li><strong>{"Real-time Validation:"}</strong> {" Instant error detection with detailed feedback on invalid characters."}</li>
                                <li><strong>{"Dynamic Base Addition:"}</strong> {" Add/remove custom bases (Base 3, 5, 7, etc.) as needed."}</li>
                                <li><strong>{"Negative Number Support:"}</strong> {" Full support for negative values in all bases."}</li>
                                <li><strong>{"Visual Error Feedback:"}</strong> {" Color-coded input validation with specific error messages."}</li>
                            </ul>

                            <h3>{"📊 Supported Input Formats:"}</h3>
                            <div class="example-box">
                                <p><strong>{"Binary formats:"}</strong></p>
                                <ul>
                                    <li>{"0b101010 (standard prefix)"}</li>
                                    <li>{"b101010 (short prefix)"}</li>
                                    <li>{"101010 (plain format)"}</li>
                                    <li>{"101010.101 (floating-point in Float Mode)"}</li>
                                </ul>
                                <p><strong>{"Hexadecimal formats:"}</strong></p>
                                <ul>
                                    <li>{"0x2A (programming style)"}</li>
                                    <li>{"x2A (short prefix)"}</li>
                                    <li>{"\\x2A (escape sequence)"}</li>
                                    <li>{"2A.A (floating-point in Float Mode)"}</li>
                                </ul>
                                <p><strong>{"Octal formats:"}</strong></p>
                                <ul>
                                    <li>{"0o52 (modern prefix)"}</li>
                                    <li>{"o52 (short prefix)"}</li>
                                    <li>{"052 (traditional C-style)"}</li>
                                    <li>{"\\052 (escape sequence)"}</li>
                                    <li>{"52.4 (floating-point in Float Mode)"}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"📚 Step-by-Step Conversion Tutorial"}</h2>
                            
                            <div class="tutorial-step">
                                <h3>{"🔄 Example 1: Decimal to Binary Conversion"}</h3>
                                <p><strong>{"Goal:"}</strong> {" Convert decimal 42 to binary using division method"}</p>
                                
                                <h4>{"Mathematical Process:"}</h4>
                                <div class="example-box">
                                    <p><strong>{"Step-by-step division by 2:"}</strong></p>
                                    <pre style="color: var(--color-font); white-space: pre; font-family: inherit; margin: 0; padding-left: 40px;">
{r#"42 ÷ 2 = 21 remainder 0  ←
21 ÷ 2 = 10 remainder 1  ←
10 ÷ 2 = 5  remainder 0  ←
5  ÷ 2 = 2  remainder 1  ←
2  ÷ 2 = 1  remainder 0  ←
1  ÷ 2 = 0  remainder 1  ← (stop when quotient = 0)

Read remainders upward: 101010"#}
                                    </pre>
                                </div>
                                
                                <h4>{"Verification (Binary to Decimal):"}</h4>
                                <div class="example-box">
                                    <p><strong>{"Place value calculation:"}</strong></p>
                                    <pre style="color: var(--color-font); white-space: pre; font-family: inherit; margin: 0; padding-left: 40px;">
{r#"Position: 5  4  3  2  1  0
Binary:   1  0  1  0  1  0
Values:   32 16 8  4  2  1

1×32 + 0×16 + 1×8 + 0×4 + 1×2 + 0×1 = 42 ✓"#}
                                    </pre>
                                </div>
                            </div>

                            <div class="tutorial-step">
                                <h3>{"🔄 Example 2: Hexadecimal to Decimal Conversion"}</h3>
                                <p><strong>{"Goal:"}</strong> {" Convert hexadecimal 2A to decimal"}</p>
                                
                                <div class="example-box">
                                    <p><strong>{"Step-by-step breakdown:"}</strong></p>
                                    <pre style="color: var(--color-font); white-space: pre; font-family: inherit; margin: 0; padding-left: 40px;">
{r#"Hex: 2A
Position values in base 16: 16¹, 16⁰

2A = 2×16¹ + A×16⁰
   = 2×16 + 10×1    (A = 10 in decimal)
   = 32 + 10
   = 42"#}
                                    </pre>
                                </div>
                            </div>

                            <div class="tutorial-step">
                                <h3>{"🔄 Example 3: Floating-Point Conversion"}</h3>
                                <p><strong>{"Goal:"}</strong> {" Convert decimal 42.625 to binary"}</p>
                                
                                <h4>{"Integer Part (42):"}</h4>
                                <div class="example-box">
                                    <p><strong>{"Same as Example 1:"}</strong> {" 42₁₀ = 101010₂"}</p>
                                </div>

                                <h4>{"Fractional Part (0.625):"}</h4>
                                <div class="example-box">
                                    <p><strong>{"Multiplication method:"}</strong></p>
                                    <pre style="color: var(--color-font); white-space: pre; font-family: inherit; margin: 0; padding-left: 40px;">
{r#"0.625 × 2 = 1.25  → integer part: 1, continue with 0.25
0.25  × 2 = 0.5   → integer part: 0, continue with 0.5
0.5   × 2 = 1.0   → integer part: 1, continue with 0.0 (stop)

Read integer parts downward: .101"#}
                                    </pre>
                                </div>

                                <h4>{"Combined Result:"}</h4>
                                <div class="example-box">
                                    <p><strong>{"42.625₁₀ = 101010.101₂"}</strong></p>
                                    <p>{"Verification: 32+8+2 + 0.5+0.125 = 42.625 ✓"}</p>
                                </div>
                            </div>

                            <div class="tutorial-step">
                                <h3>{"🔄 Example 4: Working with Uncommon Bases"}</h3>
                                <p><strong>{"Goal:"}</strong> {" Convert decimal 100 to base 7"}</p>
                                
                                <div class="example-box">
                                    <p><strong>{"Division by 7 method:"}</strong></p>
                                    <pre style="color: var(--color-font); white-space: pre; font-family: inherit; margin: 0; padding-left: 40px;">
{r#"100 ÷ 7 = 14 remainder 2  ←
14  ÷ 7 = 2  remainder 0  ←
2   ÷ 7 = 0  remainder 2  ←

Read remainders upward: 202₇

Verification: 2×7² + 0×7¹ + 2×7⁰ = 98+0+2 = 100 ✓"#}
                                    </pre>
                                </div>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"💼 Professional Use Cases & Applications"}</h2>
                            
                            <div class="use-case">
                                <h3>{"1. Software Development & Programming"}</h3>
                                <ul>
                                    <li><strong>{"Memory Address Calculation:"}</strong> {" Convert between hex addresses and decimal offsets for debugging."}</li>
                                    <li><strong>{"Bitwise Operations:"}</strong> {" Understand AND, OR, XOR operations by visualizing binary representations."}</li>
                                    <li><strong>{"File Permissions:"}</strong> {" Convert Unix file permissions between octal (755) and binary representations."}</li>
                                    <li><strong>{"Assembly Programming:"}</strong> {" Work with hex opcodes and binary instruction formats."}</li>
                                    <li><strong>{"Network Programming:"}</strong> {" Convert IP addresses, port numbers, and protocol identifiers between different formats."}</li>
                                <li><strong>{"Data Analysis:"}</strong> {" Process numerical data in different bases for statistical analysis and visualization."}</li>
                                <li><strong>{"Embedded Systems:"}</strong> {" Convert between binary, octal, and hexadecimal for microcontroller programming."}</li>
                                <li><strong>{"Game Development:"}</strong> {" Handle game states, coordinates, and resource management using different number systems."}</li>
                                <li><strong>{"Database Operations:"}</strong> {" Convert primary keys, hash values, and encoded data between formats."}</li>
                                <li><strong>{"Mathematics & Research:"}</strong> {" Explore number theory, algorithmic efficiency, and computational mathematics."}</li>
                                </ul>
                                <div class="example-box">
                                    <p><strong>{"Real Example - File Permissions:"}</strong></p>
                                    <ul>
                                        <li>{"Unix Permissions: 755"}</li>
                                        <li>{"Owner: 7₈ = 111₂ (rwx)"}</li>
                                        <li>{"Group: 5₈ = 101₂ (r-x)"}</li>
                                        <li>{"Others: 5₈ = 101₂ (r-x)"}</li>
                                    </ul>
                                </div>
                            </div>

                            <div class="use-case">
                                <h3>{"2. Computer Science Education"}</h3>
                                <ul>
                                    <li><strong>{"Algorithm Visualization:"}</strong> {" Demonstrate how computers process different number systems."}</li>
                                    <li><strong>{"Data Structure Analysis:"}</strong> {" Understand hash table indexing and array addressing."}</li>
                                    <li><strong>{"Computer Architecture:"}</strong> {" Learn CPU instruction encoding and memory organization."}</li>
                                    <li><strong>{"Floating-Point Understanding:"}</strong> {" Explore IEEE 754 representation and precision issues."}</li>
                                    <li><strong>{"Cryptography Basics:"}</strong> {" Work with large numbers in different bases for encryption algorithms."}</li>
                                </ul>
                            </div>

                            <div class="use-case">
                                <h3>{"3. Digital Electronics & Hardware"}</h3>
                                <ul>
                                    <li><strong>{"Logic Circuit Design:"}</strong> {" Convert truth tables between binary and decimal representations."}</li>
                                    <li><strong>{"Microcontroller Programming:"}</strong> {" Configure registers using hex values and understand binary flags."}</li>
                                    <li><strong>{"Protocol Analysis:"}</strong> {" Decode communication protocols (SPI, I2C, UART) data frames."}</li>
                                    <li><strong>{"Memory Layout:"}</strong> {" Calculate memory addresses and data structure offsets."}</li>
                                    <li><strong>{"Embedded Systems:"}</strong> {" Work with ADC readings, PWM values, and sensor data."}</li>
                                </ul>
                            </div>

                            <div class="use-case">
                                <h3>{"4. Cybersecurity & Forensics"}</h3>
                                <ul>
                                    <li><strong>{"Malware Analysis:"}</strong> {" Analyze hex dumps and understand shellcode patterns."}</li>
                                    <li><strong>{"Network Security:"}</strong> {" Convert packet data between hex and ASCII for analysis."}</li>
                                    <li><strong>{"Reverse Engineering:"}</strong> {" Decode binary file formats and understand data structures."}</li>
                                    <li><strong>{"Memory Forensics:"}</strong> {" Analyze memory dumps and locate specific patterns."}</li>
                                    <li><strong>{"Cryptanalysis:"}</strong> {" Work with different number representations in encryption research."}</li>
                                </ul>
                            </div>

                            <div class="use-case">
                                <h3>{"5. Mathematics & Research"}</h3>
                                <ul>
                                    <li><strong>{"Number Theory:"}</strong> {" Explore patterns and properties in different base systems."}</li>
                                    <li><strong>{"Statistical Analysis:"}</strong> {" Convert between number systems for data representation."}</li>
                                    <li><strong>{"Scientific Computing:"}</strong> {" Handle precision and rounding in different number bases."}</li>
                                    <li><strong>{"Game Development:"}</strong> {" Optimize graphics calculations and understand color blending."}</li>
                                    <li><strong>{"Financial Technology:"}</strong> {" Process transaction IDs and implement checksums."}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🔬 Advanced Mathematical Concepts"}</h2>
                            
                            <h3>{"📐 Why Different Bases Matter"}</h3>
                            <p>{"Each base has unique advantages for specific applications:"}</p>
                            
                            <div class="example-box">
                                <p><strong>{"Base efficiency for different ranges:"}</strong></p>
                                <ul>
                                    <li><strong>{"Binary (Base 2):"}</strong> {" Perfect for digital logic (on/off states)"}</li>
                                    <li><strong>{"Octal (Base 8):"}</strong> {" Compact representation of 3-bit groups"}</li>
                                    <li><strong>{"Decimal (Base 10):"}</strong> {" Human-friendly, matches our finger counting"}</li>
                                    <li><strong>{"Hexadecimal (Base 16):"}</strong> {" Compact representation of 4-bit groups"}</li>
                                    <li><strong>{"Base 64:"}</strong> {" Efficient for encoding binary data as text"}</li>
                                </ul>
                            </div>

                            <h3>{"⚡ Floating-Point Precision Insights"}</h3>
                            <div class="example-box">
                                <p><strong>{"Understanding precision limits:"}</strong></p>
                                <pre style="color: var(--color-font); white-space: pre; font-family: inherit; margin: 0; padding-left: 40px;">
{r#"Decimal: 0.1
Binary:  0.000110011001100... (repeating)
         ↑ Cannot be exactly represented in binary!

This is why: 0.1 + 0.2 ≠ 0.3 in programming"#}
                                </pre>
                            </div>

                            <h3>{"🔄 Conversion Algorithm Complexity"}</h3>
                            <p>{"Understanding computational efficiency:"}</p>
                            <ul>
                                <li><strong>{"Time Complexity:"}</strong> {" O(log n) for converting n-digit numbers"}</li>
                                <li><strong>{"Space Complexity:"}</strong> {" O(log n) for storing the result"}</li>
                                <li><strong>{"Optimization:"}</strong> {" Powers of 2 bases can be converted via bit shifting"}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"❓ Frequently Asked Questions"}</h2>
                            <div class="faq-item">
                                <h3>{"Q: What bases are supported?"}</h3>
                                <p>{"A: You can convert between any base from 2 to 36, including decimal, binary, octal, and hexadecimal."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Can I enter negative numbers?"}</h3>
                                <p>{"A: Yes, negative numbers are supported in all bases."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Is my data safe?"}</h3>
                                <p>{"A: All conversions happen locally in your browser. No data is sent to any server."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Can I copy the results?"}</h3>
                                <p>{"A: Yes, click any output field to copy the value with a notification."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Can I convert to uncommon bases (e.g., base 7, base 13)?"}</h3>
                                <p>{"A: Yes, use the 'All Bases' section to convert to and from any base between 2 and 36."}</p>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🎯 Best Practices"}</h2>
                            <ul>
                                <li><strong>{"Validate Input:"}</strong> {"Always check your input for valid digits in the selected base."}</li>
                                <li><strong>{"Error Handling:"}</strong> {"Handle invalid input gracefully in your applications."}</li>
                                <li><strong>{"Performance:"}</strong> {"For large numbers, use efficient algorithms for conversion."}</li>
                                <li><strong>{"Documentation:"}</strong> {"Document which bases are used and why in your codebase."}</li>
                                <li><strong>{"Testing:"}</strong> {"Test with edge cases, such as negative numbers and large values."}</li>
                                <li><strong>{"Security Awareness:"}</strong> {"Never trust unvalidated input from untrusted sources."}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"🔗 Related Tools"}</h2>
                            <ul>
                                {
                                    ToolCategoryManager::get_related_tools("base")
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
                        <div style="margin-bottom: 20px; padding: 15px; border: 1px solid var(--color-third); border-radius: 5px;">
                            <div style="display: flex; align-items: center; gap: 20px; margin-bottom: 10px;">
                                <div style="display: flex; align-items: center; gap: 8px;">
                                    <label>
                                        <input 
                                            type="checkbox" 
                                            checked={self.supports_float}
                                            onchange={ctx.link().callback(|_| Msg::ToggleFloatMode)}
                                        />
                                        {"Float Mode"}
                                    </label>
                                </div>
                                if self.supports_float {
                                    <div style="display: flex; align-items: center; gap: 8px;">
                                        <label for="precision-select">{"Precision:"}</label>
                                        <select 
                                            id="precision-select"
                                            value={self.decimal_precision.to_string()}
                                            onchange={ctx.link().callback(|e: Event| {
                                                let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                                                let precision = select.value().parse::<u32>().unwrap_or(6);
                                                Msg::SetPrecision(precision)
                                            })}
                                        >
                                            <option value="3" selected={self.decimal_precision == 3}>{"3 digits"}</option>
                                            <option value="6" selected={self.decimal_precision == 6}>{"6 digits"}</option>
                                            <option value="9" selected={self.decimal_precision == 9}>{"9 digits"}</option>
                                            <option value="12" selected={self.decimal_precision == 12}>{"12 digits"}</option>
                                        </select>
                                    </div>
                                }
                            </div>
                            <div style="font-size: 12px; color: var(--color-subfont);">
                                if self.supports_float {
                                    {"Float mode supports decimal numbers (e.g., 42.625). Repeating decimals shown as 0.333(3)."}
                                } else {
                                    {"Integer mode only. Enable Float Mode to work with decimal numbers."}
                                }
                            </div>
                        </div>
                        
                        <div class="base-container" style="width: 100%; display: flex; flex-direction: column; gap: 8px;">
                                { for common_bases.iter().map(|&base| {
                                    if let Some(value) = self.bases.get(&base) {
                                        let base_name = match base {
                                            2 => "Binary",
                                            8 => "Octal",
                                            10 => "Decimal",
                                            16 => "Hexadecimal",
                                            _ => unreachable!(),
                                        };
                                        self.render_input(ctx, base_name, value, base)
                                    } else {
                                        html! {}
                                    }
                                }) }
                            </div>
                        <div style="margin-top: 20px;">
                            <label for="custom-base-select" style="margin-right: 8px;">{"Add Base:"}</label>
                            <select id="custom-base-select" onchange={on_add}>
                                <option value="" selected=true disabled=true>{"Select base"}</option>
                                { for available_bases.iter().map(|&base| {
                                    html! { <option value={base.to_string()}>{format!("Base {}", base)}</option> }
                                }) }
                            </select>
                        </div>
                        <div class="base-container" style="width: 100%; margin-top: 10px; display: flex; flex-direction: column; gap: 8px;">
                            { for self.custom_bases.iter().map(|&base| {
                                if let Some(value) = self.bases.get(&base) {
                                    let label = format!("Base {}", base);
                                    let remove_cb = ctx.link().callback(move |_| Msg::RemoveCustomBase(base));
                                    html! {
                                        <div style="display: grid; align-items: center; gap: 8px; grid-auto-flow: column;">
                                            { self.render_input(ctx, &label, value, base) }
                                            <button class="tool-btn" onclick={remove_cb} style="height: 32px;">{"Delete"}</button>
                            </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }) }
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
                    doc.set_title("Number Base Converter | CompuTools");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "Number Base Converter with floating-point support, flexible input formats (0x, 0b, 0o, \\x), and dynamic base addition (2-36). Features step-by-step conversion tutorials, mathematical process visualization, real-time validation, and professional use cases for programming, cybersecurity, and data analysis. Advanced error handling with educational content.").unwrap();
                    }
                }
            }
        }
    }
}

impl ToolBase {
    fn validate_input(&self, input: &str, base: u32) -> Result<(), String> {
        if input.trim().is_empty() {
            return Ok(());
        }

        let input = input.trim();
        
        // 음수 부호 처리
        let (is_negative, number_part) = if input.starts_with('-') {
            (true, &input[1..])
        } else {
            (false, input)
        };

        if number_part.is_empty() {
            return Err("Input value is empty.".to_string());
        }

        // 부동소수점 모드에서 소수점 처리
        if self.supports_float && number_part.contains('.') {
            return self.validate_float_input(number_part, base);
        }

        // 다양한 포맷에서 실제 숫자 부분 추출
        let cleaned_input = self.parse_flexible_format(number_part, base)?;

        // prefix만 입력된 경우 (예: "0b", "0x", "0o") - 타이핑 중으로 간주
        if cleaned_input.is_empty() {
            return Ok(());
        }

        // 각 진수별 유효한 문자 집합 정의
        let valid_chars = match base {
            2 => "01",
            3 => "012",
            4 => "0123",
            5 => "01234",
            6 => "012345",
            7 => "0123456",
            8 => "01234567",
            9 => "012345678",
            10 => "0123456789",
            11 => "0123456789A",
            12 => "0123456789AB",
            13 => "0123456789ABC",
            14 => "0123456789ABCD",
            15 => "0123456789ABCDE",
            16 => "0123456789ABCDEF",
            17..=36 => "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            _ => return Err("Unsupported base.".to_string()),
        };

        let max_digit = if base <= 16 { 
            &valid_chars[..base as usize] 
        } else { 
            &valid_chars[..base as usize] 
        };

        // 각 문자가 해당 진수에서 유효한지 검사
        for ch in cleaned_input.chars() {
            let upper_ch = ch.to_ascii_uppercase();
            if !max_digit.contains(upper_ch) {
                return Err(format!(
                    "'{}' is not a valid character for base {}. Valid characters: {}",
                    ch,
                    base,
                    max_digit
                ));
            }
        }

        // 실제 파싱 테스트
        match i64::from_str_radix(&cleaned_input, base) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Cannot convert to base {}.", base)),
        }
    }

    fn validate_float_input(&self, input: &str, base: u32) -> Result<(), String> {
        let parts: Vec<&str> = input.split('.').collect();
        if parts.len() > 2 {
            return Err("Multiple decimal points are not allowed.".to_string());
        }

        let integer_part = parts[0];
        let fractional_part = if parts.len() == 2 { parts[1] } else { "" };

        // 정수 부분 검증
        if !integer_part.is_empty() {
            let cleaned_integer = self.parse_flexible_format(integer_part, base)?;
            if !cleaned_integer.is_empty() {
                self.validate_digits(&cleaned_integer, base)?;
            }
        }

        // 소수 부분 검증 (prefix 없이)
        if !fractional_part.is_empty() {
            self.validate_digits(fractional_part, base)?;
        }

        Ok(())
    }

    fn validate_digits(&self, input: &str, base: u32) -> Result<(), String> {
        let valid_chars = match base {
            2 => "01",
            3 => "012",
            4 => "0123",
            5 => "01234",
            6 => "012345",
            7 => "0123456",
            8 => "01234567",
            9 => "012345678",
            10 => "0123456789",
            11 => "0123456789A",
            12 => "0123456789AB",
            13 => "0123456789ABC",
            14 => "0123456789ABCD",
            15 => "0123456789ABCDE",
            16 => "0123456789ABCDEF",
            17..=36 => "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            _ => return Err("Unsupported base.".to_string()),
        };

        let max_digit = if base <= 16 { 
            &valid_chars[..base as usize] 
        } else { 
            &valid_chars[..base as usize] 
        };

        for ch in input.chars() {
            let upper_ch = ch.to_ascii_uppercase();
            if !max_digit.contains(upper_ch) {
                return Err(format!(
                    "'{}' is not a valid character for base {}. Valid characters: {}",
                    ch,
                    base,
                    max_digit
                ));
            }
        }

        Ok(())
    }

    fn parse_float_input(&self, input: &str, base: u32) -> Result<f64, String> {
        let input = input.trim();
        
        // 음수 처리
        let (is_negative, number_part) = if input.starts_with('-') {
            (true, &input[1..])
        } else {
            (false, input)
        };

        if !number_part.contains('.') {
            // 정수인 경우
            let cleaned = self.parse_flexible_format(number_part, base)?;
            if cleaned.is_empty() {
                return Ok(0.0);
            }
            let int_value = i64::from_str_radix(&cleaned, base)
                .map_err(|_| format!("Cannot convert to base {}.", base))? as f64;
            return Ok(if is_negative { -int_value } else { int_value });
        }

        // 소수점 분리
        let parts: Vec<&str> = number_part.split('.').collect();
        if parts.len() != 2 {
            return Err("Invalid decimal format.".to_string());
        }

        let integer_part = parts[0];
        let fractional_part = parts[1];

        // 정수 부분 변환
        let integer_value = if integer_part.is_empty() {
            0.0
        } else {
            let cleaned = self.parse_flexible_format(integer_part, base)?;
            if cleaned.is_empty() {
                0.0
            } else {
                i64::from_str_radix(&cleaned, base)
                    .map_err(|_| format!("Cannot convert integer part to base {}.", base))? as f64
            }
        };

        // 소수 부분 변환
        let fractional_value = if fractional_part.is_empty() {
            0.0
        } else {
            self.convert_fractional_from_base(fractional_part, base)?
        };

        let result = integer_value + fractional_value;
        Ok(if is_negative { -result } else { result })
    }

    fn convert_fractional_from_base(&self, fractional: &str, base: u32) -> Result<f64, String> {
        let mut result = 0.0;
        let mut power = 1.0 / base as f64;

        for ch in fractional.chars() {
            let digit_value = self.char_to_digit(ch)?;
            if digit_value >= base {
                return Err(format!("Invalid digit '{}' for base {}.", ch, base));
            }
            result += digit_value as f64 * power;
            power /= base as f64;
        }

        Ok(result)
    }

    fn char_to_digit(&self, ch: char) -> Result<u32, String> {
        match ch.to_ascii_uppercase() {
            '0'..='9' => Ok(ch as u32 - '0' as u32),
            'A'..='Z' => Ok(ch.to_ascii_uppercase() as u32 - 'A' as u32 + 10),
            _ => Err(format!("Invalid character: {}", ch)),
        }
    }

    fn convert_float_to_base(&self, value: f64, base: u32) -> String {
        if value.is_nan() {
            return "NaN".to_string();
        }
        if value.is_infinite() {
            return if value.is_sign_positive() { "∞" } else { "-∞" }.to_string();
        }

        let is_negative = value < 0.0;
        let abs_value = value.abs();
        
        let integer_part = abs_value.trunc() as i64;
        let fractional_part = abs_value.fract();

        // 정수 부분 변환
        let integer_str = if integer_part == 0 {
            "0".to_string()
        } else {
            self.convert_integer_to_base(integer_part, base)
        };

        // 소수 부분 변환
        let fractional_str = if fractional_part == 0.0 {
            String::new()
        } else {
            self.convert_fractional_to_base(fractional_part, base)
        };

        let result = if fractional_str.is_empty() {
            integer_str
        } else {
            format!("{}.{}", integer_str, fractional_str)
        };

        if is_negative {
            format!("-{}", result)
        } else {
            result
        }
    }

    fn convert_integer_to_base(&self, mut num: i64, base: u32) -> String {
        if num == 0 {
            return "0".to_string();
        }

        let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut result = String::new();

        while num > 0 {
            let digit = (num % base as i64) as usize;
            result.insert(0, digits.chars().nth(digit).unwrap());
            num /= base as i64;
        }

        result
    }

    fn convert_fractional_to_base(&self, mut fractional: f64, base: u32) -> String {
        let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut result = String::new();
        let mut seen_values = Vec::new();

        for i in 0..self.decimal_precision {
            if fractional == 0.0 {
                break;
            }

            // 반복 소수 감지
            if seen_values.contains(&fractional) {
                // 반복 패턴 찾기
                let repeat_start = seen_values.iter().position(|&x| (x - fractional).abs() < 1e-15).unwrap_or(0);
                let repeating_part = &result[repeat_start..];
                if !repeating_part.is_empty() {
                    return format!("{}({})", &result[..repeat_start], repeating_part);
                }
                break;
            }

            seen_values.push(fractional);
            fractional *= base as f64;
            let digit = fractional.trunc() as usize;
            
            if digit < base as usize {
                result.push(digits.chars().nth(digit).unwrap());
            }
            
            fractional = fractional.fract();
        }

        // 정밀도 한계에 도달한 경우 끝에 ... 추가
        if fractional != 0.0 && result.len() == self.decimal_precision as usize {
            result.push_str("...");
        }

        result
    }

    fn parse_flexible_format(&self, input: &str, base: u32) -> Result<String, String> {
        let input = input.trim();
        
        if input.is_empty() {
            return Ok(input.to_string());
        }
        
        match base {
            2 => {
                // Binary: 0b101010, b101010, 101010
                if input.starts_with("0b") || input.starts_with("0B") {
                    let remaining = &input[2..];
                    Ok(remaining.to_string())
                } else if input.starts_with("b") || input.starts_with("B") {
                    let remaining = &input[1..];
                    Ok(remaining.to_string())
                } else {
                    Ok(input.to_string())
                }
            },
            8 => {
                // Octal: 0o52, o52, 052, 52, \052
                if input.starts_with("0o") || input.starts_with("0O") {
                    let remaining = &input[2..];
                    Ok(remaining.to_string())
                } else if input.starts_with("o") || input.starts_with("O") {
                    let remaining = &input[1..];
                    Ok(remaining.to_string())
                } else if input.starts_with("\\") && input.len() > 1 {
                    // Octal escape sequence: \052
                    let remaining = &input[1..];
                    Ok(remaining.to_string())
                } else if input.starts_with("0") && input.len() > 1 && 
                         !input.starts_with("0x") && !input.starts_with("0X") && 
                         !input.starts_with("0b") && !input.starts_with("0B") && 
                         !input.starts_with("0o") && !input.starts_with("0O") {
                    // Leading zero for octal (traditional C style)
                    // But exclude other prefixes like 0x, 0b, 0o
                    Ok(input[1..].to_string())
                } else {
                    Ok(input.to_string())
                }
            },
            16 => {
                // Hexadecimal: 0x2A, x2A, 2A, \x2A
                if input.starts_with("0x") || input.starts_with("0X") {
                    let remaining = &input[2..];
                    Ok(remaining.to_string())
                } else if input.starts_with("x") || input.starts_with("X") {
                    let remaining = &input[1..];
                    Ok(remaining.to_string())
                } else if input.starts_with("\\x") || input.starts_with("\\X") {
                    let remaining = &input[2..];
                    Ok(remaining.to_string())
                } else {
                    Ok(input.to_string())
                }
            },
            _ => {
                // Other bases: just return as-is
                Ok(input.to_string())
            }
        }
    }

    fn update_all(&mut self, num: i64) {
        self.update_all_except(num, None);
    }

    fn update_all_except(&mut self, num: i64, except_base: Option<u32>) {
        let sign = if num < 0 { "-" } else { "" };
        
        // 모든 진수에 대해 변환 진행 (except_base 제외)
        for base in 2..=36 {
            if let Some(except) = except_base {
                if base == except {
                    continue; // 현재 입력 중인 필드는 건드리지 않음
                }
            }
            
            if let Some(value) = self.bases.get_mut(&base) {
                // 진수별 서식 지정
                match base {
                    10 => *value = format!("{}", num), // 10진수는 그대로 표현
                    _ => {
                        // 진수 변환을 위한 문자 집합 (0-9, A-Z)
                        let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
                        let mut result = String::new();
                        let mut n = num.abs();
                        
                        // 숫자가 0인 경우 처리
                        if n == 0 {
                            *value = "0".to_string();
                            continue;
                        }
                        
                        // 진수 변환 알고리즘
                        while n > 0 {
                            let digit = (n % base as i64) as usize;
                            result.insert(0, digits.chars().nth(digit).unwrap());
                            n /= base as i64;
                        }
                        
                        // 음수인 경우 부호 추가
                        *value = format!("{}{}", sign, result);
                    }
                }
            }
        }
    }

    fn update_all_float_except(&mut self, float_value: f64, except_base: Option<u32>) {
        // 모든 진수에 대해 변환 진행 (except_base 제외)
        for base in 2..=36 {
            if let Some(except) = except_base {
                if base == except {
                    continue; // 현재 입력 중인 필드는 건드리지 않음
                }
            }
            
            // convert_float_to_base 호출을 먼저 수행하여 borrowing 충돌 방지
            let converted_value = self.convert_float_to_base(float_value, base);
            
            if let Some(field_value) = self.bases.get_mut(&base) {
                *field_value = converted_value;
            }
        }
    }

    fn render_input(&self, ctx: &Context<Self>, label: &str, value: &str, base: u32) -> Html {
        let base_clone = base;
        let link = ctx.link().callback(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateBase(base_clone, input.value())
        });

        let has_error = self.error_messages.get(&base).unwrap_or(&None).is_some();
        let input_style = if has_error {
            "width: 100%; padding: 5px; border: 2px solid var(--color-error);"
        } else {
            "width: 100%; padding: 5px;"
        };
    
        html! {
            <div class="tool-inner" style="padding: 10px;">
                <div class="tool-subtitle">{ label }</div>
                <input 
                    type="text" 
                    value={format!("{}", value.clone())} 
                    oninput={link}
                    style={input_style}
                    placeholder={self.get_placeholder_for_base(base)}
                />
                if let Some(error_msg) = self.error_messages.get(&base).unwrap_or(&None) {
                    <div style="color: var(--color-error); font-size: 12px; margin-top: 4px; line-height: 1.3;">
                        { error_msg }
                    </div>
                }
                <div style="color: var(--color-subfont); font-size: 11px; margin-top: 2px;">
                    { self.get_help_text_for_base(base) }
                </div>
            </div>
        }
    }

    fn get_placeholder_for_base(&self, base: u32) -> String {
        if self.supports_float {
            match base {
                2 => "e.g. 0b101010.101, 101010.101".to_string(),
                8 => "e.g. 0o52.4, \\052.4".to_string(),
                10 => "e.g. 42.625, -42.625".to_string(),
                16 => "e.g. 0x2A.A, 2A.A".to_string(),
                _ => format!("Enter base {} number (float)", base),
            }
        } else {
            match base {
                2 => "e.g. 0b101010, b101010, 101010".to_string(),
                8 => "e.g. 0o52, o52, 052, \\052".to_string(),
                10 => "e.g. 42, -42".to_string(),
                16 => "e.g. 0x2A, x2A, 2A \\x2A".to_string(),
                _ => format!("Enter base {} number", base),
            }
        }
    }

    fn get_help_text_for_base(&self, base: u32) -> String {
        if self.supports_float {
            match base {
                2 => "Valid: 0-1 | Supports decimal point".to_string(),
                8 => "Valid: 0-7 | Supports decimal point".to_string(),
                10 => "Valid: 0-9 | Supports decimal point and negative".to_string(),
                16 => "Valid: 0-9, A-F | Supports decimal point".to_string(),
                _ => {
                    if base <= 10 {
                        format!("Valid: 0-{} | Supports decimal point", base - 1)
                    } else {
                        let last_letter = (b'A' + (base - 11) as u8) as char;
                        format!("Valid: 0-9, A-{} | Supports decimal point", last_letter)
                    }
                }
            }
        } else {
            match base {
                2 => "Valid: 0-1 | Formats: 0b, b, or plain".to_string(),
                8 => "Valid: 0-7 | Formats: 0o, o, 0prefix, \\, or plain".to_string(),
                10 => "Valid: 0-9 | Supports negative numbers".to_string(),
                16 => "Valid: 0-9, A-F | Formats: 0x, x, \\x, or plain".to_string(),
                _ => {
                    if base <= 10 {
                        format!("Valid: 0-{}", base - 1)
                    } else {
                        let last_letter = (b'A' + (base - 11) as u8) as char;
                        format!("Valid: 0-9, A-{}", last_letter)
                    }
                }
            }
        }
    }
}