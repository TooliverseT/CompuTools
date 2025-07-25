use serde::ser::Serialize;
use serde_json::ser::{PrettyFormatter, Serializer};
use std::io::Cursor;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;
use crate::components::tool_category::ToolCategoryManager;

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
                        <h1 class="tool-title">
                            { "JSON Formatter" }
                        </h1>
                <div class="tool-wrapper">
                        <div class="tool-intro">
                        <div class="content-section">
                            <h2>{"🔤 What is JSON?"}</h2>
                            <p>{"JSON (JavaScript Object Notation) is a lightweight, text-based data format used for data interchange between systems. It is easy for humans to read and write, and easy for machines to parse and generate."}</p>
                            <p>{"JSON is widely used in web APIs, configuration files, and data storage due to its simplicity and compatibility with most programming languages."}</p>
                        </div>

                        <div class="content-section">
                            <h2>{"⚙️ How This JSON Formatter Works"}</h2>
                            <p>{"This tool formats and validates JSON data, making it easier to read, debug, and share. It also highlights syntax errors and allows you to customize the indentation style for your needs."}</p>
                            <h3>{"Supported Features:"}</h3>
                            <ul>
                                <li><strong>{"Pretty Printing:"}</strong> {"Format unstructured JSON into a human-readable, indented format."}</li>
                                <li><strong>{"Validation:"}</strong> {"Detect syntax errors and display detailed error messages with line and column numbers."}</li>
                                <li><strong>{"Indentation Options:"}</strong> {"Choose between 2, 3, 4 spaces, tab, or compact (no indent)."}</li>
                                <li><strong>{"Copy with Notification:"}</strong> {"Click any output field to copy results with visual feedback."}</li>
                                <li><strong>{"Local Processing:"}</strong> {"All formatting and validation happens in your browser for privacy and speed."}</li>
                            </ul>
                            <h3>{"Input Format Example:"}</h3>
                            <div class="example-box">
                                <p><strong>{"Unformatted JSON input:"}</strong></p>
                                <ul>
                                    <li>{"{\"name\":\"Alice\",\"age\":30,\"skills\":[\"Rust\",\"Yew\"]}"}</li>
                                </ul>
                                <p><strong>{"Formatted output (4 spaces):"}</strong></p>
                                <pre style="color: var(--color-font); white-space: pre; font-family: inherit; margin: 0; padding-left: 40px;">
{r#"{
    "name": "Alice",
    "age": 30,
    "skills": [
        "Rust",
        "Yew"
    ]
}"#}
                                </pre>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"💡 Common Use Cases"}</h2>
                            <div class="use-case">
                                <h3>{"1. API Development & Debugging"}</h3>
                                <ul>
                                    <li><strong>{"Request/Response Inspection:"}</strong> {"Format and validate JSON payloads when working with REST or GraphQL APIs."}</li>
                                    <li><strong>{"Error Diagnosis:"}</strong> {"Quickly spot syntax errors and fix malformed JSON."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"2. Configuration & Data Files"}</h3>
                                <ul>
                                    <li><strong>{"Config Editing:"}</strong> {"Edit and validate JSON-based configuration files for applications and services."}</li>
                                    <li><strong>{"Data Migration:"}</strong> {"Format and check data before importing/exporting between systems."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"3. Education & Learning"}</h3>
                                <ul>
                                    <li><strong>{"Teaching JSON Syntax:"}</strong> {"Help students and new developers understand JSON structure and errors."}</li>
                                    <li><strong>{"Code Review:"}</strong> {"Share readable JSON snippets in documentation or code reviews."}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"📚 Step-by-Step Tutorial"}</h2>
                            <div class="tutorial-step">
                                <h3>{"Example: Formatting and Validating JSON"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Format and validate a JSON string with custom indentation."}</p>
                                <ol>
                                    <li>{"Paste or type your JSON string into the input field."}</li>
                                    <li>{"Select your preferred indentation style (e.g., 4 spaces, tab, compact)."}</li>
                                    <li>{"View the formatted JSON or error message in the output field."}</li>
                                    <li>{"Click the output to copy the result for use elsewhere."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Input:"}</strong></p> 
                                    <ul>
                                        <li>{"{\"name\":\"Alice\",\"age\":30,\"skills\":[\"Rust\",\"Yew\"]}"}</li>
                                    </ul>
                                    <p><strong>{"Output (4 spaces):"}</strong></p>
                                    <pre style="color: var(--color-font); white-space: pre; font-family: inherit; margin: 0; padding-left: 40px;">
{r#"{
    "name": "Alice",
    "age": 30,
    "skills": [
        "Rust",
        "Yew"
    ]
}"#}
                                    </pre>
                                </div>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🔧 Technical Background"}</h2>
                            <h3>{"How JSON Formatting Works"}</h3>
                            <p>{"The formatter parses the input string as JSON, then serializes it back with the chosen indentation. If the input is invalid, a detailed error message is shown with the line and column of the issue."}</p>
                            <div class="example-box">
                                <p><strong>{"Example for Error Highlighting:"}</strong></p>
                                <ul>
                                    <li>{"Input: {\"name\":\"Alice\",\"age\":,\"skills\":[\"Rust\",\"Yew\"]}"}</li>
                                    <li>{"Error: Invalid JSON:\n{\"name\":\"Alice\",\"age\":,\"skills\":[\"Rust\",\"Yew\"]}\n-----------------^\nError: expected value at line 1 column 23"}</li>
                                </ul>
                            </div>
                            <h3>{"Why Use a JSON Formatter?"}</h3>
                            <ul>
                                <li>{"Makes JSON easier to read and debug."}</li>
                                <li>{"Helps catch syntax errors before deploying or sharing data."}</li>
                                <li>{"Improves collaboration by providing consistent formatting."}</li>
                            </ul>
                            <h3>{"Performance & Implementation"}</h3>
                            <ul>
                                <li><strong>{"Instant Feedback:"}</strong> {"Formatting and validation happen in your browser as you type."}</li>
                                <li><strong>{"No Server Required:"}</strong> {"All processing is local for privacy and speed."}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"❓ Frequently Asked Questions"}</h2>
                            <div class="faq-item">
                                <h3>{"Q: What happens if my JSON is invalid?"}</h3>
                                <p>{"A: The tool will display a detailed error message with the line and column of the issue, and highlight the error in the input."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Can I use this tool offline?"}</h3>
                                <p>{"A: Yes, all formatting and validation are performed locally in your browser."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Is my data safe?"}</h3>
                                <p>{"A: Yes, your JSON data never leaves your device."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Can I format very large JSON files?"}</h3>
                                <p>{"A: Yes, but performance may vary depending on your device and browser."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Why are there different indentation options?"}</h3>
                                <p>{"A: Different projects and teams have different style preferences. Choose the one that fits your needs."}</p>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🎯 Best Practices"}</h2>
                            <ul>
                                <li><strong>{"Validate Before Sharing:"}</strong> {"Always check your JSON for errors before using it in production or sharing with others."}</li>
                                <li><strong>{"Error Handling:"}</strong> {"Handle invalid JSON gracefully in your applications."}</li>
                                <li><strong>{"Performance:"}</strong> {"For very large files, use efficient parsing and formatting libraries."}</li>
                                <li><strong>{"Documentation:"}</strong> {"Document your JSON structure and formatting conventions."}</li>
                                <li><strong>{"Testing:"}</strong> {"Test with a variety of JSON structures, including edge cases and deeply nested data."}</li>
                                <li><strong>{"Security Awareness:"}</strong> {"Never trust unvalidated JSON from untrusted sources."}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"🔗 Related Tools"}</h2>
                            <ul>
                                {
                                    ToolCategoryManager::get_related_tools("json")
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
                        <div>
                            <div style="display: flex; align-items: center; padding-left: 20px; padding-right: 20px; margin-bottom: 10px; margin-top: 5px; padding-top: 5px; padding-bottom: 5px;">
                                <div class="tool-subtitle" style="width: 60%; margin-bottom: 0px;">{ "Input" }</div>
                            </div>
                            <div class="tool-inner">
                                <div>
                                    <textarea
                                        type="text"
                                        style="overflow-y: auto; overflow-x: hidden; height: 250px; white-space: pre-wrap; word-wrap: break-word;"
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
                            <div style="display: flex; align-items: center; padding-left: 20px; padding-right: 20px; margin-bottom: 10px; margin-top: 20px;">
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
                                        style={if self.compact { "cursor: pointer; overflow-y: auto; overflow-x: hidden; height: 250px; white-space: pre-wrap; word-wrap: break-word;" } else {"cursor: pointer; overflow: auto; height: 250px;"}}
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
                        meta_tag.set_attribute("content", "This tool helps you format and validate JSON data easily. JSON (JavaScript Object Notation) is a lightweight data format commonly used for data exchange between systems. Simplify your JSON workflow with this easy-to-use formatter and validator.").unwrap();
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
