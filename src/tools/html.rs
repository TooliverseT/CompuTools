use yew::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlInputElement, Storage};
use crate::components::tool_category::ToolCategoryManager;
use regex::Regex;

#[derive(Clone, PartialEq)]
pub enum HtmlMode {
    Encode,
    Decode,
}

#[derive(Clone, PartialEq)]
pub enum EntityStyle {
    Named,         // &lt; &gt; &amp; &quot; &apos;
    Decimal,       // &#60; &#62; &#38; &#34; &#39;
    Hexadecimal,   // &#x3C; &#x3E; &#x26; &#x22; &#x27;
    MixedNamedHex, // Named for common, hex for others
    MixedNamedDecimal, // Named for common, decimal for others
    None,          // No encoding (pass through)
}

#[derive(Clone, PartialEq)]
pub enum SelectiveMode {
    Essential,     // Only &, <, >, ", '
    Extended,      // Common HTML entities + accented characters
    Unicode,       // All non-ASCII characters
    All,          // Everything that can be encoded
    Custom,       // User-defined characters
}

pub struct ToolHtml {
    input_text: String,
    output_text: String,
    mode: HtmlMode,
    entity_style: EntityStyle,
    selective_mode: SelectiveMode,
    error_message: Option<String>,
    show_entity_table: bool, // HTML Entity 테이블 표시 여부
    custom_chars: String,    // 사용자 정의 인코딩할 문자들
}

pub enum Msg {
    UpdateInput(String),
    CopyToClipboard(String),
    Convert, // 모드 전환 버튼
    EntityStyleChanged(EntityStyle),
    SelectiveModeChanged(SelectiveMode),
    ToggleEntityTable, // HTML Entity 테이블 토글
    InsertEntity(String), // 엔티티를 입력창에 삽입
    UpdateCustomChars(String), // 사용자 정의 문자 업데이트
}

impl Component for ToolHtml {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::load_from_storage()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateInput(text) => {
                self.input_text = text;
                self.error_message = None;

                if self.input_text.is_empty() {
                    self.output_text = String::new();
                    return true;
                }

                match self.mode {
                    HtmlMode::Encode => {
                        self.output_text = self.encode_html(&self.input_text);
                    }
                    HtmlMode::Decode => {
                        let warning_msg = ToolHtml::detect_invalid_entities(&self.input_text);
                        match self.decode_html(&self.input_text) {
                            Ok(decoded) => {
                                self.output_text = decoded;
                                if let Some(warning) = warning_msg {
                                    self.error_message = Some(warning);
                                }
                            }
                            Err(err) => {
                                self.error_message = Some(err);
                                self.output_text = String::new();
                            }
                        }
                    }
                }
                true
            }
            Msg::CopyToClipboard(value) => {
                if let Some(clipboard) = window().map(|w| w.navigator().clipboard()) {
                    wasm_bindgen_futures::spawn_local(async move {
                        let promise = clipboard.write_text(&value);
                        let future = JsFuture::from(promise);
                        let _ = future.await;
                    });
                }
                false
            }
            Msg::Convert => {
                self.mode = match self.mode {
                    HtmlMode::Encode => HtmlMode::Decode,
                    HtmlMode::Decode => HtmlMode::Encode,
                };
                self.error_message = None;
                // Re-process current input with new mode
                if !self.input_text.is_empty() {
                    let cb = _ctx.link().callback(|value| Msg::UpdateInput(value));
                    cb.emit(self.input_text.clone());
                }
                self.save_to_storage();
                true
            }
            Msg::EntityStyleChanged(style) => {
                self.entity_style = style;
                // Re-process if in encode mode
                if self.mode == HtmlMode::Encode && !self.input_text.is_empty() {
                    self.output_text = self.encode_html(&self.input_text);
                }
                self.save_to_storage();
                true
            }
            Msg::SelectiveModeChanged(selective_mode) => {
                self.selective_mode = selective_mode;
                // Re-process if in encode mode
                if self.mode == HtmlMode::Encode && !self.input_text.is_empty() {
                    self.output_text = self.encode_html(&self.input_text);
                }
                self.save_to_storage();
                true
            }
            Msg::ToggleEntityTable => {
                self.show_entity_table = !self.show_entity_table;
                true
            }
            Msg::InsertEntity(entity) => {
                let mut new_input = self.input_text.clone();
                new_input.push_str(&entity);
                self.input_text = new_input;
                self.error_message = None;
                
                // 현재 모드에 따라 적절히 처리
                match self.mode {
                    HtmlMode::Encode => {
                        self.output_text = self.encode_html(&self.input_text);
                    }
                    HtmlMode::Decode => {
                        let warning_msg = ToolHtml::detect_invalid_entities(&self.input_text);
                        match self.decode_html(&self.input_text) {
                            Ok(decoded) => {
                                self.output_text = decoded;
                                if let Some(warning) = warning_msg {
                                    self.error_message = Some(warning);
                                }
                            }
                            Err(err) => {
                                self.error_message = Some(err);
                                self.output_text = String::new();
                            }
                        }
                    }
                }
                true
            }
            Msg::UpdateCustomChars(chars) => {
                self.custom_chars = chars;
                // Re-process if in encode mode
                if self.mode == HtmlMode::Encode && !self.input_text.is_empty() {
                    self.output_text = self.encode_html(&self.input_text);
                }
                self.save_to_storage();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let on_convert = _ctx.link().callback(|_| Msg::Convert);
        
        html! {
            <>
                        <h1 class="tool-title">
                            { "HTML Entity Converter" }
                        </h1>
                <div class="tool-wrapper">
                        <div class="tool-intro">
                        <div class="content-section">
                            <h2>{"🔤 What is an HTML Entity?"}</h2>
                            <p>{"An HTML entity is a special sequence of characters used to represent reserved or invisible characters in HTML. Entities are used to display characters that would otherwise be interpreted as HTML markup, such as <, >, &, or non-breaking spaces."}</p>
                            <p>{"HTML entities are essential for ensuring that web content displays correctly and securely, especially when working with user-generated or international text."}</p>
                        </div>

                        <div class="content-section">
                            <h2>{"⚙️ How This HTML Entity Converter Works"}</h2>
                            <p>{"This tool encodes and decodes HTML entities, supporting both standard and Unicode characters. It provides instant conversion and is ideal for web developers, content creators, and anyone working with HTML content."}</p>
                            <h3>{"🔥 Advanced Features:"}</h3>
                            <ul>
                                <li><strong>{"Multiple Entity Styles:"}</strong> {"Choose between Named (&amp;lt;), Decimal (&#60;), Hexadecimal (&#x3C;), Mixed (Named+Hex), Mixed (Named+Decimal), or None (pass-through)"}</li>
                                <li><strong>{"Selective Encoding Modes:"}</strong> {"Encode only essential characters, extended sets, Unicode, everything, or custom user-defined characters"}</li>
                                <li><strong>{"Custom Character Selection:"}</strong> {"Define your own set of characters to encode with the Custom mode - perfect for specific use cases"}</li>
                                <li><strong>{"Interactive Entity Reference:"}</strong> {"Comprehensive HTML entity table with click-to-insert functionality for all major entities"}</li>
                                <li><strong>{"Smart Input Recognition:"}</strong> {"Automatically detect and decode mixed entity formats"}</li>
                                <li><strong>{"Real-time Error Feedback:"}</strong> {"Instant validation with detailed error messages"}</li>
                                <li><strong>{"Flexible Output Styles:"}</strong> {"Customize entity format based on your needs"}</li>
                                <li><strong>{"Unicode Support:"}</strong> {"Handle international characters and symbols"}</li>
                                <li><strong>{"Copy with Feedback:"}</strong> {"Click any output field to copy results"}</li>
                            </ul>
                            
                            <h3>{"📊 Entity Style Examples:"}</h3>
                            <div class="example-box">
                                <p><strong>{"Input text: <div>Hello & \"World\"</div>"}</strong></p>
                                <p><strong>{"Named entities:"}</strong> {"&lt;div&gt;Hello &amp; &quot;World&quot;&lt;/div&gt;"}</p>
                                <p><strong>{"Decimal entities:"}</strong> {"&#60;div&#62;Hello &#38; &#34;World&#34;&#60;/div&#62;"}</p>
                                <p><strong>{"Hexadecimal entities:"}</strong> {"&#x3C;div&#x3E;Hello &#x26; &#x22;World&#x22;&#x3C;/div&#x3E;"}</p>
                                <p><strong>{"Mixed (Named + Hex):"}</strong> {"&lt;div&gt;Hello &amp; &quot;World&quot;&lt;/div&gt; (common chars named, others hex)"}</p>
                                <p><strong>{"Mixed (Named + Decimal):"}</strong> {"&lt;div&gt;Hello &amp; &quot;World&quot;&lt;/div&gt; (common chars named, others decimal)"}</p>
                                <p><strong>{"None (Pass Through):"}</strong> {"<div>Hello & \"World\"</div> (no encoding applied)"}</p>
                            </div>
                            
                            <h3>{"🎯 Selective Encoding Modes:"}</h3>
                            <div class="example-box">
                                <p><strong>{"Essential mode:"}</strong> {"Only encodes &, <, >, \", ' characters"}</p>
                                <p><strong>{"Extended mode:"}</strong> {"Common entities + accented characters (á, é, ñ, etc.)"}</p>
                                <p><strong>{"Unicode mode:"}</strong> {"All non-ASCII characters including emojis and symbols"}</p>
                                <p><strong>{"All mode:"}</strong> {"Every character that can be represented as an entity"}</p>
                                <p><strong>{"Custom mode:"}</strong> {"Only encode characters you specify manually - enter characters separated by commas"}</p>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"💡 Common Use Cases"}</h2>
                            <div class="use-case">
                                <h3>{"1. Web Development"}</h3>
                                <ul>
                                    <li><strong>{"Safe Content Display:"}</strong> {"Prevent HTML injection by encoding user input."}</li>
                                    <li><strong>{"Template Rendering:"}</strong> {"Ensure dynamic content displays correctly in templates."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"2. Internationalization"}</h3>
                                <ul>
                                    <li><strong>{"Unicode Characters:"}</strong> {"Encode and display non-ASCII characters in HTML documents."}</li>
                                    <li><strong>{"Multilingual Content:"}</strong> {"Support for accented letters, symbols, and scripts from around the world."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"3. Data Exchange & APIs"}</h3>
                                <ul>
                                    <li><strong>{"XML/JSON Embedding:"}</strong> {"Safely embed HTML content in XML or JSON payloads."}</li>
                                    <li><strong>{"Email Templates:"}</strong> {"Prepare HTML for use in email clients that require entity encoding."}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"📚 Step-by-Step Tutorial"}</h2>
                            <div class="tutorial-step">
                                <h3>{"Example 1: Encoding HTML with Different Styles"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Convert special characters using different entity formats."}</p>
                                <ol>
                                    <li>{"Set the mode to 'Encode'."}</li>
                                    <li>{"Choose your preferred entity style from the dropdown."}</li>
                                    <li>{"Select the selective mode based on what you want to encode."}</li>
                                    <li>{"Enter text containing special characters."}</li>
                                    <li>{"View the encoded result in your chosen format."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Input:"}</strong> {"<script>alert('Hello');</script>"}</p>
                                    <p><strong>{"Named entities:"}</strong> {"&lt;script&gt;alert(&apos;Hello&apos;);&lt;/script&gt;"}</p>
                                    <p><strong>{"Decimal entities:"}</strong> {"&#60;script&#62;alert(&#39;Hello&#39;);&#60;/script&#62;"}</p>
                                    <p><strong>{"Hexadecimal entities:"}</strong> {"&#x3C;script&#x3E;alert(&#x27;Hello&#x27;);&#x3C;/script&#x3E;"}</p>
                                    <p><strong>{"Mixed (Named+Hex):"}</strong> {"&lt;script&gt;alert(&apos;Hello&apos;);&lt;/script&gt;"}</p>
                                    <p><strong>{"Mixed (Named+Decimal):"}</strong> {"&lt;script&gt;alert(&apos;Hello&apos;);&lt;/script&gt;"}</p>
                                    <p><strong>{"None:"}</strong> {"<script>alert('Hello');</script> (unchanged)"}</p>
                                </div>
                            </div>
                            <div class="tutorial-step">
                                <h3>{"Example 2: Decoding Mixed Entity Formats"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Convert mixed HTML entities back to readable text."}</p>
                                <ol>
                                    <li>{"Set the mode to 'Decode'."}</li>
                                    <li>{"Enter text containing various entity formats."}</li>
                                    <li>{"The tool automatically recognizes and decodes all formats."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Input:"}</strong> {"&lt;div&gt;Hello &#38; &#x22;World&#x22;&lt;/div&gt;"}</p>
                                    <p><strong>{"Output:"}</strong> {"<div>Hello & \"World\"</div>"}</p>
                                </div>
                            </div>
                            <div class="tutorial-step">
                                <h3>{"Example 3: Using the HTML Entity Reference Table"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Quickly insert HTML entities using the interactive reference table."}</p>
                                <ol>
                                    <li>{"Click 'Show HTML Entity Table' to open the reference."}</li>
                                    <li>{"Browse through categories: Essential, Currency, Math, Punctuation, Accented characters, etc."}</li>
                                    <li>{"Click any entity (character, named, decimal, or hex) to insert it into your input field."}</li>
                                    <li>{"The inserted entity will be automatically processed based on your current mode and settings."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Example workflow:"}</strong></p>
                                    <ul>
                                        <li>{"Click '©' in the table → '©' is inserted into input"}</li>
                                        <li>{"Click '&copy;' in the table → '&copy;' is inserted into input"}</li>
                                        <li>{"Click '&#169;' in the table → '&#169;' is inserted into input"}</li>
                                        <li>{"Result automatically appears in output based on your current mode"}</li>
                                    </ul>
                                </div>
                            </div>
                            <div class="tutorial-step">
                                <h3>{"Example 4: Using Custom Character Selection"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Encode only specific characters that you define manually."}</p>
                                <ol>
                                    <li>{"Set the mode to 'Encode'."}</li>
                                    <li>{"Choose your preferred entity style (Named, Decimal, etc.)."}</li>
                                    <li>{"Select 'Custom' from the Selective Mode dropdown."}</li>
                                    <li>{"Enter the characters you want to encode in the custom characters field, separated by commas."}</li>
                                    <li>{"Enter your text and see only the specified characters get encoded."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Custom characters:"}</strong> {"<, >, @"}</p>
                                    <p><strong>{"Input:"}</strong> {"<div>Email: user@example.com & password</div>"}</p>
                                    <p><strong>{"Output (Named entities):"}</strong> {"&lt;div&gt;Email: user&#64;example.com & password&lt;/div&gt;"}</p>
                                    <p><strong>{"Note:"}</strong> {"Only <, >, and @ are encoded while & and other characters remain unchanged"}</p>
                                </div>
                                <div class="example-box">
                                    <p><strong>{"Use cases for Custom mode:"}</strong></p>
                                    <ul>
                                        <li>{"Email protection: encode only @ symbols"}</li>
                                        <li>{"Template safety: encode only specific template delimiters"}</li>
                                        <li>{"Selective escaping: encode only problematic characters for your specific context"}</li>
                                        <li>{"Performance optimization: minimize encoding overhead by targeting specific characters"}</li>
                                    </ul>
                                </div>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"📋 HTML Entity Reference Table"}</h2>
                            <p>
                                {"The complete HTML entity reference includes the most commonly used entities for web development. This interactive table shows each entity with its named form, decimal numeric form, hexadecimal numeric form, and character description. Click any entity to insert it into your input field."}
                            </p>
                            
                            <div style="margin: 20px 0;">
                                <button 
                                    class="tool-btn"
                                    onclick={_ctx.link().callback(|_| Msg::ToggleEntityTable)}
                                    style="padding: 10px 20px; background-color: var(--color-fourth); color: white; border: none; border-radius: 5px; cursor: pointer;"
                                >
                                    if self.show_entity_table {
                                        {"Hide HTML Entity Table"}
                                    } else {
                                        {"Show HTML Entity Table"}
                                    }
                                </button>
                            </div>
                            
                            if self.show_entity_table {
                                <div style="max-height: 400px; overflow-y: auto; overflow-x: auto; font-family: monospace; font-size: 12px; border: 1px solid #ddd; border-radius: 5px;">
                                    <table style="width: 100%; border-collapse: collapse; min-width: 800px;">
                                        <thead>
                                            <tr style="background-color: var(--color-fourth); color: white; position: sticky; top: 0;">
                                                <th style="padding: 8px; border: 1px solid #ddd;">{"Character"}</th>
                                                <th style="padding: 8px; border: 1px solid #ddd;">{"Named Entity"}</th>
                                                <th style="padding: 8px; border: 1px solid #ddd;">{"Decimal"}</th>
                                                <th style="padding: 8px; border: 1px solid #ddd;">{"Hexadecimal"}</th>
                                                <th style="padding: 8px; border: 1px solid #ddd;">{"Description"}</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            { self.render_entity_table(_ctx) }
                                        </tbody>
                                    </table>
                                </div>
                            }
                            
                            <p style="margin-top: 15px;">
                                {"Understanding these HTML entities is essential for web development, preventing XSS attacks, and ensuring proper text display across different browsers and platforms. Each entity has multiple representations that can be used interchangeably."}
                            </p>
                        </div>

                        <div class="content-section">
                            <h2>{"🔧 Technical Background"}</h2>
                            <h3>{"How HTML Entities Work"}</h3>
                            <p>{"HTML entities use a special syntax: &amp;name; for named entities (e.g., &amp;lt; for <), and &amp;#xHEX; or &amp;#DEC; for numeric entities. Browsers automatically convert these entities to their corresponding characters when rendering HTML."}</p>
                            <div class="example-box">
                                <p><strong>{"Example for Unicode Character:"}</strong></p>
                                <ul>
                                    <li>{"Input: Café"}</li>
                                    <li>{"Encoded: Caf&eacute; or Caf&#xE9;"}</li>
                                </ul>
                            </div>
                            <h3>{"Why Use HTML Entities?"}</h3>
                            <ul>
                                <li>{"Prevent HTML injection and XSS attacks."}</li>
                                <li>{"Ensure correct display of special and international characters."}</li>
                                <li>{"Maintain compatibility across browsers and email clients."}</li>
                            </ul>
                            <h3>{"Performance & Implementation"}</h3>
                            <ul>
                                <li><strong>{"Fast Conversion:"}</strong> {"Instant encoding/decoding in your browser."}</li>
                                <li><strong>{"No Server Required:"}</strong> {"All processing happens locally for privacy and speed."}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"❓ Frequently Asked Questions"}</h2>
                            <div class="faq-item">
                                <h3>{"Q: What is the difference between encoding and decoding?"}</h3>
                                <p>{"A: Encoding converts special characters to HTML entities, while decoding converts entities back to their original characters."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Can this tool handle Unicode characters?"}</h3>
                                <p>{"A: Yes, use the 'Encode with unicode' mode to convert non-ASCII characters to hexadecimal entities."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Is this tool safe for sensitive data?"}</h3>
                                <p>{"A: Yes, all processing happens locally in your browser. No data is sent to any server."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Why do I need to encode HTML?"}</h3>
                                <p>{"A: Encoding prevents browsers from interpreting special characters as HTML markup, ensuring your content displays as intended."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: What if I enter invalid HTML entities?"}</h3>
                                <p>{"A: The tool will attempt to decode as much as possible. Invalid entities will remain unchanged in the output."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: When should I use the 'None (Pass Through)' option?"}</h3>
                                <p>{"A: Use this option when you want to preview text without any encoding, or when working with content that should remain in its original form. This is useful for testing and comparing different encoding styles."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: What's the difference between the two Mixed options?"}</h3>
                                <p>{"A: 'Mixed (Named+Hex)' uses named entities for common characters and hexadecimal for others, while 'Mixed (Named+Decimal)' uses decimal numbers instead of hexadecimal for uncommon characters."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: How does the Custom mode work?"}</h3>
                                <p>{"A: Custom mode allows you to specify exactly which characters should be encoded. Simply enter the characters you want to encode in the custom characters field, separated by commas. Only those specific characters will be converted to HTML entities, while all other characters remain unchanged."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: What format should I use for the Custom characters field?"}</h3>
                                <p>{"A: Enter characters separated by commas. For example: '<, >, &, @, #' will encode only those five characters. You can include special characters, symbols, letters, or numbers - whatever you need to encode for your specific use case."}</p>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🎯 Best Practices"}</h2>
                            <ul>
                                <li><strong>{"Validate Input:"}</strong> {"Always check your input for unescaped special characters."}</li>
                                <li><strong>{"Error Handling:"}</strong> {"Handle invalid or incomplete entities gracefully in your applications."}</li>
                                <li><strong>{"Performance:"}</strong> {"Use local tools for instant conversion and privacy."}</li>
                                <li><strong>{"Documentation:"}</strong> {"Document when and why entity encoding is used in your codebase."}</li>
                                <li><strong>{"Testing:"}</strong> {"Test with a variety of characters, including Unicode and edge cases."}</li>
                                <li><strong>{"Security Awareness:"}</strong> {"Remember that encoding is essential for preventing XSS and injection attacks."}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"🔗 Related Tools"}</h2>
                            <ul>
                                {
                                    ToolCategoryManager::get_related_tools("html")
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
                                if self.mode == HtmlMode::Encode {
                                    {"Text to HTML Entities"}
                                } else {
                                    {"HTML Entities to Text"}
                                }
                            </div>
                            <div onclick={on_convert} class="tool-change" style="width: 10%; display: flex; justify-content: center;">
                                <i class="fa-solid fa-arrows-rotate"></i>
                            </div>
                        </div>

                        // Entity Style 선택 (Encode 모드일 때만 표시)
                        if self.mode == HtmlMode::Encode {
                            <div style="display: flex; align-items: center; margin-bottom: 10px;">
                                <div style="width: 70%; font-size: 13px;">
                                    {"Entity Style: "}
                                </div>
                                <select
                                    style="width: 30%; padding: 2px; font-size: 12px;"
                                    onchange={_ctx.link().callback(|e: Event| {
                                        let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                        match value.as_str() {
                                            "named" => Msg::EntityStyleChanged(EntityStyle::Named),
                                            "decimal" => Msg::EntityStyleChanged(EntityStyle::Decimal),
                                            "hexadecimal" => Msg::EntityStyleChanged(EntityStyle::Hexadecimal),
                                            "mixed_named_hex" => Msg::EntityStyleChanged(EntityStyle::MixedNamedHex),
                                            "mixed_named_decimal" => Msg::EntityStyleChanged(EntityStyle::MixedNamedDecimal),
                                            "none" => Msg::EntityStyleChanged(EntityStyle::None),
                                            _ => unreachable!(),
                                        }
                                    })}>
                                    <option value="named" selected={self.entity_style == EntityStyle::Named}>{ "Named" }</option>
                                    <option value="decimal" selected={self.entity_style == EntityStyle::Decimal}>{ "Decimal" }</option>
                                    <option value="hexadecimal" selected={self.entity_style == EntityStyle::Hexadecimal}>{ "Hexadecimal" }</option>
                                    <option value="mixed_named_hex" selected={self.entity_style == EntityStyle::MixedNamedHex}>{ "Mixed (Named for common, Hex for others)" }</option>
                                    <option value="mixed_named_decimal" selected={self.entity_style == EntityStyle::MixedNamedDecimal}>{ "Mixed (Named for common, Decimal for others)" }</option>
                                    <option value="none" selected={self.entity_style == EntityStyle::None}>{ "None (Pass Through)" }</option>
                                </select>
                            </div>

                            <div style="display: flex; align-items: center; margin-bottom: 10px;">
                                <div style="width: 70%; font-size: 13px;">
                                    {"Selective Mode: "}
                                </div>
                                <select
                                    style="width: 30%; padding: 2px; font-size: 12px;"
                                    onchange={_ctx.link().callback(|e: Event| {
                                        let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                        match value.as_str() {
                                            "essential" => Msg::SelectiveModeChanged(SelectiveMode::Essential),
                                            "extended" => Msg::SelectiveModeChanged(SelectiveMode::Extended),
                                            "unicode" => Msg::SelectiveModeChanged(SelectiveMode::Unicode),
                                            "all" => Msg::SelectiveModeChanged(SelectiveMode::All),
                                            "custom" => Msg::SelectiveModeChanged(SelectiveMode::Custom),
                                            _ => unreachable!(),
                                        }
                                    })}>
                                    <option value="essential" selected={self.selective_mode == SelectiveMode::Essential}>{ "Essential" }</option>
                                    <option value="extended" selected={self.selective_mode == SelectiveMode::Extended}>{ "Extended" }</option>
                                    <option value="unicode" selected={self.selective_mode == SelectiveMode::Unicode}>{ "Unicode" }</option>
                                    <option value="all" selected={self.selective_mode == SelectiveMode::All}>{ "All" }</option>
                                    <option value="custom" selected={self.selective_mode == SelectiveMode::Custom}>{ "Custom" }</option>
                                </select>
                            </div>

                            // Custom Characters 입력 필드 (SelectiveMode::Custom 일 때만 표시)
                            if self.selective_mode == SelectiveMode::Custom {
                                <div style="display: flex; align-items: center; margin-bottom: 10px;">
                                    <div style="width: 70%; font-size: 13px;">
                                        {"Custom Characters:"}
                                    </div>
                                    <input
                                        type="text"
                                        style="width: 30%; padding: 2px; border: 1px solid #ccc; border-radius: 4px; font-size: 12px;"
                                        value={self.custom_chars.clone()}
                                        oninput={_ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::UpdateCustomChars(input.value())
                                        })}
                                        placeholder="<, >, &, @"
                                    />
                                </div>
                                <div style="color: var(--color-subfont); font-size: 11px; margin-bottom: 10px; margin-top: -5px;">
                                    {"Enter characters separated by commas (e.g., <, >, &, @, #). Only these characters will be encoded."}
                                </div>
                            }
                        }

                        <div>
                            <div style="display: flex; align-items: center; padding-left: 20px; padding-right: 20px; margin-bottom: 10px; margin-top: 5px; padding-top: 5px; padding-bottom: 5px;">
                                <div class="tool-subtitle" style="width: 100%; margin-bottom: 0px;">
                                    if self.mode == HtmlMode::Encode {
                                        { "Text Input" }
                                    } else {
                                        { "HTML Entities Input" }
                                    }
                                </div>
                            </div>
                            <div class="tool-inner">
                                <div>
                                    <textarea
                                        type="text"
                                        style={if self.error_message.is_some() { 
                                            "overflow-y: auto; overflow-x: hidden; height: 150px; white-space: pre-wrap; word-wrap: break-word; border: 2px solid var(--color-error);" 
                                        } else { 
                                            "overflow-y: auto; overflow-x: hidden; height: 150px; white-space: pre-wrap; word-wrap: break-word;" 
                                        }}
                                        wrap="off"
                                        value={self.input_text.clone()}
                                        placeholder={
                                            if self.mode == HtmlMode::Encode {
                                                "Enter text to encode to HTML entities..."
                                            } else {
                                                "Enter HTML entities to decode to text..."
                                            }
                                        }
                                        oninput={_ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::UpdateInput(input.value())
                                        })}
                                    />
                                    if let Some(error_msg) = &self.error_message {
                                        <div style="color: var(--color-error); font-size: 12px; margin-top: 4px; line-height: 1.3;">
                                            { error_msg }
                                        </div>
                                    }
                                </div>
                            </div>
                        </div>
                        <div>
                            <div style="display: flex; align-items: center; padding-left: 20px; padding-right: 20px; margin-bottom: 10px; margin-top: 20px;">
                                <div class="tool-subtitle">
                                    if self.mode == HtmlMode::Encode {
                                        { "HTML Entities Output" }
                                    } else {
                                        { "Decoded Text Output" }
                                    }
                                </div>
                            </div>
                            <div class="tool-inner">
                                <div>
                                    <textarea
                                        type="text"
                                        readonly=true
                                        wrap="off"
                                        style={"cursor: pointer; overflow-y: auto; overflow-x: hidden; height: 150px; white-space: pre-wrap; word-wrap: break-word;"}
                                        value={self.output_text.clone()}
                                        placeholder={
                                            if self.mode == HtmlMode::Encode {
                                                "Encoded HTML entities will appear here..."
                                            } else {
                                                "Decoded text will appear here..."
                                            }
                                        }
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
                    doc.set_title("HTML Entity Converter | CompuTools");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "Advanced HTML Entity Converter with comprehensive encoding options. Features multiple entity styles (Named, Decimal, Hexadecimal, Mixed formats), selective encoding modes (Essential, Extended, Unicode, All, Custom), and interactive entity reference table. Supports custom character selection, real-time error feedback, and bidirectional conversion with Convert button. Perfect for web developers, security professionals, and content creators. Includes comprehensive entity support with click-to-insert functionality.").unwrap();
                    }
                }
            }
        }
    }
}

impl ToolHtml {
    // Local Storage 키 상수들
    const STORAGE_KEY_MODE: &'static str = "html_mode";
    const STORAGE_KEY_ENTITY_STYLE: &'static str = "html_entity_style";
    const STORAGE_KEY_SELECTIVE_MODE: &'static str = "html_selective_mode";
    const STORAGE_KEY_CUSTOM_CHARS: &'static str = "html_custom_chars";

    fn get_local_storage() -> Option<Storage> {
        window()?.local_storage().ok()?
    }

    fn load_from_storage() -> Self {
        let storage = Self::get_local_storage();
        
        let mode = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_MODE).ok().flatten())
            .and_then(|s| match s.as_str() {
                "encode" => Some(HtmlMode::Encode),
                "decode" => Some(HtmlMode::Decode),
                _ => None,
            })
            .unwrap_or(HtmlMode::Encode);

        let entity_style = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_ENTITY_STYLE).ok().flatten())
            .and_then(|s| match s.as_str() {
                "named" => Some(EntityStyle::Named),
                "decimal" => Some(EntityStyle::Decimal),
                "hexadecimal" => Some(EntityStyle::Hexadecimal),
                "mixed_named_hex" => Some(EntityStyle::MixedNamedHex),
                "mixed_named_decimal" => Some(EntityStyle::MixedNamedDecimal),
                "none" => Some(EntityStyle::None),
                _ => None,
            })
            .unwrap_or(EntityStyle::Named);

        let selective_mode = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_SELECTIVE_MODE).ok().flatten())
            .and_then(|s| match s.as_str() {
                "essential" => Some(SelectiveMode::Essential),
                "extended" => Some(SelectiveMode::Extended),
                "unicode" => Some(SelectiveMode::Unicode),
                "all" => Some(SelectiveMode::All),
                "custom" => Some(SelectiveMode::Custom),
                _ => None,
            })
            .unwrap_or(SelectiveMode::Essential);

        let custom_chars = storage
            .as_ref()
            .and_then(|s| s.get_item(Self::STORAGE_KEY_CUSTOM_CHARS).ok().flatten())
            .unwrap_or(String::new());

        Self {
            input_text: String::new(),
            output_text: String::new(),
            mode,
            entity_style,
            selective_mode,
            error_message: None,
            show_entity_table: false,
            custom_chars,
        }
    }

    fn save_to_storage(&self) {
        if let Some(storage) = Self::get_local_storage() {
            let mode_str = match self.mode {
                HtmlMode::Encode => "encode",
                HtmlMode::Decode => "decode",
            };
            let _ = storage.set_item(Self::STORAGE_KEY_MODE, mode_str);

            let entity_style_str = match self.entity_style {
                EntityStyle::Named => "named",
                EntityStyle::Decimal => "decimal",
                EntityStyle::Hexadecimal => "hexadecimal",
                EntityStyle::MixedNamedHex => "mixed_named_hex",
                EntityStyle::MixedNamedDecimal => "mixed_named_decimal",
                EntityStyle::None => "none",
            };
            let _ = storage.set_item(Self::STORAGE_KEY_ENTITY_STYLE, entity_style_str);

            let selective_mode_str = match self.selective_mode {
                SelectiveMode::Essential => "essential",
                SelectiveMode::Extended => "extended",
                SelectiveMode::Unicode => "unicode",
                SelectiveMode::All => "all",
                SelectiveMode::Custom => "custom",
            };
            let _ = storage.set_item(Self::STORAGE_KEY_SELECTIVE_MODE, selective_mode_str);

            let _ = storage.set_item(Self::STORAGE_KEY_CUSTOM_CHARS, &self.custom_chars);
        }
    }

    fn encode_html(&self, input: &str) -> String {
        // None 모드일 때는 그대로 반환
        if self.entity_style == EntityStyle::None {
            return input.to_string();
        }

        let chars: Vec<char> = input.chars().collect();
        let mut result = String::new();

        for &ch in &chars {
            if self.should_encode_char(ch) {
                result.push_str(&self.encode_char(ch));
            } else {
                result.push(ch);
            }
        }

        result
    }

    fn should_encode_char(&self, ch: char) -> bool {
        match self.selective_mode {
            SelectiveMode::Essential => {
                matches!(ch, '&' | '<' | '>' | '"' | '\'')
            }
            SelectiveMode::Extended => {
                matches!(ch, '&' | '<' | '>' | '"' | '\'' | '©' | '®' | '™' | '€' | '£' | '¥' | '¢' | '¿' | '¡' | 'á' | 'à' | 'â' | 'ä' | 'ã' | 'å' | 'ą' | 'æ' | 'ć' | 'ç' | 'é' | 'è' | 'ê' | 'ë' | 'ę' | 'í' | 'ì' | 'î' | 'ï' | 'ł' | 'ñ' | 'ó' | 'ò' | 'ô' | 'ö' | 'õ' | 'ø' | 'ś' | 'š' | 'ú' | 'ù' | 'û' | 'ü' | 'ý' | 'ÿ' | 'ź' | 'ž' | 'Á' | 'À' | 'Â' | 'Ä' | 'Ã' | 'Å' | 'Ą' | 'Æ' | 'Ć' | 'Ç' | 'É' | 'È' | 'Ê' | 'Ë' | 'Ę' | 'Í' | 'Ì' | 'Î' | 'Ï' | 'Ł' | 'Ñ' | 'Ó' | 'Ò' | 'Ô' | 'Ö' | 'Õ' | 'Ø' | 'Ś' | 'Š' | 'Ú' | 'Ù' | 'Û' | 'Ü' | 'Ý' | 'Ÿ' | 'Ź' | 'Ž')
            }
            SelectiveMode::Unicode => {
                !ch.is_ascii() || matches!(ch, '&' | '<' | '>' | '"' | '\'')
            }
            SelectiveMode::All => {
                !ch.is_ascii_alphanumeric() && ch != ' ' && ch != '\n' && ch != '\r' && ch != '\t'
            }
            SelectiveMode::Custom => {
                // 쉼표로 구분된 문자들을 파싱하여 정확히 매칭
                self.parse_custom_chars().contains(&ch)
            }
        }
    }

    fn parse_custom_chars(&self) -> Vec<char> {
        if self.custom_chars.trim().is_empty() {
            return Vec::new();
        }
        
        self.custom_chars
            .split(',')
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed.len() == 1 {
                    trimmed.chars().next()
                } else if trimmed.is_empty() {
                    None
                } else {
                    // 여러 문자로 이루어진 경우 첫 번째 문자만 사용
                    trimmed.chars().next()
                }
            })
            .collect()
    }

    fn encode_char(&self, ch: char) -> String {
        match self.entity_style {
            EntityStyle::Named => self.encode_char_named(ch),
            EntityStyle::Decimal => format!("&#{};", ch as u32),
            EntityStyle::Hexadecimal => format!("&#x{:X};", ch as u32),
            EntityStyle::MixedNamedHex => {
                // Named for common characters, hex for others
                match ch {
                    '&' => "&amp;".to_string(),
                    '<' => "&lt;".to_string(),
                    '>' => "&gt;".to_string(),
                    '"' => "&quot;".to_string(),
                    '\'' => "&apos;".to_string(),
                    ' ' => "&nbsp;".to_string(),
                    '©' => "&copy;".to_string(),
                    '®' => "&reg;".to_string(),
                    '™' => "&trade;".to_string(),
                    _ => format!("&#x{:X};", ch as u32),
                }
            }
            EntityStyle::MixedNamedDecimal => {
                // Named for common characters, decimal for others
                match ch {
                    '&' => "&amp;".to_string(),
                    '<' => "&lt;".to_string(),
                    '>' => "&gt;".to_string(),
                    '"' => "&quot;".to_string(),
                    '\'' => "&apos;".to_string(),
                    ' ' => "&nbsp;".to_string(),
                    '©' => "&copy;".to_string(),
                    '®' => "&reg;".to_string(),
                    '™' => "&trade;".to_string(),
                    _ => format!("&#{};", ch as u32),
                }
            }
            EntityStyle::None => {
                ch.to_string()
            }
        }
    }

    fn encode_char_named(&self, ch: char) -> String {
        match ch {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&apos;".to_string(),
            ' ' => "&nbsp;".to_string(),
            '©' => "&copy;".to_string(),
            '®' => "&reg;".to_string(),
            '™' => "&trade;".to_string(),
            '€' => "&euro;".to_string(),
            '£' => "&pound;".to_string(),
            '¥' => "&yen;".to_string(),
            '¢' => "&cent;".to_string(),
            'á' => "&aacute;".to_string(),
            'à' => "&agrave;".to_string(),
            'â' => "&acirc;".to_string(),
            'ä' => "&auml;".to_string(),
            'ã' => "&atilde;".to_string(),
            'å' => "&aring;".to_string(),
            'æ' => "&aelig;".to_string(),
            'ç' => "&ccedil;".to_string(),
            'é' => "&eacute;".to_string(),
            'è' => "&egrave;".to_string(),
            'ê' => "&ecirc;".to_string(),
            'ë' => "&euml;".to_string(),
            'í' => "&iacute;".to_string(),
            'ì' => "&igrave;".to_string(),
            'î' => "&icirc;".to_string(),
            'ï' => "&iuml;".to_string(),
            'ñ' => "&ntilde;".to_string(),
            'ó' => "&oacute;".to_string(),
            'ò' => "&ograve;".to_string(),
            'ô' => "&ocirc;".to_string(),
            'ö' => "&ouml;".to_string(),
            'õ' => "&otilde;".to_string(),
            'ø' => "&oslash;".to_string(),
            'ú' => "&uacute;".to_string(),
            'ù' => "&ugrave;".to_string(),
            'û' => "&ucirc;".to_string(),
            'ü' => "&uuml;".to_string(),
            'ý' => "&yacute;".to_string(),
            'ÿ' => "&yuml;".to_string(),
            // 대문자 버전들
            'Á' => "&Aacute;".to_string(),
            'À' => "&Agrave;".to_string(),
            'Â' => "&Acirc;".to_string(),
            'Ä' => "&Auml;".to_string(),
            'Ã' => "&Atilde;".to_string(),
            'Å' => "&Aring;".to_string(),
            'Æ' => "&AElig;".to_string(),
            'Ç' => "&Ccedil;".to_string(),
            'É' => "&Eacute;".to_string(),
            'È' => "&Egrave;".to_string(),
            'Ê' => "&Ecirc;".to_string(),
            'Ë' => "&Euml;".to_string(),
            'Í' => "&Iacute;".to_string(),
            'Ì' => "&Igrave;".to_string(),
            'Î' => "&Icirc;".to_string(),
            'Ï' => "&Iuml;".to_string(),
            'Ñ' => "&Ntilde;".to_string(),
            'Ó' => "&Oacute;".to_string(),
            'Ò' => "&Ograve;".to_string(),
            'Ô' => "&Ocirc;".to_string(),
            'Ö' => "&Ouml;".to_string(),
            'Õ' => "&Otilde;".to_string(),
            'Ø' => "&Oslash;".to_string(),
            'Ú' => "&Uacute;".to_string(),
            'Ù' => "&Ugrave;".to_string(),
            'Û' => "&Ucirc;".to_string(),
            'Ü' => "&Uuml;".to_string(),
            'Ý' => "&Yacute;".to_string(),
            // 명명된 엔티티가 없는 경우 16진수로 폴백
            _ => format!("&#x{:X};", ch as u32),
        }
    }

    fn decode_html(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() {
            return Ok(String::new());
        }

        // 1. 불완전/잘못된 엔티티 감지 (경고용)
        let warning_msg = Self::detect_invalid_entities(input);
        
        let mut result = input.to_string();
        
        // 기본 HTML 엔티티를 먼저 처리 (명명된 엔티티)
        let named_entities = [
            ("&amp;", "&"),
            ("&lt;", "<"),
            ("&gt;", ">"),
            ("&quot;", "\""),
            ("&apos;", "'"),
            ("&nbsp;", " "),
            ("&copy;", "©"),
            ("&reg;", "®"),
            ("&trade;", "™"),
            ("&euro;", "€"),
            ("&pound;", "£"),
            ("&yen;", "¥"),
            ("&cent;", "¢"),
            // 라틴 문자들
            ("&aacute;", "á"), ("&agrave;", "à"), ("&acirc;", "â"), ("&auml;", "ä"),
            ("&atilde;", "ã"), ("&aring;", "å"), ("&aelig;", "æ"), ("&ccedil;", "ç"),
            ("&eacute;", "é"), ("&egrave;", "è"), ("&ecirc;", "ê"), ("&euml;", "ë"),
            ("&iacute;", "í"), ("&igrave;", "ì"), ("&icirc;", "î"), ("&iuml;", "ï"),
            ("&ntilde;", "ñ"), ("&oacute;", "ó"), ("&ograve;", "ò"), ("&ocirc;", "ô"),
            ("&ouml;", "ö"), ("&otilde;", "õ"), ("&oslash;", "ø"), ("&uacute;", "ú"),
            ("&ugrave;", "ù"), ("&ucirc;", "û"), ("&uuml;", "ü"), ("&yacute;", "ý"),
            ("&yuml;", "ÿ"),
            // 대문자들
            ("&Aacute;", "Á"), ("&Agrave;", "À"), ("&Acirc;", "Â"), ("&Auml;", "Ä"),
            ("&Atilde;", "Ã"), ("&Aring;", "Å"), ("&AElig;", "Æ"), ("&Ccedil;", "Ç"),
            ("&Eacute;", "É"), ("&Egrave;", "È"), ("&Ecirc;", "Ê"), ("&Euml;", "Ë"),
            ("&Iacute;", "Í"), ("&Igrave;", "Ì"), ("&Icirc;", "Î"), ("&Iuml;", "Ï"),
            ("&Ntilde;", "Ñ"), ("&Oacute;", "Ó"), ("&Ograve;", "Ò"), ("&Ocirc;", "Ô"),
            ("&Ouml;", "Ö"), ("&Otilde;", "Õ"), ("&Oslash;", "Ø"), ("&Uacute;", "Ú"),
            ("&Ugrave;", "Ù"), ("&Ucirc;", "Û"), ("&Uuml;", "Ü"), ("&Yacute;", "Ý"),
        ];
        
        for (entity, replacement) in named_entities.iter() {
            result = result.replace(entity, replacement);
        }
        
        // 16진수 엔티티 처리 (&#x[0-9A-F]+; 형식)
        let hex_re = Regex::new(r"&#x([0-9A-Fa-f]+);").map_err(|_| "Regex compilation failed")?;
        
        while hex_re.is_match(&result) {
            result = hex_re.replace_all(&result, |caps: &regex::Captures| {
                let hex_str = &caps[1];
                if let Ok(code_point) = u32::from_str_radix(hex_str, 16) {
                    if let Some(character) = char::from_u32(code_point) {
                        character.to_string()
                    } else {
                        caps[0].to_string() // 유효하지 않은 코드 포인트는 원래 문자열 유지
                    }
                } else {
                    caps[0].to_string() // 16진수 파싱 실패시 원래 문자열 유지
                }
            }).to_string();
        }
        
        // 10진수 엔티티 처리 (&#[0-9]+; 형식)
        let decimal_re = Regex::new(r"&#([0-9]+);").map_err(|_| "Regex compilation failed")?;
        
        result = decimal_re.replace_all(&result, |caps: &regex::Captures| {
            let decimal_str = &caps[1];
            if let Ok(code_point) = decimal_str.parse::<u32>() {
                if let Some(character) = char::from_u32(code_point) {
                    character.to_string()
                } else {
                    caps[0].to_string()
                }
            } else {
                caps[0].to_string()
            }
        }).to_string();
        
        // 경고 메시지가 있으면 Ok(결과)로 반환, 경고는 UI에서 표시
        if let Some(warning) = warning_msg {
            // UI에서 self.error_message = Some(warning)으로 처리됨
            return Ok(result);
        }
        
        Ok(result)
    }

    // 불완전/잘못된 엔티티 감지 함수 추가
    fn detect_invalid_entities(input: &str) -> Option<String> {
        use std::collections::HashSet;
        let known_entities: HashSet<&'static str> = [
            "&amp;", "&lt;", "&gt;", "&quot;", "&apos;", "&nbsp;", "&copy;", "&reg;", "&trade;", "&euro;", "&pound;", "&yen;", "&cent;",
            "&aacute;", "&agrave;", "&acirc;", "&auml;", "&atilde;", "&aring;", "&aelig;", "&ccedil;", "&eacute;", "&egrave;", "&ecirc;", "&euml;",
            "&iacute;", "&igrave;", "&icirc;", "&iuml;", "&ntilde;", "&oacute;", "&ograve;", "&ocirc;", "&ouml;", "&otilde;", "&oslash;", "&uacute;",
            "&ugrave;", "&ucirc;", "&uuml;", "&yacute;", "&yuml;",
            "&Aacute;", "&Agrave;", "&Acirc;", "&Auml;", "&Atilde;", "&Aring;", "&AElig;", "&Ccedil;", "&Eacute;", "&Egrave;", "&Ecirc;", "&Euml;",
            "&Iacute;", "&Igrave;", "&Icirc;", "&Iuml;", "&Ntilde;", "&Oacute;", "&Ograve;", "&Ocirc;", "&Ouml;", "&Otilde;", "&Oslash;", "&Uacute;",
            "&Ugrave;", "&Ucirc;", "&Uuml;", "&Yacute;"
        ].iter().cloned().collect();
        let re = Regex::new(r"&[#a-zA-Z0-9xX]+;?").unwrap();
        for mat in re.find_iter(input) {
            let entity = mat.as_str();
            let idx = mat.start();
            // 줄 번호, 열 번호 계산
            let (line, col) = {
                let mut line = 1;
                let mut last_newline = 0;
                for (i, c) in input[..idx].char_indices() {
                    if c == '\n' {
                        line += 1;
                        last_newline = i + 1;
                    }
                }
                (line, idx - last_newline + 1)
            };
            let pos_info = format!(" at line {}, column {}", line, col);
            if entity.starts_with("&#x") || entity.starts_with("&#X") {
                // 16진수 엔티티
                let hex = entity.trim_start_matches("&#x").trim_start_matches("&#X").trim_end_matches(';');
                if hex.is_empty() {
                    return Some(format!("Invalid hexadecimal entity (no value): {}{}", entity, pos_info));
                }
                if u32::from_str_radix(hex, 16).is_err() {
                    return Some(format!("Invalid hexadecimal entity: {}{}", entity, pos_info));
                }
                if !entity.ends_with(';') {
                    return Some(format!("Incomplete hexadecimal entity (missing semicolon): {}{}", entity, pos_info));
                }
            } else if entity.starts_with("&#") {
                // 10진수 엔티티
                let dec = entity.trim_start_matches("&#").trim_end_matches(';');
                if dec.is_empty() {
                    return Some(format!("Invalid decimal entity (no value): {}{}", entity, pos_info));
                }
                if dec.parse::<u32>().is_err() {
                    return Some(format!("Invalid decimal entity: {}{}", entity, pos_info));
                }
                if !entity.ends_with(';') {
                    return Some(format!("Incomplete decimal entity (missing semicolon): {}{}", entity, pos_info));
                }
            } else if entity.ends_with(';') {
                // 명명된 엔티티
                if !known_entities.contains(entity) {
                    return Some(format!("Unknown named entity: {}{}", entity, pos_info));
                }
            } else {
                // 세미콜론 없는 불완전 명명 엔티티
                return Some(format!("Incomplete named entity (missing semicolon): {}{}", entity, pos_info));
            }
        }
        None
    }

    fn render_entity_table(&self, ctx: &Context<Self>) -> Html {
        let entities: Vec<(&str, &str, &str, &str, &str)> = vec![
            // Essential HTML entities
            ("&", "&amp;", "&#38;", "&#x26;", "Ampersand"),
            ("<", "&lt;", "&#60;", "&#x3C;", "Less than"),
            (">", "&gt;", "&#62;", "&#x3E;", "Greater than"),
            ("\"", "&quot;", "&#34;", "&#x22;", "Quotation mark"),
            ("'", "&apos;", "&#39;", "&#x27;", "Apostrophe"),
            (" ", "&nbsp;", "&#160;", "&#xA0;", "Non-breaking space"),
            
            // Currency symbols
            ("©", "&copy;", "&#169;", "&#xA9;", "Copyright"),
            ("®", "&reg;", "&#174;", "&#xAE;", "Registered trademark"),
            ("™", "&trade;", "&#8482;", "&#x2122;", "Trademark"),
            ("€", "&euro;", "&#8364;", "&#x20AC;", "Euro"),
            ("£", "&pound;", "&#163;", "&#xA3;", "Pound sterling"),
            ("¥", "&yen;", "&#165;", "&#xA5;", "Yen"),
            ("¢", "&cent;", "&#162;", "&#xA2;", "Cent"),
            ("¤", "&curren;", "&#164;", "&#xA4;", "Generic currency"),
            
            // Math symbols
            ("±", "&plusmn;", "&#177;", "&#xB1;", "Plus-minus"),
            ("×", "&times;", "&#215;", "&#xD7;", "Multiplication"),
            ("÷", "&divide;", "&#247;", "&#xF7;", "Division"),
            ("¼", "&frac14;", "&#188;", "&#xBC;", "One quarter"),
            ("½", "&frac12;", "&#189;", "&#xBD;", "One half"),
            ("¾", "&frac34;", "&#190;", "&#xBE;", "Three quarters"),
            ("°", "&deg;", "&#176;", "&#xB0;", "Degree"),
            ("²", "&sup2;", "&#178;", "&#xB2;", "Superscript 2"),
            ("³", "&sup3;", "&#179;", "&#xB3;", "Superscript 3"),
            
            // Punctuation
            ("–", "&ndash;", "&#8211;", "&#x2013;", "En dash"),
            ("—", "&mdash;", "&#8212;", "&#x2014;", "Em dash"),
            ("'", "&lsquo;", "&#8216;", "&#x2018;", "Left single quote"),
            ("'", "&rsquo;", "&#8217;", "&#x2019;", "Right single quote"),
            ("\"", "&ldquo;", "&#8220;", "&#x201C;", "Left double quote"),
            ("\"", "&rdquo;", "&#8221;", "&#x201D;", "Right double quote"),
            ("…", "&hellip;", "&#8230;", "&#x2026;", "Horizontal ellipsis"),
            ("•", "&bull;", "&#8226;", "&#x2022;", "Bullet"),
            
            // Accented characters (lowercase)
            ("à", "&agrave;", "&#224;", "&#xE0;", "A grave"),
            ("á", "&aacute;", "&#225;", "&#xE1;", "A acute"),
            ("â", "&acirc;", "&#226;", "&#xE2;", "A circumflex"),
            ("ã", "&atilde;", "&#227;", "&#xE3;", "A tilde"),
            ("ä", "&auml;", "&#228;", "&#xE4;", "A umlaut"),
            ("å", "&aring;", "&#229;", "&#xE5;", "A ring"),
            ("æ", "&aelig;", "&#230;", "&#xE6;", "AE ligature"),
            ("ç", "&ccedil;", "&#231;", "&#xE7;", "C cedilla"),
            ("è", "&egrave;", "&#232;", "&#xE8;", "E grave"),
            ("é", "&eacute;", "&#233;", "&#xE9;", "E acute"),
            ("ê", "&ecirc;", "&#234;", "&#xEA;", "E circumflex"),
            ("ë", "&euml;", "&#235;", "&#xEB;", "E umlaut"),
            ("ì", "&igrave;", "&#236;", "&#xEC;", "I grave"),
            ("í", "&iacute;", "&#237;", "&#xED;", "I acute"),
            ("î", "&icirc;", "&#238;", "&#xEE;", "I circumflex"),
            ("ï", "&iuml;", "&#239;", "&#xEF;", "I umlaut"),
            ("ñ", "&ntilde;", "&#241;", "&#xF1;", "N tilde"),
            ("ò", "&ograve;", "&#242;", "&#xF2;", "O grave"),
            ("ó", "&oacute;", "&#243;", "&#xF3;", "O acute"),
            ("ô", "&ocirc;", "&#244;", "&#xF4;", "O circumflex"),
            ("õ", "&otilde;", "&#245;", "&#xF5;", "O tilde"),
            ("ö", "&ouml;", "&#246;", "&#xF6;", "O umlaut"),
            ("ø", "&oslash;", "&#248;", "&#xF8;", "O slash"),
            ("ù", "&ugrave;", "&#249;", "&#xF9;", "U grave"),
            ("ú", "&uacute;", "&#250;", "&#xFA;", "U acute"),
            ("û", "&ucirc;", "&#251;", "&#xFB;", "U circumflex"),
            ("ü", "&uuml;", "&#252;", "&#xFC;", "U umlaut"),
            ("ý", "&yacute;", "&#253;", "&#xFD;", "Y acute"),
            ("ÿ", "&yuml;", "&#255;", "&#xFF;", "Y umlaut"),
            
            // Accented characters (uppercase)
            ("À", "&Agrave;", "&#192;", "&#xC0;", "A grave (upper)"),
            ("Á", "&Aacute;", "&#193;", "&#xC1;", "A acute (upper)"),
            ("Â", "&Acirc;", "&#194;", "&#xC2;", "A circumflex (upper)"),
            ("Ã", "&Atilde;", "&#195;", "&#xC3;", "A tilde (upper)"),
            ("Ä", "&Auml;", "&#196;", "&#xC4;", "A umlaut (upper)"),
            ("Å", "&Aring;", "&#197;", "&#xC5;", "A ring (upper)"),
            ("Æ", "&AElig;", "&#198;", "&#xC6;", "AE ligature (upper)"),
            ("Ç", "&Ccedil;", "&#199;", "&#xC7;", "C cedilla (upper)"),
            ("È", "&Egrave;", "&#200;", "&#xC8;", "E grave (upper)"),
            ("É", "&Eacute;", "&#201;", "&#xC9;", "E acute (upper)"),
            ("Ê", "&Ecirc;", "&#202;", "&#xCA;", "E circumflex (upper)"),
            ("Ë", "&Euml;", "&#203;", "&#xCB;", "E umlaut (upper)"),
            ("Ì", "&Igrave;", "&#204;", "&#xCC;", "I grave (upper)"),
            ("Í", "&Iacute;", "&#205;", "&#xCD;", "I acute (upper)"),
            ("Î", "&Icirc;", "&#206;", "&#xCE;", "I circumflex (upper)"),
            ("Ï", "&Iuml;", "&#207;", "&#xCF;", "I umlaut (upper)"),
            ("Ñ", "&Ntilde;", "&#209;", "&#xD1;", "N tilde (upper)"),
            ("Ò", "&Ograve;", "&#210;", "&#xD2;", "O grave (upper)"),
            ("Ó", "&Oacute;", "&#211;", "&#xD3;", "O acute (upper)"),
            ("Ô", "&Ocirc;", "&#212;", "&#xD4;", "O circumflex (upper)"),
            ("Õ", "&Otilde;", "&#213;", "&#xD5;", "O tilde (upper)"),
            ("Ö", "&Ouml;", "&#214;", "&#xD6;", "O umlaut (upper)"),
            ("Ø", "&Oslash;", "&#216;", "&#xD8;", "O slash (upper)"),
            ("Ù", "&Ugrave;", "&#217;", "&#xD9;", "U grave (upper)"),
            ("Ú", "&Uacute;", "&#218;", "&#xDA;", "U acute (upper)"),
            ("Û", "&Ucirc;", "&#219;", "&#xDB;", "U circumflex (upper)"),
            ("Ü", "&Uuml;", "&#220;", "&#xDC;", "U umlaut (upper)"),
            ("Ý", "&Yacute;", "&#221;", "&#xDD;", "Y acute (upper)"),
            
            // Special symbols
            ("§", "&sect;", "&#167;", "&#xA7;", "Section"),
            ("¶", "&para;", "&#182;", "&#xB6;", "Paragraph"),
            ("†", "&dagger;", "&#8224;", "&#x2020;", "Dagger"),
            ("‡", "&Dagger;", "&#8225;", "&#x2021;", "Double dagger"),
            ("‰", "&permil;", "&#8240;", "&#x2030;", "Per mille"),
            ("‹", "&lsaquo;", "&#8249;", "&#x2039;", "Left single angle quote"),
            ("›", "&rsaquo;", "&#8250;", "&#x203A;", "Right single angle quote"),
            ("«", "&laquo;", "&#171;", "&#xAB;", "Left double angle quote"),
            ("»", "&raquo;", "&#187;", "&#xBB;", "Right double angle quote"),
            ("¿", "&iquest;", "&#191;", "&#xBF;", "Inverted question mark"),
            ("¡", "&iexcl;", "&#161;", "&#xA1;", "Inverted exclamation"),
        ];

        let mut rows = Vec::new();
        
        for (i, (character, named, decimal, hex, description)) in entities.iter().enumerate() {
            let row_style = if i % 2 == 0 {
                "background-color: var(--color-third)"
            } else {
                ""
            };

            let character_display = if *character == " " {
                "[SPACE]".to_string()
            } else {
                character.to_string()
            };

            // 각 엔티티 형태를 클릭할 수 있도록 만들기
            let named_entity = named.to_string();
            let decimal_entity = decimal.to_string();
            let hex_entity = hex.to_string();
            let char_entity = character.to_string();

            rows.push(html! {
                <tr style={row_style}>
                    <td style="padding: 8px; border: 1px solid #ddd; text-align: center; font-weight: bold; cursor: pointer;"
                        onclick={ctx.link().callback(move |_| Msg::InsertEntity(char_entity.clone()))}
                        title="Click to insert character">
                        { character_display }
                    </td>
                    <td style="padding: 8px; border: 1px solid #ddd; text-align: center; cursor: pointer; color: var(--color-fourth);"
                        onclick={ctx.link().callback(move |_| Msg::InsertEntity(named_entity.clone()))}
                        title="Click to insert named entity">
                        { named }
                    </td>
                    <td style="padding: 8px; border: 1px solid #ddd; text-align: center; cursor: pointer; color: var(--color-fourth);"
                        onclick={ctx.link().callback(move |_| Msg::InsertEntity(decimal_entity.clone()))}
                        title="Click to insert decimal entity">
                        { decimal }
                    </td>
                    <td style="padding: 8px; border: 1px solid #ddd; text-align: center; cursor: pointer; color: var(--color-fourth);"
                        onclick={ctx.link().callback(move |_| Msg::InsertEntity(hex_entity.clone()))}
                        title="Click to insert hexadecimal entity">
                        { hex }
                    </td>
                    <td style="padding: 8px; border: 1px solid #ddd; text-align: left; font-size: 11px;">
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

fn encode_unicode_text(input: &str) -> String {
    input.chars().map(|c| {
        // 유니코드 문자일 경우 &#x<유니코드>로 변환
        if c.is_ascii() {
            c.to_string()
        } else {
            format!("&#x{:X};", c as u32)
        }
    }).collect()
}

fn decode_html_custom(input: &str) -> String {
    // 기본 HTML 엔티티를 먼저 처리 (공통 엔티티)
    let basic_entities = [
        ("&amp;", "&"),
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&apos;", "'"),
    ];
    
    let mut result = input.to_string();
    for (entity, replacement) in basic_entities.iter() {
        result = result.replace(entity, replacement);
    }
    
    // 유니코드 엔티티 처리 (&#x[0-9A-F]+; 형식)
    let re = Regex::new(r"&#x([0-9A-Fa-f]+);").unwrap();
    
    // 모든 유니코드 엔티티를 찾아서 처리
    while re.is_match(&result) {
        result = re.replace_all(&result, |caps: &regex::Captures| {
            let hex_str = &caps[1];
            if let Ok(code_point) = u32::from_str_radix(hex_str, 16) {
                if let Some(character) = char::from_u32(code_point) {
                    character.to_string()
                } else {
                    caps[0].to_string() // 유효하지 않은 코드 포인트는 원래 문자열 유지
                }
            } else {
                caps[0].to_string() // 16진수 파싱 실패시 원래 문자열 유지
            }
        }).to_string();
    }
    
    // 10진수 엔티티도 처리 (&#[0-9]+; 형식)
    let re_decimal = Regex::new(r"&#([0-9]+);").unwrap();
    
    result = re_decimal.replace_all(&result, |caps: &regex::Captures| {
        let decimal_str = &caps[1];
        if let Ok(code_point) = decimal_str.parse::<u32>() {
            if let Some(character) = char::from_u32(code_point) {
                character.to_string()
            } else {
                caps[0].to_string()
            }
        } else {
            caps[0].to_string()
        }
    }).to_string();
    
    result
}