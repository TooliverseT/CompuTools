use yew::prelude::*;
use log::info;
use std::collections::BTreeMap;
use web_sys::window;

pub struct ToolBase {
    bases: BTreeMap<u32, String>,
}

pub enum Msg {
    UpdateBase(u32, String),
}

impl Component for ToolBase {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut bases = BTreeMap::new();
        // 2진수부터 36진수까지 초기화
        for base in 2..=36 {
            bases.insert(base, String::new());
        }
        
        Self { bases }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateBase(base, value) => {
                if value.is_empty() {
                    // 모든 입력 필드 초기화
                    for (_, val) in self.bases.iter_mut() {
                        val.clear();
                    }
                } else if let Ok(num) = i64::from_str_radix(&value, base) {
                    // 모든 진법으로 변환하여 업데이트
                    self.update_all(num);
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // 가장 많이 사용되는 진수들 (10, 2, 8, 16)
        let common_bases = [10, 2, 8, 16];
        
        html! {
            <>
                <div class="tool-wrapper ver2">
                    <div>
                        <div class="tool-title">
                            { "Base Converter" }
                        </div>
                        <div class="tool-intro">
                            <p>
                                { "This tool provides convenient number base conversion across multiple numeral systems. Number bases are different ways to represent numerical values using various digit sets." }
                            </p>
                            <p>{ "With this base converter, you can:" }</p>
                            <ul>
                                <li>{ "Convert numbers between common bases like decimal (10), binary (2), octal (8), and hexadecimal (16) instantly." }</li>
                                <li>{ "Access conversions for all bases from 2 to 36, supporting both standard and specialized numeral systems." }</li>
                                <li>{ "See all conversions simultaneously, allowing you to compare representations across different bases." }</li>
                            </ul>
                            <p>
                                { "This tool is especially valuable for programmers, computer science students, mathematics enthusiasts, and anyone working with different number representations." }
                            </p>
                            <p>{ "Features:" }</p>
                            <ul>
                                <li>{ "Real-time conversion that updates all fields as you type in any base input." }</li>
                                <li>{ "Support for negative numbers across all base systems." }</li>
                                <li>{ "Clear organization with frequently used bases displayed prominently for quick access." }</li>
                            </ul>
                            <p>
                                { "Simplify your numerical conversions with this comprehensive base conversion tool." }
                            </p>
                        </div>
                        <div class="tool-container ver3">
                            <div class="base-container" style="width: 100%;">
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
                        </div>
                        <div class="tool-container ver3" style="margin-top: 10px;">
                            <div class="base-container" style="width: 100%;">
                                { for self.bases.iter().map(|(&base, value)| {
                                    self.render_input(ctx, &format!("Base {}", base), value, base)
                                }) }
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