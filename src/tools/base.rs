use yew::prelude::*;
use log::info;
use std::collections::BTreeMap;
use web_sys::window;

pub struct ToolBase {
    bases: BTreeMap<u32, String>,
    custom_bases: Vec<u32>, // 동적으로 추가된 진수 목록
}

pub enum Msg {
    UpdateBase(u32, String),
    AddCustomBase(u32),
    RemoveCustomBase(u32),
}

impl Component for ToolBase {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut bases = BTreeMap::new();
        for base in 2..=36 {
            bases.insert(base, String::new());
        }
        Self { bases, custom_bases: vec![] }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateBase(base, value) => {
                if value.is_empty() {
                    for (_, val) in self.bases.iter_mut() {
                        val.clear();
                    }
                } else if let Ok(num) = i64::from_str_radix(&value, base) {
                    self.update_all(num);
                }
            }
            Msg::AddCustomBase(base) => {
                if !self.custom_bases.contains(&base) {
                    self.custom_bases.push(base);
                }
            }
            Msg::RemoveCustomBase(base) => {
                self.custom_bases.retain(|&b| b != base);
            }
        }
        true
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
                <h1 class="tool-title">{ "Base Converter" }</h1>
                <div class="tool-wrapper">
                        <div class="tool-intro">
                        <div class="content-section">
                            <h2>{"🔢 What is Base Conversion?"}</h2>
                            <p>{"Base conversion is the process of translating numbers from one numeral system (base) to another. Common bases include decimal (10), binary (2), octal (8), and hexadecimal (16). Each base uses a different set of digits and place values."}</p>
                            <p>{"Base conversion is fundamental in computer science, mathematics, and engineering, enabling seamless data representation and manipulation across systems."}</p>
                        </div>

                        <div class="content-section">
                            <h2>{"⚙️ How This Base Converter Works"}</h2>
                            <p>{"This tool instantly converts numbers between decimal, binary, octal, hexadecimal, and any base from 2 to 36. Enter a value in any field, and all other bases update in real time."}</p>
                            <h3>{"Supported Features:"}</h3>
                            <ul>
                                <li><strong>{"Multi-Base Conversion:"}</strong> {"Convert between decimal, binary, octal, hexadecimal, and all bases 2–36."}</li>
                                <li><strong>{"Negative Numbers:"}</strong> {"Supports negative values in all bases."}</li>
                                <li><strong>{"Real-time Update:"}</strong> {"All fields update instantly as you type."}</li>
                                <li><strong>{"Copy with Notification:"}</strong> {"Click any output field to copy results with visual feedback."}</li>
                                <li><strong>{"Local Processing:"}</strong> {"All conversions happen in your browser for privacy and speed."}</li>
                            </ul>
                            <h3>{"Input Format Example:"}</h3>
                            <div class="example-box">
                                <p><strong>{"Decimal input:"}</strong></p>
                                <ul>
                                    <li>{"42"}</li>
                                </ul>
                                <p><strong>{"Binary input:"}</strong></p>
                                <ul>
                                    <li>{"101010"}</li>
                                </ul>
                                <p><strong>{"Hexadecimal input:"}</strong></p>
                                <ul>
                                    <li>{"2A"}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"💡 Common Use Cases"}</h2>
                            <div class="use-case">
                                <h3>{"1. Programming & Development"}</h3>
                                <ul>
                                    <li><strong>{"Bitwise Operations:"}</strong> {"Convert between binary and hex for low-level programming."}</li>
                                    <li><strong>{"Debugging:"}</strong> {"Interpret memory addresses and data in different bases."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"2. Education & Learning"}</h3>
                                <ul>
                                    <li><strong>{"Teaching Number Systems:"}</strong> {"Help students understand base conversion concepts."}</li>
                                    <li><strong>{"Math Exercises:"}</strong> {"Practice converting numbers between bases."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"3. Data Analysis & Engineering"}</h3>
                                <ul>
                                    <li><strong>{"Data Encoding:"}</strong> {"Convert data for communication protocols and file formats."}</li>
                                    <li><strong>{"System Integration:"}</strong> {"Work with legacy systems using non-decimal bases."}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"📚 Step-by-Step Tutorial"}</h2>
                            <div class="tutorial-step">
                                <h3>{"Example: Converting Decimal to Binary, Octal, and Hexadecimal"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Convert the decimal number 42 to binary, octal, and hexadecimal."}</p>
                                <ol>
                                    <li>{"Enter '42' in the Decimal field."}</li>
                                    <li>{"View the converted values in the Binary, Octal, and Hexadecimal fields instantly."}</li>
                                    <li>{"Click any field to copy the value."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Input (Decimal):"}</strong> {"42"}</p>
                                    <p><strong>{"Output:"}</strong></p>
                                    <pre style="color: var(--color-font); white-space: pre; font-family: inherit; margin: 0; padding-left: 40px;">
{r#"Binary:      101010
Octal:       52
Hexadecimal: 2A"#}
                                    </pre>
                                </div>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🔧 Technical Background"}</h2>
                            <h3>{"How Base Conversion Works"}</h3>
                            <p>{"Base conversion involves dividing or multiplying by the base and mapping digits to their respective symbols. For example, decimal 42 is binary 101010, octal 52, and hexadecimal 2A."}</p>
                            <div class="example-box">
                                <p><strong>{"Example for Base 16:"}</strong></p>
                                <ul>
                                    <li>{"Input: 255 (Decimal)"}</li>
                                    <li>{"Output: FF (Hexadecimal)"}</li>
                                </ul>
                            </div>
                            <h3>{"Why Use Base Conversion?"}</h3>
                            <ul>
                                <li>{"Facilitates communication between systems using different numeral systems."}</li>
                                <li>{"Essential for low-level programming, networking, and data encoding."}</li>
                                <li>{"Helps visualize and debug binary data."}</li>
                            </ul>
                            <h3>{"Performance & Implementation"}</h3>
                            <ul>
                                <li><strong>{"Instant Conversion:"}</strong> {"All calculations happen in your browser as you type."}</li>
                                <li><strong>{"No Server Required:"}</strong> {"Local processing ensures privacy and speed."}</li>
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
                            <p>{"Enhance your workflow with this mathematical tool:"}</p>
                            <ul>
                                <li><a href="/quaternion/">{"Quaternion Calculator"}</a> {" - For advanced mathematical operations with quaternions."}</li>
                            </ul>
                        </div>
                    </div>
                    <div class="tool-container">
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
                    doc.set_title("Base Converter | CompuTools");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "This tool provides convenient number base conversion across multiple numeral systems. Number bases are different ways to represent numerical values using various digit sets. Convert numbers between common bases like decimal (10), binary (2), octal (8), and hexadecimal (16) instantly. Access conversions for all bases from 2 to 36, supporting both standard and specialized numeral systems.").unwrap();
                    }
                }
            }
        }
    }
}

impl ToolBase {
    fn update_all(&mut self, num: i64) {
        let sign = if num < 0 { "-" } else { "" };
        
        // 모든 진수에 대해 변환 진행
        for base in 2..=36 {
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

    fn render_input(&self, ctx: &Context<Self>, label: &str, value: &str, base: u32) -> Html {
        let base_clone = base;
        let link = ctx.link().callback(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateBase(base_clone, input.value())
        });
    
        html! {
            <div class="tool-inner" style="padding: 10px;">
                <div class="tool-subtitle">{ label }</div>
                <input 
                    type="text" 
                    value={format!("{}", value.clone())} 
                    oninput={link}
                    style="width: 100%; padding: 5px;" 
                />
            </div>
        }
    }
}