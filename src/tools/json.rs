use serde::ser::Serialize;
use serde_json::ser::{PrettyFormatter, Serializer};
use std::io::Cursor;
use std::collections::HashSet;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlInputElement, Storage};
use yew::prelude::*;
use crate::components::tool_category::ToolCategoryManager;

#[derive(Clone, PartialEq)]
pub enum JsonViewMode {
    Formatted,
    TreeView,
    Yaml,
    Table,
}

#[derive(Clone, PartialEq)]
pub struct TableColumn {
    pub name: String,
    pub data_type: String,
}

#[derive(Clone, PartialEq)]
pub struct TableRow {
    pub values: Vec<String>,
}

#[derive(Clone, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
    None,
}

#[derive(Clone, PartialEq)]
pub struct TableState {
    pub sort_column: Option<String>,
    pub sort_direction: SortDirection,
    pub current_page: usize,
    pub rows_per_page: usize,
    pub search_query: String,
}

pub struct ToolJson {
    input: String,
    output: String,
    error: Option<String>,
    tab_style: String,
    compact: bool,
    show_tree_view: bool,
    expanded_nodes: HashSet<String>,
    view_mode: JsonViewMode,
    table_state: TableState,
}

pub enum Msg {
    UpdateInput(String),
    // FormatJson,
    CopyToClipboard(String),
    UpdateTabSize(String),
    UpdateViewMode(String),
    ToggleNode(String),
    SortColumn(String),
    ChangePage(usize),
    ChangeRowsPerPage(usize),
    UpdateSearchQuery(String),
}

impl Component for ToolJson {
    type Message = Msg;
    type Properties = (); // No props needed

    fn create(_ctx: &Context<Self>) -> Self {
        Self::load_from_storage()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateInput(new_input) => {
                self.input = new_input;
                self.error = None;

                match serde_json::from_str::<serde_json::Value>(&self.input) {
                    Ok(json_value) => {
                        let indent = match self.tab_style.as_str() {
                            "2space" => vec![b' '; 2],
                            "3space" => vec![b' '; 3],
                            "4space" => vec![b' '; 4],
                            "compact" => vec![],
                            "1tab" => vec![b'\t'],
                            _ => vec![b' '; 4],
                        };
                        let is_compact = indent.is_empty();
                        self.output = self.format_json_with_order_preservation(&json_value, &indent);
                        self.compact = is_compact;
                        self.error = None;
                    }
                    Err(err) => {
                        self.output.clear();
                        self.error = Some(self.format_error_message(&self.input, err));
                    }
                }

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
            Msg::UpdateTabSize(size) => {
                self.tab_style = size;

                match serde_json::from_str::<serde_json::Value>(&self.input) {
                    Ok(json_value) => {
                        let indent = match self.tab_style.as_str() {
                            "2space" => vec![b' '; 2],
                            "3space" => vec![b' '; 3],
                            "4space" => vec![b' '; 4],
                            "compact" => vec![],
                            "1tab" => vec![b'\t'],
                            _ => vec![b' '; 4],
                        };
                        let is_compact = indent.is_empty();
                        self.output = self.format_json_with_order_preservation(&json_value, &indent);
                        self.compact = is_compact;
                        self.error = None;
                    }
                    Err(err) => {
                        self.output.clear();
                        self.error = Some(self.format_error_message(&self.input, err));
                    }
                }

                self.save_to_storage();
                true
            }
            Msg::UpdateViewMode(mode) => {
                self.view_mode = match mode.as_str() {
                    "formatted" => JsonViewMode::Formatted,
                    "tree" => JsonViewMode::TreeView,
                    "yaml" => JsonViewMode::Yaml,
                    "table" => JsonViewMode::Table,
                    _ => JsonViewMode::Formatted, // Default to formatted
                };
                self.show_tree_view = self.view_mode == JsonViewMode::TreeView;
                self.save_to_storage();
                true
            }
            Msg::ToggleNode(path) => {
                if self.expanded_nodes.contains(&path) {
                    self.expanded_nodes.remove(&path);
                } else {
                    self.expanded_nodes.insert(path);
                }
                true
            }
            Msg::SortColumn(column_name) => {
                if self.table_state.sort_column.as_ref() == Some(&column_name) {
                    // 같은 컬럼을 클릭한 경우 정렬 방향 변경
                    self.table_state.sort_direction = match self.table_state.sort_direction {
                        SortDirection::None => SortDirection::Ascending,
                        SortDirection::Ascending => SortDirection::Descending,
                        SortDirection::Descending => SortDirection::None,
                    };
                    if self.table_state.sort_direction == SortDirection::None {
                        self.table_state.sort_column = None;
                    }
                } else {
                    // 새로운 컬럼을 클릭한 경우
                    self.table_state.sort_column = Some(column_name);
                    self.table_state.sort_direction = SortDirection::Ascending;
                }
                self.table_state.current_page = 0; // 정렬 시 첫 페이지로 이동
                true
            }
            Msg::ChangePage(page) => {
                self.table_state.current_page = page;
                true
            }
            Msg::ChangeRowsPerPage(rows_per_page) => {
                self.table_state.rows_per_page = rows_per_page;
                self.table_state.current_page = 0; // 페이지 크기 변경 시 첫 페이지로 이동
                true
            }
            Msg::UpdateSearchQuery(query) => {
                self.table_state.search_query = query;
                self.table_state.current_page = 0; // 검색어 변경 시 첫 페이지로 이동
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                        <h1 class="tool-title">
                            { "JSON Formatter & Converter" }
                        </h1>
                <div class="tool-wrapper">
                        <div class="tool-intro">
                        <div class="content-section">
                            <h2>{"🔤 What is JSON?"}</h2>
                            <p>{"JSON (JavaScript Object Notation) is a lightweight, text-based data format used for data interchange between systems. It is easy for humans to read and write, and easy for machines to parse and generate."}</p>
                            <p>{"JSON is widely used in web APIs, configuration files, and data storage due to its simplicity and compatibility with most programming languages."}</p>
                        </div>

                        <div class="content-section">
                            <h2>{"⚙️ How This JSON Formatter & Converter Works"}</h2>
                            <p>{"This tool formats and validates JSON data, making it easier to read, debug, and share. It also highlights syntax errors and allows you to customize the indentation style for your needs."}</p>
                            <h3>{"Supported Features:"}</h3>
                            <ul>
                                <li><strong>{"Pretty Printing:"}</strong> {"Format unstructured JSON into a human-readable, indented format. Field order is preserved as in your input JSON."}</li>
                                <li><strong>{"Tree View Visualization:"}</strong> {"Interactive tree structure display with expand/collapse functionality for easy JSON navigation. Object fields are shown in the same order as your input JSON."}</li>
                                <li><strong>{"Table View:"}</strong> {"Interactive table display with sorting, searching, and pagination for array of objects data. Columns are displayed in the exact order as defined in your input JSON. Removing sorting will restore the original order."}</li>
                                <li><strong>{"YAML Conversion:"}</strong> {"Convert JSON data to YAML format for configuration files and data exchange. Field order is preserved when converting to YAML."}</li>
                                <li><strong>{"Validation:"}</strong> {"Detect syntax errors and display detailed error messages with line and column numbers."}</li>
                                <li><strong>{"Indentation Options:"}</strong> {"Choose between 2, 3, 4 spaces, tab, or compact (no indent)."}</li>
                                <li><strong>{"Copy with Notification:"}</strong> {"Click any output field to copy results with visual feedback."}</li>
                                <li><strong>{"Local Processing:"}</strong> {"All formatting and validation happens in your browser for privacy and speed."}</li>
                                <li><strong>{"Order Preservation:"}</strong> {"All views (Formatted, Tree, Table, YAML) display fields in the exact order as your input JSON."}</li>
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
                            <h2>{"🌳 JSON Tree View Feature"}</h2>
                            <p>{"The Tree View feature provides an interactive, hierarchical visualization of your JSON data structure. This powerful tool helps you understand complex JSON structures at a glance and navigate through nested objects and arrays with ease."}</p>
                            
                            <h3>{"🎯 Key Features:"}</h3>
                            <ul>
                                <li><strong>{"Interactive Navigation:"}</strong> {"Click on any object or array node to expand or collapse its contents."}</li>
                                <li><strong>{"Visual Type Indicators:"}</strong> {"Each data type has a unique icon for quick identification:"}</li>
                                <ul>
                                    <li>{"📁/📂 Objects: Folders (closed/open)"}</li>
                                    <li>{"📋/📄 Arrays: Lists (closed/open)"}</li>
                                    <li>{"📄 Strings: Document icon"}</li>
                                    <li>{"🔢 Numbers: Number icon"}</li>
                                    <li>{"✅/❌ Booleans: Checkbox icons"}</li>
                                    <li>{"❓ Null: Question mark icon"}</li>
                                </ul>
                                <li><strong>{"Path Information:"}</strong> {"Each node displays its JSON path (e.g., 'user.address.city') for easy reference."}</li>
                                <li><strong>{"Size Indicators:"}</strong> {"Objects and arrays show their element count (e.g., '3 items') for quick assessment."}</li>
                                <li><strong>{"Real-time Updates:"}</strong> {"Tree view updates automatically as you modify the JSON input."}</li>
                            </ul>

                            <h3>{"📋 How to Use Tree View:"}</h3>
                            <ol>
                                <li>{"Enter or paste your JSON data in the input field."}</li>
                                <li>{"Click the '🌳 Tree View' button to switch to tree visualization mode."}</li>
                                <li>{"Click on any folder (📂) or list (📄) icon to expand that node."}</li>
                                <li>{"Click again on an open folder (📁) or list (📋) to collapse it."}</li>
                                <li>{"Navigate through the structure to understand your JSON hierarchy."}</li>
                                <li>{"Click '📄 Formatted' to return to the traditional formatted view."}</li>
                            </ol>

                            <div class="example-box">
                                <p><strong>{"Example JSON Structure in Tree View:"}</strong></p>
                                <pre style="color: var(--color-font); white-space: pre; font-family: monospace; margin: 0; padding-left: 20px;">
{ r#"📂 root (3 items)
├── 📄 name: "Alice"
├── 🔢 age: 30
└── 📋 skills (2 items)
    ├── 📄 [0]: "Rust"
    └── 📄 [1]: "Yew""# }
                                </pre>
                                <p><strong>{"Benefits:"}</strong></p>
                                <ul>
                                    <li>{"Quick structure understanding for complex JSON data"}</li>
                                    <li>{"Easy navigation through deeply nested objects"}</li>
                                    <li>{"Visual identification of data types and relationships"}</li>
                                    <li>{"Helpful for debugging and data analysis"}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"📊 JSON Table View Feature"}</h2>
                            <p>{"The Table View feature transforms JSON arrays of objects into interactive, sortable tables with advanced search and pagination capabilities. This powerful tool is perfect for analyzing structured data, comparing records, and exploring large datasets in a familiar spreadsheet-like interface."}</p>
                            
                            <h3>{"🎯 Key Features:"}</h3>
                            <ul>
                                <li><strong>{"Interactive Sorting:"}</strong> {"Click any column header to sort data in ascending or descending order. Click again to reverse the sort direction, or click a third time to remove sorting."}</li>
                                <li><strong>{"Real-time Search:"}</strong> {"Search across all columns simultaneously with case-insensitive matching. Results update instantly as you type."}</li>
                                <li><strong>{"Smart Pagination:"}</strong> {"Navigate through large datasets with customizable page sizes (5, 10, 20, or 50 rows per page)."}</li>
                                <li><strong>{"Data Type Detection:"}</strong> {"Automatic detection and display of data types (string, number, boolean, object, array, null) for each column."}</li>
                                <li><strong>{"Responsive Design:"}</strong> {"Table automatically adjusts height based on content, with horizontal scrolling for wide datasets."}</li>
                                <li><strong>{"Search Result Statistics:"}</strong> {"Shows filtered result count and total records for better data understanding."}</li>
                            </ul>

                            <h3>{"📋 How to Use Table View:"}</h3>
                            <ol>
                                <li>{"Enter or paste a JSON array of objects in the input field."}</li>
                                <li>{"Select 'Table View' from the View Mode dropdown."}</li>
                                <li>{"Use the search box to filter data across all columns."}</li>
                                <li>{"Adjust 'Rows per page' to control how many records are displayed."}</li>
                                <li>{"Click column headers to sort data by that column."}</li>
                                <li>{"Use pagination controls to navigate through large datasets."}</li>
                                <li>{"View data type information in column headers for better understanding."}</li>
                            </ol>

                            <div class="example-box">
                                <p><strong>{"Example JSON Structure for Table View:"}</strong></p>
                                <pre style="color: var(--color-font); white-space: pre; font-family: monospace; margin: 10px 0; padding: 10px; background-color: var(--color-bg); border-radius: 5px;">
{r#"[
  {
    "id": 1,
    "name": "Alice Johnson",
    "email": "alice@example.com",
    "age": 28,
    "active": true,
    "department": "Engineering"
  },
  {
    "id": 2,
    "name": "Bob Smith",
    "email": "bob@example.com",
    "age": 32,
    "active": false,
    "department": "Marketing"
  },
  {
    "id": 3,
    "name": "Carol Davis",
    "email": "carol@example.com",
    "age": 25,
    "active": true,
    "department": "Engineering"
  }
]"#}
                                </pre>
                                <p><strong>{"Table View Output:"}</strong></p>
                                <div style="background-color: var(--color-bg); border: 1px solid var(--color-border); border-radius: 5px; padding: 10px; margin: 10px 0;">
                                    <table style="width: 100%; border-collapse: collapse; font-family: monospace; font-size: 12px;">
                                        <thead>
                                            <tr style="background-color: var(--color-fourth); color: white;">
                                                <th style="padding: 8px; border: 1px solid var(--color-border); cursor: pointer;">{"id ↑"}</th>
                                                <th style="padding: 8px; border: 1px solid var(--color-border); cursor: pointer;">{"name"}</th>
                                                <th style="padding: 8px; border: 1px solid var(--color-border); cursor: pointer;">{"email"}</th>
                                                <th style="padding: 8px; border: 1px solid var(--color-border); cursor: pointer;">{"age"}</th>
                                                <th style="padding: 8px; border: 1px solid var(--color-border); cursor: pointer;">{"active"}</th>
                                                <th style="padding: 8px; border: 1px solid var(--color-border); cursor: pointer;">{"department"}</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            <tr style="border-bottom: 1px solid var(--color-border);">
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"1"}</td>
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"Alice Johnson"}</td>
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"alice@example.com"}</td>
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"28"}</td>
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"true"}</td>
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"Engineering"}</td>
                                            </tr>
                                            <tr style="border-bottom: 1px solid var(--color-border);">
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"2"}</td>
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"Bob Smith"}</td>
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"bob@example.com"}</td>
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"32"}</td>
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"false"}</td>
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"Marketing"}</td>
                                            </tr>
                                            <tr style="border-bottom: 1px solid var(--color-border);">
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"3"}</td>
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"Carol Davis"}</td>
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"carol@example.com"}</td>
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"25"}</td>
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"true"}</td>
                                                <td style="padding: 8px; border: 1px solid var(--color-border);">{"Engineering"}</td>
                                            </tr>
                                        </tbody>
                                    </table>
                                    <div style="margin-top: 10px; font-size: 11px; color: var(--color-subfont);">
                                        {"Total: 3 rows, 6 columns"}
                                    </div>
                                </div>
                                <p><strong>{"Benefits:"}</strong></p>
                                <ul>
                                    <li>{"Quick data analysis and comparison across multiple records"}</li>
                                    <li>{"Efficient searching and filtering of large datasets"}</li>
                                    <li>{"Familiar spreadsheet-like interface for data exploration"}</li>
                                    <li>{"Sortable columns for data organization and pattern recognition"}</li>
                                    <li>{"Pagination for handling large datasets without performance issues"}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"📚 Step-by-Step Tutorial"}</h2>
                            <div class="tutorial-step">
                                <h3>{"Example 1: Formatting and Validating JSON"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Format and validate a JSON string with custom indentation. Check that the output preserves the field order of your input JSON."}</p>
                                <ol>
                                    <li>{"Paste or type your JSON string into the input field."}</li>
                                    <li>{"Select your preferred indentation style (e.g., 4 spaces, tab, compact)."}</li>
                                    <li>{"View the formatted JSON or error message in the output field."}</li>
                                    <li>{"Check that the order of fields in the output matches your input JSON."}</li>
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

                            <div class="tutorial-step">
                                <h3>{"Example 2: Using Tree View for Complex JSON Analysis"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Navigate and understand a complex JSON structure using the Tree View feature. Confirm that object fields are shown in the same order as your input JSON."}</p>
                                <ol>
                                    <li>{"Enter a complex JSON structure in the input field:"}</li>
                                    <pre style="color: var(--color-font); white-space: pre; font-family: monospace; margin: 10px 0; padding: 10px; background-color: var(--color-bg); border-radius: 5px;">
{r#"{
  "user": {
    "id": 12345,
    "profile": {
      "name": "Alice",
      "email": "alice@example.com",
      "preferences": {
        "theme": "dark",
        "notifications": true
      }
    },
    "posts": [
      {"id": 1, "title": "First Post"},
      {"id": 2, "title": "Second Post"}
    ]
  }
}"#}
                                    </pre>
                                    <li>{"Click the 'Tree View' button to switch to tree visualization mode."}</li>
                                    <li>{"Click on the 'user' folder to expand it and see its contents."}</li>
                                    <li>{"Click on 'profile' to expand the nested profile object."}</li>
                                    <li>{"Navigate through 'preferences' to see the nested settings."}</li>
                                    <li>{"Click on 'posts' array to see the list of posts."}</li>
                                    <li>{"Use the tree structure to quickly understand the data hierarchy."}</li>
                                    <li>{"Check that the order of fields in the tree matches your input JSON."}</li>
                                    <li>{"Click 'Formatted' to return to the traditional formatted view."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Tree View Output:"}</strong></p>
                                    <pre style="color: var(--color-font); white-space: pre; font-family: monospace; margin: 0; padding-left: 20px;">
{ r#"📂 root (1 items)
└── 📁 user (3 items)
    ├── 🔢 id: 12345
    ├── 📁 profile (3 items)
    │   ├── 📄 name: "Alice"
    │   ├── 📄 email: "alice@example.com"
    │   └── 📁 preferences (2 items)
    │       ├── 📄 theme: "dark"
    │       └── ✅ notifications: true
    └── 📋 posts (2 items)
        ├── 📁 [0] (2 items)
        │   ├── 🔢 id: 1
        │   └── 📄 title: "First Post"
        └── 📁 [1] (2 items)
            ├── 🔢 id: 2
            └── 📄 title: "Second Post""# }
                                    </pre>
                                </div>
                            </div>

                            <div class="tutorial-step">
                                <h3>{"Example 3: Using Table View for Data Analysis"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Analyze and explore a dataset of user information using Table View's interactive features. Confirm that columns are displayed in the same order as your input JSON."}</p>
                                <ol>
                                    <li>{"Enter a JSON array of objects in the input field:"}</li>
                                    <pre style="color: var(--color-font); white-space: pre; font-family: monospace; margin: 10px 0; padding: 10px; background-color: var(--color-bg); border-radius: 5px;">
{r#"[
  {"id": 1, "name": "Alice Johnson", "email": "alice@example.com", "age": 28, "active": true, "department": "Engineering", "salary": 75000},
  {"id": 2, "name": "Bob Smith", "email": "bob@example.com", "age": 32, "active": false, "department": "Marketing", "salary": 65000},
  {"id": 3, "name": "Carol Davis", "email": "carol@example.com", "age": 25, "active": true, "department": "Engineering", "salary": 70000},
  {"id": 4, "name": "David Wilson", "email": "david@example.com", "age": 35, "active": true, "department": "Sales", "salary": 80000},
  {"id": 5, "name": "Eva Brown", "email": "eva@example.com", "age": 29, "active": false, "department": "Marketing", "salary": 60000}
]"#}
                                    </pre>
                                    <li>{"Select 'Table View' from the View Mode dropdown."}</li>
                                    <li>{"Click on the 'age' column header to sort by age in ascending order."}</li>
                                    <li>{"Click the 'age' header again to sort in descending order."}</li>
                                    <li>{"Type 'Engineering' in the search box to filter only Engineering department employees."}</li>
                                    <li>{"Change 'Rows per page' to 10 to see more records at once."}</li>
                                    <li>{"Click on 'salary' column to sort by salary and identify highest/lowest earners."}</li>
                                    <li>{"Use the search box to find specific employees by name or email."}</li>
                                    <li>{"Navigate through pages if you have more than 10 records."}</li>
                                    <li>{"Check that the order of columns in the table matches your input JSON."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Analysis Results:"}</strong></p>
                                    <ul>
                                        <li>{"Age range: 25-35 years old"}</li>
                                        <li>{"Department distribution: Engineering (2), Marketing (2), Sales (1)"}</li>
                                        <li>{"Active status: 3 active, 2 inactive employees"}</li>
                                        <li>{"Salary range: $60,000 - $80,000"}</li>
                                        <li>{"Average age: 29.8 years"}</li>
                                    </ul>
                                    <p><strong>{"Table View Benefits Demonstrated:"}</strong></p>
                                    <ul>
                                        <li>{"Quick sorting revealed age and salary patterns"}</li>
                                        <li>{"Search functionality helped identify department-specific data"}</li>
                                        <li>{"Pagination would handle larger datasets efficiently"}</li>
                                        <li>{"Data type detection showed numeric vs text fields clearly"}</li>
                                    </ul>
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
                            <h3>{"Why Use a JSON Formatter & Converter?"}</h3>
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
                            <div class="faq-item">
                                <h3>{"Q: What is the Tree View feature and when should I use it?"}</h3>
                                <p>{"A: Tree View provides an interactive, hierarchical visualization of your JSON structure. Use it when you need to quickly understand complex nested data, navigate through large JSON objects, or identify data types and relationships at a glance."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: How do I navigate the Tree View?"}</h3>
                                <p>{"A: Click on any folder (📂) or list (📄) icon to expand that node and see its contents. Click again on an open folder (📁) or list (📋) to collapse it. The tree shows the JSON path for each element and the number of items in objects and arrays."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: What do the different icons in Tree View mean?"}</h3>
                                <p>{"A: Each data type has a unique icon: 📁/📂 for objects (closed/open), 📋/📄 for arrays (closed/open), 📄 for strings, 🔢 for numbers, ✅/❌ for booleans, and ❓ for null values. This helps you quickly identify data types."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Can I use Tree View with invalid JSON?"}</h3>
                                <p>{"A: No, Tree View only works with valid JSON. If your JSON has syntax errors, the tool will display an error message instead of the tree visualization. Fix the JSON errors first to use Tree View."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Does Tree View work with very large JSON files?"}</h3>
                                <p>{"A: Yes, but performance may vary with extremely large files. Tree View is optimized for typical JSON structures and provides the best experience with moderately sized data (up to several MB). For very large files, consider using the formatted view."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Does this tool preserve the order of fields in my JSON?"}</h3>
                                <p>{"A: Yes! All views (Formatted, Tree, Table, YAML) display fields in the exact order as your input JSON, thanks to order-preserving parsing and serialization."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: What is Table View and when should I use it?"}</h3>
                                <p>{"A: Table View transforms JSON arrays of objects into interactive, sortable tables. Use it when you have structured data (like user lists, product catalogs, or API responses) that you want to analyze, search, or compare in a spreadsheet-like format."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: How does sorting work in Table View?"}</h3>
                                <p>{"A: Click any column header to sort by that column in ascending order. Click again for descending order, and click a third time to remove sorting. The current sort direction is indicated by arrows (↑ for ascending, ↓ for descending) next to the column name."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Can I search across multiple columns in Table View?"}</h3>
                                <p>{"A: Yes! The search box searches across all columns simultaneously. It's case-insensitive and finds partial matches, so typing 'eng' will find 'Engineering' department entries."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: What data types does Table View support?"}</h3>
                                <p>{"A: Table View automatically detects and displays all JSON data types: strings, numbers, booleans, objects, arrays, and null values. Object and array values show their item count (e.g., '3 items') for quick reference."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: How do I handle large datasets in Table View?"}</h3>
                                <p>{"A: Use the 'Rows per page' setting to control how many records are displayed at once. Choose from 5, 10, 20, or 50 rows per page. The pagination controls at the bottom allow you to navigate through all pages efficiently."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Can I use Table View with non-array JSON data?"}</h3>
                                <p>{"A: No, Table View requires JSON arrays of objects. If your JSON is a single object or other structure, you'll see an error message. Use Tree View or Formatted View for non-array data."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Does Table View preserve the original order of my data?"}</h3>
                                <p>{"A: Yes, by default Table View maintains the original order of your JSON array. Only when you explicitly sort by clicking column headers will the order change. You can always remove sorting to return to the original order."}</p>
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
                                <li><strong>{"Tree View Usage:"}</strong> {"Use Tree View for complex JSON analysis, API response inspection, and data structure understanding. It's particularly helpful for debugging nested objects and arrays."}</li>
                                <li><strong>{"Navigation Strategy:"}</strong> {"When using Tree View, start by expanding the root level, then systematically explore nested objects and arrays to understand the data hierarchy."}</li>
                                <li><strong>{"Type Identification:"}</strong> {"Pay attention to the icons in Tree View to quickly identify data types and understand the structure of your JSON data."}</li>
                                <li><strong>{"Path Awareness:"}</strong> {"Use the JSON paths displayed in Tree View to understand the exact location of data elements within your structure."}</li>
                                <li><strong>{"Table View Usage:"}</strong> {"Use Table View for analyzing structured data, comparing records, and exploring datasets with multiple similar objects. It's particularly effective for user lists, product catalogs, and API response analysis."}</li>
                                <li><strong>{"Data Preparation:"}</strong> {"Ensure your JSON is an array of objects for Table View. Each object should have consistent properties for the best table experience."}</li>
                                <li><strong>{"Search Strategy:"}</strong> {"Use the search box to quickly filter data across all columns. Combine search with sorting to find specific patterns or outliers in your data."}</li>
                                <li><strong>{"Pagination Management:"}</strong> {"Adjust 'Rows per page' based on your screen size and data complexity. Use smaller page sizes for detailed analysis and larger sizes for overview browsing."}</li>
                                <li><strong>{"Sorting Best Practices:"}</strong> {"Use column sorting to identify patterns, find outliers, and organize data logically. Sort by numeric fields to find highest/lowest values, and by text fields for alphabetical organization."}</li>
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
                        <div style="display: flex; align-items: center; margin-bottom: 10px; margin-top: 5px;">
                            <div style="width: 70%;">
                                {"View Mode: "}
                            </div>
                            <div style="width: 30%;">
                                <select
                                    id="view-mode-select"
                                    style="width: 100%;"
                                    onchange={_ctx.link().callback(|e: Event| {
                                        let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                        Msg::UpdateViewMode(value)
                                    })}>
                                    <option value="formatted" selected={self.view_mode == JsonViewMode::Formatted}>{ "Formatted View" }</option>
                                    <option value="yaml" selected={self.view_mode == JsonViewMode::Yaml}>{ "YAML View" }</option>
                                    <option value="tree" selected={self.view_mode == JsonViewMode::TreeView}>{ "Tree View" }</option>
                                    <option value="table" selected={self.view_mode == JsonViewMode::Table}>{ "Table View" }</option>
                                </select>
                            </div>
                        </div>
                        
                        if self.view_mode == JsonViewMode::Table {
                            <div style="display: flex; align-items: center; margin-bottom: 10px; margin-top: 5px;">
                                <div style="width: 70%;">
                                    {"Search: "}
                                </div>
                                <div style="width: 30%;">
                                    <input
                                        type="text"
                                        placeholder="Search in table..."
                                        value={self.table_state.search_query.clone()}
                                        oninput={_ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::UpdateSearchQuery(input.value())
                                        })}
                                        style="width: 100%; padding: 5px; font-size: 12px; border: 1px solid var(--color-border); border-radius: 3px;"
                                    />
                                </div>
                            </div>
                            <div style="display: flex; align-items: center; margin-bottom: 10px; margin-top: 5px;">
                                <div style="width: 70%;">
                                    {"Rows per page: "}
                        </div>
                                <div style="width: 30%;">
                                    <select
                                        value={self.table_state.rows_per_page.to_string()}
                                        onchange={_ctx.link().callback(|e: Event| {
                                            let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                            if let Ok(rows) = value.parse::<usize>() {
                                                Msg::ChangeRowsPerPage(rows)
                                            } else {
                                                Msg::ChangeRowsPerPage(5)
                                            }
                                        })}
                                        style="width: 100%; padding: 5px; font-size: 12px; border: 1px solid var(--color-border); border-radius: 3px;"
                                    >
                                        <option value="5" selected={self.table_state.rows_per_page == 5}>{"5"}</option>
                                        <option value="10" selected={self.table_state.rows_per_page == 10}>{"10"}</option>
                                        <option value="20" selected={self.table_state.rows_per_page == 20}>{"20"}</option>
                                        <option value="50" selected={self.table_state.rows_per_page == 50}>{"50"}</option>
                                    </select>
                                </div>
                            </div>
                        }
                        
                        if self.view_mode == JsonViewMode::Formatted {
                            <div style="display: flex; align-items: center; margin-bottom: 10px; margin-top: 5px;">
                                <div style="width: 70%;">
                                    {"Indentation Style: "}
                                </div>    
                                <select
                                    id="input-mode-select"
                                    style="width: 30%;"
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
                        }
                        
                        <div>
                            <div class="tool-inner">
                                <div>
                                    <div class="tool-subtitle" style="margin-bottom: 5px;">{ "Input" }</div>
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
                                <div class="tool-subtitle" style="width: 40%; margin-bottom: 0px;">
                                    if self.view_mode == JsonViewMode::Formatted {
                                        { "Formatted JSON" }
                                    } else if self.view_mode == JsonViewMode::TreeView {
                                        { "JSON Tree View" }
                                    } else if self.view_mode == JsonViewMode::Yaml {
                                        { "YAML Output" }
                                    } else if self.view_mode == JsonViewMode::Table {
                                        { "JSON Table View" }
                                    } else {
                                        { "Formatted JSON" }
                                    }
                                </div>
                            </div>
                            <div class="tool-inner">
                                <div>
                                    if self.view_mode == JsonViewMode::Formatted || self.view_mode == JsonViewMode::Yaml {
                                    <textarea
                                        type="text"
                                        readonly=true
                                        wrap="off"
                                            style={if self.compact { "cursor: pointer; overflow-y: auto; overflow-x: hidden; height: 350px; white-space: pre-wrap; word-wrap: break-word;" } else {"cursor: pointer; overflow: auto; height: 350px;"}}
                                        value={self.view_output()}
                                        onclick={_ctx.link().callback(|e: MouseEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::CopyToClipboard(input.value())
                                        })} />
                                    } else if self.view_mode == JsonViewMode::TreeView {
                                        <div style="min-height: 350px; max-height: 600px; overflow-y: auto; border: 1px solid var(--color-border); border-radius: 5px; padding: 10px; background-color: var(--color-bg);">
                                            { self.render_tree_view(_ctx) }
                                </div>
                                    } else {
                                        <div style="min-height: 350px; max-height: 600px; overflow-y: auto; border: 1px solid var(--color-border); border-radius: 5px; padding: 10px; background-color: var(--color-bg);">
                                            { self.render_table_view(_ctx) }
                                        </div>
                                    }
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
                    doc.set_title("JSON Formatter & Converter | CompuTools");

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
    // Local Storage 키 상수들
    const STORAGE_KEY_TAB_STYLE: &'static str = "json_tab_style";
    const STORAGE_KEY_VIEW_MODE: &'static str = "json_view_mode";

    fn get_local_storage() -> Option<Storage> {
        window()?.local_storage().ok()?
    }

    fn load_from_storage() -> Self {
        let storage = Self::get_local_storage();
        
        let tab_style = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_TAB_STYLE).ok().flatten())
            .unwrap_or("4space".to_string());

        let view_mode_str = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_VIEW_MODE).ok().flatten())
            .unwrap_or("formatted".to_string());

        let view_mode = match view_mode_str.as_str() {
            "tree" => JsonViewMode::TreeView,
            "yaml" => JsonViewMode::Yaml,
            "table" => JsonViewMode::Table,
            _ => JsonViewMode::Formatted,
        };

        let show_tree_view = view_mode == JsonViewMode::TreeView;

        Self {
            input: String::new(),
            output: String::new(),
            error: None,
            tab_style,
            compact: false,
            show_tree_view,
            expanded_nodes: HashSet::new(),
            view_mode,
            table_state: TableState {
                sort_column: None,
                sort_direction: SortDirection::None,
                current_page: 0,
                rows_per_page: 5,
                search_query: String::new(),
            },
        }
    }

    fn save_to_storage(&self) {
        if let Some(storage) = Self::get_local_storage() {
            let _ = storage.set_item(Self::STORAGE_KEY_TAB_STYLE, &self.tab_style);
            
            let view_mode_str = match self.view_mode {
                JsonViewMode::Formatted => "formatted",
                JsonViewMode::TreeView => "tree",
                JsonViewMode::Yaml => "yaml",
                JsonViewMode::Table => "table",
            };
            let _ = storage.set_item(Self::STORAGE_KEY_VIEW_MODE, view_mode_str);
        }
    }

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
            format!("{}", error)
        } else {
            match self.view_mode {
                JsonViewMode::Yaml => self.convert_to_yaml(),
                _ => format!("{}", self.output)
            }
        }
    }

    fn convert_to_yaml(&self) -> String {
        match serde_json::from_str::<serde_json::Value>(&self.input) {
            Ok(json_value) => {
                self.convert_to_yaml_with_order_preservation(&json_value)
            }
            Err(_) => {
                if let Some(error) = &self.error {
                    format!("Invalid JSON - cannot convert to YAML:\n{}", error)
                } else {
                    "Invalid JSON - cannot convert to YAML".to_string()
                }
            }
        }
    }

    fn format_json_with_order_preservation(&self, json_value: &serde_json::Value, indent: &[u8]) -> String {
        let mut output = Vec::new();
        
        if indent.is_empty() {
            let mut serializer = Serializer::with_formatter(
                Cursor::new(&mut output),
                serde_json::ser::CompactFormatter,
            );
            json_value.serialize(&mut serializer).unwrap();
        } else {
            let formatter = PrettyFormatter::with_indent(indent);
            let mut serializer = Serializer::with_formatter(Cursor::new(&mut output), formatter);
            json_value.serialize(&mut serializer).unwrap();
        }
        
        String::from_utf8(output).unwrap()
    }

    fn convert_to_yaml_with_order_preservation(&self, json_value: &serde_json::Value) -> String {
        match serde_yaml::to_string(json_value) {
            Ok(yaml_string) => yaml_string,
            Err(_) => "Error converting to YAML".to_string()
        }
    }

    fn render_tree_view(&self, _ctx: &Context<Self>) -> Html {
        match serde_json::from_str::<serde_json::Value>(&self.input) {
            Ok(json_value) => {
                html! {
                    <div class="tree-view-container" style="max-height: 400px; overflow-y: auto; font-family: monospace; font-size: 12px;">
                        { self.render_tree_node("root", &json_value, _ctx) }
                    </div>
                }
            }
            Err(_) => {
                html! {
                    <div style="color: var(--color-error); padding: 10px;">
                        { "Invalid JSON - cannot display tree view" }
                    </div>
                }
            }
        }
    }

    fn render_tree_node(&self, path: &str, value: &serde_json::Value, _ctx: &Context<Self>) -> Html {
        match value {
            serde_json::Value::Object(obj) => {
                let is_expanded = self.expanded_nodes.contains(path);
                let path_string = path.to_string();
                let toggle_callback = _ctx.link().callback(move |_| Msg::ToggleNode(path_string.clone()));
                
                html! {
                    <div class="tree-node" style="margin-left: 20px;">
                        <div class="node-header" style="display: flex; align-items: center; cursor: pointer; padding: 2px 0;" onclick={toggle_callback}>
                            <span style="margin-right: 5px;">
                                { if is_expanded { "📁" } else { "📂" } }
                            </span>
                            <span style="font-weight: bold; color: var(--color-fourth);">{ path }</span>
                            <span style="margin-left: 10px; color: var(--color-subfont); font-size: 11px;">
                                { format!("({} items)", obj.len()) }
                            </span>
                        </div>
                        if is_expanded {
                            <div class="node-children">
                                { for obj.keys().map(|k| {
                                    let v = obj.get(k).unwrap();
                                    let child_path = if path == "root" { 
                                        k.clone() 
                                    } else { 
                                        format!("{}.{}", path, k) 
                                    };
                                    self.render_tree_node(&child_path, v, _ctx)
                                }) }
                            </div>
                        }
                    </div>
                }
            }
            serde_json::Value::Array(arr) => {
                let is_expanded = self.expanded_nodes.contains(path);
                let path_string = path.to_string();
                let toggle_callback = _ctx.link().callback(move |_| Msg::ToggleNode(path_string.clone()));
                
                html! {
                    <div class="tree-node" style="margin-left: 20px;">
                        <div class="node-header" style="display: flex; align-items: center; cursor: pointer; padding: 2px 0;" onclick={toggle_callback}>
                            <span style="margin-right: 5px;">
                                { if is_expanded { "📋" } else { "📄" } }
                            </span>
                            <span style="font-weight: bold; color: var(--color-fourth);">{ path }</span>
                            <span style="margin-left: 10px; color: var(--color-subfont); font-size: 11px;">
                                { format!("({} items)", arr.len()) }
                            </span>
                        </div>
                        if is_expanded {
                            <div class="node-children">
                                { for arr.iter().enumerate().map(|(i, v)| {
                                    let child_path = format!("{}[{}]", path, i);
                                    self.render_tree_node(&child_path, v, _ctx)
                                }) }
                            </div>
                        }
                    </div>
                }
            }
            _ => {
                html! {
                    <div class="tree-node" style="margin-left: 20px;">
                        <div class="node-item" style="display: flex; align-items: center; padding: 2px 0;">
                            <span style="margin-right: 5px;">
                                { self.get_type_icon(value) }
                            </span>
                            <span style="font-weight: bold; color: var(--color-font);">{ path }</span>
                            <span style="margin-left: 10px; color: var(--color-subfont);">
                                { ":" }
                            </span>
                            <span style="margin-left: 5px; color: var(--color-font);">
                                { self.format_value(value) }
                            </span>
                        </div>
                    </div>
                }
            }
        }
    }

    fn get_type_icon(&self, value: &serde_json::Value) -> &'static str {
        match value {
            serde_json::Value::String(_) => "📄",
            serde_json::Value::Number(_) => "🔢",
            serde_json::Value::Bool(true) => "✅",
            serde_json::Value::Bool(false) => "❌",
            serde_json::Value::Null => "❓",
            serde_json::Value::Object(_) => "📁",
            serde_json::Value::Array(_) => "📋",
        }
    }

    fn format_value(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => format!("\"{}\"", s),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Null => "null".to_string(),
            serde_json::Value::Object(obj) => format!("{{ {} items }}", obj.len()),
            serde_json::Value::Array(arr) => format!("[ {} items ]", arr.len()),
        }
    }

    fn render_table(&self, _ctx: &Context<Self>, columns: Vec<TableColumn>, rows: Vec<TableRow>) -> Html {
        let (sorted_rows, total_pages) = self.get_sorted_and_paginated_rows(&columns, &rows);
        
        html! {
            <div style="overflow-x: auto;">
                <table style="width: 100%; border-collapse: collapse; border: 1px solid var(--color-border);">
                    <thead>
                        <tr style="background-color: var(--color-fourth); color: white;">
                            { for columns.iter().map(|col| {
                                let column_name = col.name.clone();
                                let sort_indicator = if self.table_state.sort_column.as_ref() == Some(&col.name) {
                                    match self.table_state.sort_direction {
                                        SortDirection::Ascending => " ↑",
                                        SortDirection::Descending => " ↓",
                                        SortDirection::None => "",
                                    }
                                } else {
                                    ""
                                };
                                
                                html! {
                                    <th 
                                        style="padding: 8px; text-align: left; border: 1px solid var(--color-border); font-weight: bold; cursor: pointer;"
                                        onclick={_ctx.link().callback(move |_| Msg::SortColumn(column_name.clone()))}
                                    >
                                        <div>{ &col.name }{ sort_indicator }</div>
                                        <div style="font-size: 10px; opacity: 0.8;">{ &col.data_type }</div>
                                    </th>
                                }
                            }) }
                        </tr>
                    </thead>
                    <tbody>
                        { for sorted_rows.iter().map(|row| {
                            html! {
                                <tr style="border-bottom: 1px solid var(--color-border);">
                                    { for row.values.iter().map(|value| {
                                        html! {
                                            <td style="padding: 8px; border: 1px solid var(--color-border); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 200px;">
                                                { value }
                                            </td>
                                        }
                                    }) }
                                </tr>
                            }
                        }) }
                    </tbody>
                </table>
                
                { self.render_pagination(_ctx, total_pages) }
                
                <div style="margin-top: 10px; font-size: 11px; color: var(--color-subfont);">
                    if !self.table_state.search_query.is_empty() {
                        { format!("Showing {} filtered results from {} total rows, {} columns", 
                            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&self.input) {
                                if let Ok((_, all_rows)) = self.convert_to_table_data(&json_value) {
                                    let filtered_count = all_rows.iter().filter(|row| {
                                        row.values.iter().any(|value| {
                                            value.to_lowercase().contains(&self.table_state.search_query.to_lowercase())
                                        })
                                    }).count();
                                    filtered_count
                                } else {
                                    rows.len()
                                }
                            } else {
                                rows.len()
                            },
                            rows.len(), 
                            columns.len()) }
                    } else {
                        { format!("Total: {} rows, {} columns", rows.len(), columns.len()) }
                    }
                </div>
            </div>
        }
    }

    fn render_table_view(&self, _ctx: &Context<Self>) -> Html {
        match serde_json::from_str::<serde_json::Value>(&self.input) {
            Ok(json_value) => {
                match self.convert_to_table_data(&json_value) {
                    Ok((columns, rows)) => {
                        html! {
                            <div class="table-view-container" style="max-height: 400px; overflow-y: auto; font-family: monospace; font-size: 12px;">
                                { self.render_table(_ctx, columns, rows) }
                            </div>
                        }
                    }
                    Err(_) => {
                        html! {
                            <div style="color: var(--color-error); padding: 10px;">
                                { "JSON structure is not suitable for table view (requires array of objects)" }
                            </div>
                        }
                    }
                }
            }
            Err(_) => {
                html! {
                    <div style="color: var(--color-error); padding: 10px;">
                        { "Invalid JSON - cannot display table view" }
                    </div>
                }
            }
        }
    }

    fn convert_to_table_data(&self, value: &serde_json::Value) -> Result<(Vec<TableColumn>, Vec<TableRow>), ()> {
        match value {
            serde_json::Value::Array(arr) => {
                if arr.is_empty() {
                    return Err(());
                }

                // 첫 번째 객체에서 컬럼 순서를 결정
                let first_item = &arr[0];
                let column_order: Vec<String> = if let serde_json::Value::Object(obj) = first_item {
                    obj.keys().cloned().collect()
                } else {
                    return Err(());
                };

                // 컬럼 정보 생성 (첫 번째 객체의 순서 유지)
                let columns: Vec<TableColumn> = column_order.iter().map(|key| {
                    let data_type = if let serde_json::Value::Object(obj) = first_item {
                        obj.get(key)
                            .map(|v| self.get_value_type(v))
                            .unwrap_or_else(|| "unknown".to_string())
                    } else {
                        "unknown".to_string()
                    };
                    
                    TableColumn {
                        name: key.clone(),
                        data_type,
                    }
                }).collect();

                // 모든 행 데이터 추출 (컬럼 순서 유지)
                let rows: Vec<TableRow> = arr.iter().filter_map(|item| {
                    if let serde_json::Value::Object(obj) = item {
                        let values: Vec<String> = column_order.iter().map(|col_name| {
                            obj.get(col_name)
                                .map(|v| self.format_value_for_table(v))
                                .unwrap_or_else(|| "".to_string())
                        }).collect();
                        Some(TableRow { values })
                    } else {
                        None
                    }
                }).collect();

                if rows.is_empty() {
                    Err(())
                } else {
                    Ok((columns, rows))
                }
            }
            _ => Err(()),
        }
    }

    fn get_value_type(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(_) => "string".to_string(),
            serde_json::Value::Number(_) => "number".to_string(),
            serde_json::Value::Bool(_) => "boolean".to_string(),
            serde_json::Value::Null => "null".to_string(),
            serde_json::Value::Object(_) => "object".to_string(),
            serde_json::Value::Array(_) => "array".to_string(),
        }
    }

    fn format_value_for_table(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Null => "null".to_string(),
            serde_json::Value::Object(obj) => format!("{{ {} items }}", obj.len()),
            serde_json::Value::Array(arr) => format!("[ {} items ]", arr.len()),
        }
    }

    fn get_sorted_and_paginated_rows(&self, columns: &[TableColumn], rows: &[TableRow]) -> (Vec<TableRow>, usize) {
        let mut filtered_rows = rows.to_vec();
        
        // 검색 필터링 적용
        if !self.table_state.search_query.is_empty() {
            filtered_rows = filtered_rows.into_iter().filter(|row| {
                row.values.iter().any(|value| {
                    value.to_lowercase().contains(&self.table_state.search_query.to_lowercase())
                })
            }).collect();
        }
        
        // 정렬이 명시적으로 요청된 경우에만 정렬 적용
        if let Some(sort_column) = &self.table_state.sort_column {
            if let Some(column_index) = columns.iter().position(|col| &col.name == sort_column) {
                filtered_rows.sort_by(|a, b| {
                    let a_val = &a.values[column_index];
                    let b_val = &b.values[column_index];
                    
                    let comparison = match self.table_state.sort_direction {
                        SortDirection::Ascending => a_val.cmp(b_val),
                        SortDirection::Descending => b_val.cmp(a_val),
                        SortDirection::None => std::cmp::Ordering::Equal,
                    };
                    
                    comparison
                });
            }
        }
        // 정렬이 요청되지 않은 경우 원본 순서 유지
        
        // 페이지네이션 적용
        let total_pages = (filtered_rows.len() + self.table_state.rows_per_page - 1) / self.table_state.rows_per_page;
        let start_index = self.table_state.current_page * self.table_state.rows_per_page;
        let end_index = std::cmp::min(start_index + self.table_state.rows_per_page, filtered_rows.len());
        
        let paginated_rows = if start_index < filtered_rows.len() {
            filtered_rows[start_index..end_index].to_vec()
        } else {
            vec![]
        };
        
        (paginated_rows, total_pages)
    }

    fn render_pagination(&self, _ctx: &Context<Self>, total_pages: usize) -> Html {
        if total_pages <= 1 {
            return html! {};
        }
        
        let current_page = self.table_state.current_page;
        
        html! {
            <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 15px; padding: 10px; background-color: var(--color-bg); border: 1px solid var(--color-border); border-radius: 5px;">
                <div style="display: flex; align-items: center; gap: 5px;">
                    <button
                        disabled={current_page == 0}
                        onclick={_ctx.link().callback(move |_| Msg::ChangePage(current_page.saturating_sub(1)))}
                        style="padding: 5px 10px; font-size: 12px; cursor: pointer;"
                        disabled={current_page == 0}
                    >
                        {"←"}
                    </button>
                    
                    { for (0..total_pages).map(|page| {
                        let page_num = page;
                        let is_current = page == current_page;
                        
                        html! {
                            <button
                                onclick={_ctx.link().callback(move |_| Msg::ChangePage(page_num))}
                                style={format!("padding: 5px 8px; font-size: 12px; cursor: pointer; margin: 0 2px; {}", 
                                    if is_current { "background-color: var(--color-fourth); color: white;" } else { "" }
                                )}
                            >
                                { page + 1 }
                            </button>
                        }
                    }) }
                    
                    <button
                        disabled={current_page >= total_pages.saturating_sub(1)}
                        onclick={_ctx.link().callback(move |_| Msg::ChangePage(current_page + 1))}
                        style="padding: 5px 10px; font-size: 12px; cursor: pointer;"
                        disabled={current_page >= total_pages.saturating_sub(1)}
                    >
                        {"→"}
                    </button>
                </div>
                
                <div style="font-size: 12px; color: var(--color-subfont);">
                    { format!("Page {} of {}", current_page + 1, total_pages) }
                </div>
            </div>
        }
    }
}
