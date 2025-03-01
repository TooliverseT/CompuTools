use serde::ser::Serialize;
use serde_json::ser::{PrettyFormatter, Serializer};
use std::io::Cursor;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;

pub struct ToolJson {
    input: String,
    output: String,
    error: Option<String>,
    tab_style: String,
    compact: bool,
}

pub enum Msg {
    UpdateInput(String),
    // FormatJson,
    CopyToClipboard(String),
    UpdateTabSize(String),
}

impl Component for ToolJson {
    type Message = Msg;
    type Properties = (); // No props needed

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            error: None,
            tab_style: "4space".to_string(),
            compact: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateInput(new_input) => {
                self.input = new_input;
                self.error = None;

                match serde_json::from_str::<serde_json::Value>(&self.input) {
                    Ok(json_value) => {
                        let mut output = Vec::new();
                        let indent = match self.tab_style.as_str() {
                            "2space" => vec![b' '; 2],
                            "3space" => vec![b' '; 3],
                            "4space" => vec![b' '; 4],
                            "compact" => vec![],
                            "1tab" => vec![b'\t'],
                            _ => vec![b' '; 4],
                        };
                        if indent.is_empty() {
                            self.compact = true;
                            let mut serializer = Serializer::with_formatter(
                                Cursor::new(&mut output),
                                serde_json::ser::CompactFormatter,
                            );
                            json_value.serialize(&mut serializer).unwrap();
                        } else {
                            self.compact = false;
                            let formatter = PrettyFormatter::with_indent(&indent);
                            let mut serializer =
                                Serializer::with_formatter(Cursor::new(&mut output), formatter);
                            json_value.serialize(&mut serializer).unwrap();
                        }
                        self.output = String::from_utf8(output).unwrap();
                        self.error = None;
                    }
                    Err(err) => {
                        self.output.clear();
                        self.error = Some(self.format_error_message(&self.input, err));
                    }
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
                false // 리렌더링 필요 없음
            }
            Msg::UpdateTabSize(size) => {
                self.tab_style = size;

                match serde_json::from_str::<serde_json::Value>(&self.input) {
                    Ok(json_value) => {
                        let mut output = Vec::new();
                        let indent = match self.tab_style.as_str() {
                            "2space" => vec![b' '; 2],
                            "3space" => vec![b' '; 3],
                            "4space" => vec![b' '; 4],
                            "compact" => vec![],
                            "1tab" => vec![b'\t'],
                            _ => vec![b' '; 4],
                        };
                        if indent.is_empty() {
                            self.compact = true;
                            let mut serializer = Serializer::with_formatter(
                                Cursor::new(&mut output),
                                serde_json::ser::CompactFormatter,
                            );
                            json_value.serialize(&mut serializer).unwrap();
                        } else {
                            self.compact = false;
                            let formatter = PrettyFormatter::with_indent(&indent);
                            let mut serializer =
                                Serializer::with_formatter(Cursor::new(&mut output), formatter);
                            json_value.serialize(&mut serializer).unwrap();
                        }
                        self.output = String::from_utf8(output).unwrap();
                        self.error = None;
                    }
                    Err(err) => {
                        self.output.clear();
                        self.error = Some(self.format_error_message(&self.input, err));
                    }
                }

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="tool-wrapper ver2">
                    <div>
                        <div class="tool-title">
                            { "JSON Formatter" }
                        </div>
                        <div class="tool-intro">
                            <p>
                                { "This tool helps you format and validate JSON data easily. JSON (JavaScript Object Notation) is a lightweight data format commonly used for data exchange between systems." }
                            </p>
                            <p>{ "With this tool, you can:" }</p>
                            <ul>
                                <li>{ "Format unstructured JSON into a human-readable, pretty-printed format." }</li>
                                <li>{ "Validate JSON input and detect syntax errors with detailed error messages." }</li>
                                <li>{ "Customize the indentation style to suit your preference, choosing between 2 spaces, 3 spaces, 4 spaces, no indent, or tab-based indentation." }</li>
                            </ul>
                            <p>
                                { "This tool is especially useful for developers working with APIs, configuration files, or any JSON-based data structure." }
                            </p>
                            <p>{ "Note:" }</p>
                            <ul>
                                <li>{ "Errors are highlighted with specific line and column details for quick debugging." }</li>
                                <li>{ "Formatted JSON maintains key ordering and consistent indentation for clarity." }</li>
                            </ul>
                            <p>
                                { "Simplify your JSON workflow with this easy-to-use formatter and validator." }
                            </p>

                        </div>
                    </div>
                    <div class="tool-container ver2">
                        <div>
                            <div style="display: flex; align-items: center; padding-left: 20px; padding-right: 20px; margin-bottom: 10px; margin-top: 5px; padding-top: 5px; padding-bottom: 5px;">
                                <div class="tool-subtitle">{ "JSON" }</div>
                            </div>
                            <div class="tool-inner">
                                <div>
                                    <textarea
                                        type="text"
                                        style="overflow: auto; height: 800px;"
                                        wrap="off"
                                        value={self.input.clone()}
                                        placeholder={"Enter JSON here"}
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
                                <div class="tool-subtitle" style="width: 60%; margin-bottom: 0px;">{ "Formatted JSON" }</div>
                                <select
                                    id="input-mode-select"
                                    style="width: 40%;"
                                    onchange={_ctx.link().callback(|e: Event| {
                                        let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                        Msg::UpdateTabSize(value)
                                    })}>
                                    <option value="2space" selected={self.tab_style == "2space"}>{ "2 Spaces" }</option>
                                    <option value="3space" selected={self.tab_style == "3space"}>{ "3 Spaces" }</option>
                                    <option value="4space" selected={self.tab_style == "4space"}>{ "4 Spaces" }</option>
                                    <option value="compact" selected={self.tab_style == "compact"}>{ "No Indent" }</option>
                                    <option value="1tab" selected={self.tab_style == "1tab"}>{ "1 Tab" }</option>
                                </select>
                            </div>
                            <div class="tool-inner">
                                <div>
                                    <textarea
                                        type="text"
                                        readonly=true
                                        wrap="off"
                                        style={if self.compact { "cursor: pointer; overflow-y: auto; overflow-x: hidden; height: 800px; white-space: pre-wrap; word-wrap: break-word;" } else {"cursor: pointer; overflow: auto; height: 800px;"}}
                                        value={self.view_output()}
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
                    doc.set_title("JSON Formatter | CompuTools");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "This tool helps you format and validate JSON data easily. JSON (JavaScript Object Notation) is a lightweight data format commonly used for data exchange between systems.").unwrap();
                    }
                }
            }
        }
    }
}

impl ToolJson {
    fn format_error_message(&self, input: &str, err: serde_json::Error) -> String {
        let line = err.line();
        let column = err.column();
        let lines: Vec<&str> = input.lines().collect();

        if line > 0 && line <= lines.len() {
            let error_line = lines[line - 1];
            let marker = format!("{}^", "-".repeat(column.saturating_sub(1)));
            return format!("Invalid JSON:\n{}\n{}\nError: {}", error_line, marker, err);
        }
        format!("Invalid JSON: {}", err)
    }

    fn view_output(&self) -> String {
        if let Some(error) = &self.error {
            // html! { <pre style="color: red; white-space: pre-wrap;">{ error }</pre> }
            format!("{}", error)
        } else {
            // html! { <pre>{ &self.output }</pre> }
            format!("{}", self.output)
        }
    }
}
