use gloo_file::futures::read_as_bytes;
use gloo_file::File as GlooFile;
use gloo_timers::future::TimeoutFuture;
use indexmap::IndexMap;
use log::info;
use md5::{Digest as Md5Digest, Md5};
use sha1::{Digest as Sha1Digest, Sha1};
use sha2::{Digest as Sha2Digest, Sha256, Sha512};
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::Blob;
use web_sys::{window, HtmlInputElement};
use web_sys::{File, FileReader as WebFileReader, ProgressEvent};
use yew::prelude::*;

// 청크 처리를 위한 상수 - 성능 향상을 위해 청크 크기 증가
const CHUNK_SIZE: usize = 16 * 1024 * 1024;
const PROGRESS_UPDATE_INTERVAL: u32 = 1;
const UI_UPDATE_DELAY_MS: u32 = 10;
const PROGRESS_UPDATE_RETURN: u32 = 20;
const UPDATE_DELAY_MS: u32 = 5;

pub struct ToolFileHash {
    file_name: String,
    file_size: String,
    hash_md5: String,
    hash_sha1: String,
    hash_sha256: String,
    hash_sha512: String,
    is_computing: bool,
    step: bool,
    progress: f64,
    selected: IndexMap<String, bool>,
}

pub enum Msg {
    FileSelected(File),
    HashesComputed(String, String, String, String),
    CopyToClipboard(String),
    ComputeStarted,
    ProgressUpdate(bool, f64),
    Toggle(String),
    NoOp,
}

impl Component for ToolFileHash {
    type Message = Msg;
    type Properties = (); // No props needed

    fn create(_ctx: &Context<Self>) -> Self {
        let mut selected = IndexMap::new();
        let items = vec!["md5", "sha1", "sha256", "sha512"];

        for item in items {
            selected.insert(item.to_string(), true);
        }

        Self {
            file_name: "No file selected".to_string(),
            file_size: "".to_string(),
            hash_md5: "".to_string(),
            hash_sha1: "".to_string(),
            hash_sha256: "".to_string(),
            hash_sha512: "".to_string(),
            is_computing: false,
            step: false,
            progress: 0.0,
            selected,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FileSelected(file) => {
                let file_size = format!("{} bytes", file.size());
                let file_name = file.name();
                let link = _ctx.link().clone();
                
                // 계산 시작 상태로 변경
                link.send_message(Msg::ComputeStarted);
                
                // 선택된 해시 알고리즘 확인
                let selected = self.selected.clone();
                
                // 해시 계산을 청크 단위로 수행하여 UI 블로킹 방지
                spawn_local(async move {
                    let mut md5_hasher = 
                        selected.get("md5").copied().unwrap_or(false).then(Md5::new);
                    let mut sha1_hasher = selected
                        .get("sha1")
                        .copied()
                        .unwrap_or(false)
                        .then(Sha1::new);
                    let mut sha256_hasher = selected
                        .get("sha256")
                        .copied()
                        .unwrap_or(false)
                        .then(Sha256::new);
                    let mut sha512_hasher = selected
                        .get("sha512")
                        .copied()
                        .unwrap_or(false)
                        .then(Sha512::new);
                    
                    let total_size = file.size() as f64;
                    let mut bytes_processed: f64 = 0.0;
                    let mut chunk_counter: u32 = 0;

                    link.send_message(Msg::ProgressUpdate(false, 0.0));
                    
                    let total_chunks = (total_size / CHUNK_SIZE as f64).ceil() as usize;

                    // 파일을 청크 단위로 처리
                    for i in 0..total_chunks {
                        let start = (i as f64) * (CHUNK_SIZE as f64);
                        let end = (start + CHUNK_SIZE as f64).min(total_size);
                        
                        // 현재 청크 슬라이스 생성
                        let chunk = match file.slice_with_f64_and_f64(start as f64, end as f64) {
                            Ok(slice) => slice,
                            Err(_) => {
                                // 슬라이스 생성 실패 처리
                                link.send_message(Msg::HashesComputed(
                                    String::new(),
                                    String::new(),
                                    String::new(),
                                    String::new()
                                ));
                                return;
                            }
                        };

                        // 청크 데이터 읽기
                        let chunk_data = match read_slice_as_array_buffer(&chunk).await {
                            Ok(data) => data,
                            Err(_) => {
                                // 읽기 실패 처리
                                link.send_message(Msg::HashesComputed(
                                    String::new(),
                                    String::new(),
                                    String::new(),
                                    String::new()
                                ));
                                return;
                            }
                        };

                        // 해시 업데이트
                        if let Some(h) = &mut md5_hasher {
                            h.update(&chunk_data);
                        }
                        if let Some(h) = &mut sha1_hasher {
                            h.update(&chunk_data);
                        }
                        if let Some(h) = &mut sha256_hasher {
                            h.update(&chunk_data);
                        }
                        if let Some(h) = &mut sha512_hasher {
                            h.update(&chunk_data);
                        }
                        
                        // 처리 바이트 수 업데이트
                        bytes_processed += chunk_data.len() as f64;
                        chunk_counter = chunk_counter.wrapping_add(1);  // 오버플로우 방지
                        
                        // 진행 상황 업데이트 (일정 간격으로만)
                        if chunk_counter % PROGRESS_UPDATE_INTERVAL == 0 {
                            let progress = bytes_processed as f64 / total_size as f64;
                            link.send_message(Msg::ProgressUpdate(true, progress));
                            
                            // UI 업데이트를 위한 짧은 지연
                            TimeoutFuture::new(UI_UPDATE_DELAY_MS).await;
                        }

                        if chunk_counter % PROGRESS_UPDATE_RETURN == 0 {
                            TimeoutFuture::new(UPDATE_DELAY_MS).await;  // 5ms 지연으로 브라우저에 제어권 반환
                        }
                    }
                    
                    // 최종 진행률 업데이트
                    link.send_message(Msg::ProgressUpdate(true, 1.0));
                    
                    // 최종 해시 값 계산
                    let md5_result = md5_hasher.map(|h| format!("{:x}", h.finalize()));
                    let sha1_result = sha1_hasher.map(|h| format!("{:x}", h.finalize()));
                    let sha256_result = sha256_hasher.map(|h| format!("{:x}", h.finalize()));
                    let sha512_result = sha512_hasher.map(|h| format!("{:x}", h.finalize()));
                    
                    // 해시 값 전송
                    link.send_message(Msg::HashesComputed(
                        md5_result.unwrap_or_default(),
                        sha1_result.unwrap_or_default(),
                        sha256_result.unwrap_or_default(),
                        sha512_result.unwrap_or_default(),
                    ));
                });
                
                self.file_name = file_name;
                self.file_size = file_size;
                true
            }
            Msg::ComputeStarted => {
                self.is_computing = true;
                self.progress = 0.0;
                true
            }
            Msg::ProgressUpdate(step, progress) => {
                self.step = step;
                self.progress = progress;
                true
            }
            Msg::HashesComputed(md5, sha1, sha256, sha512) => {
                self.hash_md5 = md5;
                self.hash_sha1 = sha1;
                self.hash_sha256 = sha256;
                self.hash_sha512 = sha512;
                self.is_computing = false;
                self.progress = 1.0;
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
            Msg::Toggle(key) => {
                if let Some(value) = self.selected.get_mut(&key) {
                    *value = !*value;
                }
                false // 상태가 변경되었으므로 리렌더링
            }
            Msg::NoOp => false,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let hashes = vec![
            ("md5", "MD5", &self.hash_md5),
            ("sha1", "SHA-1", &self.hash_sha1),
            ("sha256", "SHA-256", &self.hash_sha256),
            ("sha512", "SHA-512", &self.hash_sha512),
        ];

        html! {
            <>
                        <h1 class="tool-title">
                            { "File Hash Generator" }
                        </h1>
                <div class="tool-wrapper">
                        <div class="tool-intro">
                        <div class="content-section">
                            <h2>{"🔤 What is a File Hash?"}</h2>
                            <p>{"A file hash is a unique, fixed-size string generated by applying a cryptographic hash function (such as MD5, SHA-1, SHA-256, or SHA-512) to the contents of a file. Hashes are used to verify file integrity, detect tampering, and uniquely identify files."}</p>
                            <p>{"Hash functions are designed to be fast, deterministic, and collision-resistant, making them ideal for security and verification tasks."}</p>
                        </div>

                        <div class="content-section">
                            <h2>{"⚙️ How This File Hash Generator Works"}</h2>
                            <p>{"This tool computes cryptographic hash values for any file you select. It supports multiple algorithms and processes files locally in your browser for privacy and speed."}</p>
                            <h3>{"Supported Features:"}</h3>
                            <ul>
                                <li><strong>{"Multiple Algorithms:"}</strong> {"MD5, SHA-1, SHA-256, SHA-512 supported."}</li>
                                <li><strong>{"Real-time Progress:"}</strong> {"See progress for large files as they are processed in chunks."}</li>
                                <li><strong>{"Selective Hashing:"}</strong> {"Choose which hash algorithms to compute."}</li>
                                <li><strong>{"Copy with Notification:"}</strong> {"Click any output field to copy results with visual feedback."}</li>
                                <li><strong>{"Local Processing:"}</strong> {"No data is uploaded—everything runs in your browser."}</li>
                            </ul>
                            <h3>{"Input Example:"}</h3>
                            <div class="example-box">
                                <p><strong>{"File input example:"}</strong></p>
                                <ul>
                                    <li>{"Select any file from your device (e.g., image.jpg, document.pdf, archive.zip)"}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"💡 Common Use Cases"}</h2>
                            <div class="use-case">
                                <h3>{"1. File Integrity Verification"}</h3>
                                <ul>
                                    <li><strong>{"Download Verification:"}</strong> {"Compare the hash of a downloaded file with the published hash to ensure it has not been tampered with."}</li>
                                    <li><strong>{"Backup Validation:"}</strong> {"Check that backup files are identical to the originals by comparing hashes."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"2. Security & Forensics"}</h3>
                                <ul>
                                    <li><strong>{"Malware Detection:"}</strong> {"Identify known malware by comparing file hashes to threat databases."}</li>
                                    <li><strong>{"Digital Evidence:"}</strong> {"Prove file authenticity in digital investigations."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"3. Data Management"}</h3>
                                <ul>
                                    <li><strong>{"Deduplication:"}</strong> {"Detect duplicate files by comparing their hashes."}</li>
                                    <li><strong>{"Unique File IDs:"}</strong> {"Generate unique identifiers for files in databases or systems."}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"📚 Step-by-Step Tutorial"}</h2>
                            <div class="tutorial-step">
                                <h3>{"Example: Verifying a Downloaded File"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Verify the integrity of a downloaded file using SHA-256."}</p>
                                <ol>
                                    <li>{"Select the file you downloaded (e.g., installer.exe)."}</li>
                                    <li>{"Ensure 'SHA-256' is checked in the algorithm list."}</li>
                                    <li>{"Wait for the hash to be computed (progress bar will show for large files)."}</li>
                                    <li>{"Compare the computed SHA-256 hash with the one provided by the publisher."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"File:"}</strong> {"installer.exe"}</p>
                                    <p><strong>{"SHA-256 Output:"}</strong> {"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"}</p>
                                </div>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🔧 Technical Background"}</h2>
                            <h3>{"How File Hashing Works"}</h3>
                            <p>{"A hash function takes the contents of a file and produces a fixed-size string (the hash). Even a tiny change in the file will result in a completely different hash. Common algorithms include MD5 (128 bits), SHA-1 (160 bits), SHA-256 (256 bits), and SHA-512 (512 bits)."}</p>
                            <div class="example-box">
                                <p><strong>{"Example for 'hello.txt':"}</strong></p>
                                <ul>
                                    <li>{"MD5: 5d41402abc4b2a76b9719d911017c592"}</li>
                                    <li>{"SHA-1: aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"}</li>
                                    <li>{"SHA-256: 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"}</li>
                                </ul>
                            </div>
                            <h3>{"Why Use File Hashes?"}</h3>
                            <ul>
                                <li>{"Detects accidental or malicious file changes."}</li>
                                <li>{"Enables secure file verification and authentication."}</li>
                                <li>{"Widely supported in security, backup, and data management tools."}</li>
                            </ul>
                            <h3>{"Performance & Implementation"}</h3>
                            <ul>
                                <li><strong>{"Chunked Processing:"}</strong> {"Large files are processed in chunks to avoid freezing the browser."}</li>
                                <li><strong>{"Efficient Algorithms:"}</strong> {"Optimized for speed and low memory usage."}</li>
                                <li><strong>{"Local Only:"}</strong> {"No file data ever leaves your device."}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"❓ Frequently Asked Questions"}</h2>
                            <div class="faq-item">
                                <h3>{"Q: What is the difference between MD5, SHA-1, SHA-256, and SHA-512?"}</h3>
                                <p>{"A: They are different cryptographic hash algorithms with varying output lengths and security levels. SHA-256 and SHA-512 are more secure than MD5 and SHA-1."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Can I hash very large files?"}</h3>
                                <p>{"A: Yes, this tool processes files in chunks and shows progress for large files. Performance depends on your device."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Is file hashing secure?"}</h3>
                                <p>{"A: Hashing is secure for integrity verification, but not for encryption. Do not use hashes alone for storing sensitive data."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: Why are there checkboxes for each algorithm?"}</h3>
                                <p>{"A: You can select which hash algorithms to compute, saving time and resources if you only need one or two."}</p>
                            </div>
                            <div class="faq-item">
                                <h3>{"Q: What if I select a huge file?"}</h3>
                                <p>{"A: The tool will process it, but it may take longer and use more memory. Progress is shown in real time."}</p>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🎯 Best Practices"}</h2>
                            <ul>
                                <li><strong>{"Validate Hashes:"}</strong> {"Always compare computed hashes with trusted sources."}</li>
                                <li><strong>{"Error Handling:"}</strong> {"Handle large file errors gracefully in your applications."}</li>
                                <li><strong>{"Performance:"}</strong> {"Use chunked processing for large files to avoid UI freezing."}</li>
                                <li><strong>{"Documentation:"}</strong> {"Document which hash algorithms are used and why."}</li>
                                <li><strong>{"Testing:"}</strong> {"Test with files of various sizes and types."}</li>
                                <li><strong>{"Security Awareness:"}</strong> {"Remember that hashes are not encryption—do not use for storing secrets."}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"🔗 Related Tools"}</h2>
                            <p>{"Enhance your workflow with this related tool:"}</p>
                            <ul>
                                <li><a href="/crc/">{"CRC tool"}</a> {" - For calculating CRC checksums for data integrity verification."}</li>
                            </ul>
                        </div>
                    </div>
                    <div class="tool-container">
                        <div class="tool-inner" style="width: 100%; margin-bottom: 10px;">
                            <div>
                                <div style="display: flex; flex-wrap: wrap; justify-content: space-between; align-items: center;">
                                    <div class="tool-subtitle" style="margin-bottom: 5px; width: 100%;">{ "Choose Hash Algorithms" }</div>
                                    <div style="display: flex; flex-wrap: wrap; gap: 20px; align-items: center; justify-content: right;">
                                        { for self.selected.iter().map(|(key, &checked)| {
                                            let key_clone = key.clone();
                                            let id = format!("checkbox-{}", key); // 고유 ID 생성
                                            html! {
                                                <div style="display: flex; align-items: center; gap: 5px;">
                                                    <input
                                                        type="checkbox"
                                                        id={id.clone()} // ID 적용
                                                        checked={checked}
                                                        onclick={_ctx.link().callback(move |_| Msg::Toggle(key_clone.clone()))}
                                                    />
                                                    <label for={id.clone()} style="cursor: pointer; margin-bottom: 0px;">{ key.clone() }</label> // 라벨 클릭 가능
                                                </div>
                                            }
                                        })}
                                    </div>
                                </div>
                                <div class="input-div" style="display: grid; grid-template-columns: 2.6fr 1fr; gap: 5px;">
                                    <input id="file-input" type="file" style="display: none;"
                                        onchange={_ctx.link().callback(|e: Event| {
                                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                            if let Some(files) = input.files() {
                                                if let Some(file) = files.get(0) {
                                                    let file_clone = file.clone(); // 파일 복사
                                                    input.set_value(""); // ✅ 같은 파일을 다시 선택할 수 있도록 초기화
                                                    return Msg::FileSelected(file_clone);
                                                }
                                            };
                                            Msg::NoOp
                                        })} />
                                    <span style="display: flex; align-items: center;">{ &self.file_name }</span>
                                    <button
                                        class="tool-btn"
                                        disabled={self.is_computing}
                                        onclick={_ctx.link().callback(|_| {
                                            let document = web_sys::window().unwrap().document().unwrap();
                                            if let Some(input) = document.get_element_by_id("file-input") {
                                                input.dyn_ref::<web_sys::HtmlInputElement>().unwrap().click();
                                            };
                                            Msg::NoOp
                                        })}>
                                        { if self.is_computing { "Computing..." } else { "Select" } }
                                    </button>
                                </div>
                            </div>
                        </div>
                        if self.is_computing && self.progress >= 0.0 {
                            <div style="width: 100%; margin-bottom: 10px;">
                                <div style="width: 100%; background-color: var(--color-third); border-radius: 4px; height: 20px; overflow: hidden;">
                                    <div
                                        style={format!("width: {}%; background-color: var(--color-fourth); height: 20px; border-radius: 4px;",
                                            (self.progress * 100.0).max(0.0).min(100.0))}
                                    >
                                    </div>
                                </div>
                                <div style="text-align: center; margin-top: 5px;">
                                    if self.step {
                                        { format!("Processing: {:.1}%", self.progress * 100.0) }
                                    } else {
                                        { format!("Loading...") }
                                    }
                                </div>
                            </div>
                        } else if self.progress == 1.0 {
                            <div class="tool-inner" style="width: 100%;">
                                <div>
                                    <div class="tool-subtitle" style="margin-bottom: 5px;">{ "File Size" } </div>
                                    <input
                                        type="text"
                                        readonly=true
                                        style="cursor: pointer;"
                                        value={self.file_size.clone()}
                                        onclick={_ctx.link().callback(|e: MouseEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::CopyToClipboard(input.value())
                                        })}
                                    />
                                </div>
                            {
                                for hashes.iter().filter(|(key, _, _)| *self.selected.get(*key).unwrap_or(&false)).map(|(key, label, value)| html! {
                                    <div>
                                        <div class="tool-subtitle" style="margin-bottom: 5px; margin-top: 10px;">{ *label } </div>
                                        <input
                                            type="text"
                                            readonly=true
                                            style="cursor: pointer;"
                                            value={format!("{:}", value.clone())}
                                            onclick={_ctx.link().callback(|e: MouseEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                Msg::CopyToClipboard(input.value())
                                            })}
                                        />
                                    </div>
                                })
                            }
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
                    doc.set_title("File Hash Generator | CompuTools");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "This tool allows you to calculate cryptographic hash values for files, ensuring data integrity and security. It supports multiple hash algorithms commonly used for verification and authentication purposes. Compute hash values for any file using MD5, SHA-1, SHA-256, and SHA-512. Simply select a file, and the tool will compute its hash values efficiently.").unwrap();
                    }
                }
            }
        }
    }
}

async fn read_slice_as_array_buffer(slice: &Blob) -> Result<Vec<u8>, JsValue> {
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let reader = WebFileReader::new().unwrap();
        let onload = Closure::once(Box::new(move |event: ProgressEvent| {
            let reader: WebFileReader = event.target().unwrap().dyn_into().unwrap();
            let array_buffer = reader.result().unwrap();
            // Just pass the ArrayBuffer directly to resolve
            let _ = resolve.call1(&JsValue::NULL, &array_buffer);
        }) as Box<dyn FnOnce(ProgressEvent)>);

        let onerror = Closure::once(Box::new(move |event: ProgressEvent| {
            let reader: WebFileReader = event.target().unwrap().dyn_into().unwrap();
            let error = reader.error().unwrap();
            let _ = reject.call1(&JsValue::NULL, &error);
        }) as Box<dyn FnOnce(ProgressEvent)>);

        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        reader.read_as_array_buffer(slice).unwrap();
        onload.forget();
        onerror.forget();
    });

    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    let uint8_array = js_sys::Uint8Array::new(&result);
    let rust_array = uint8_array.to_vec();

    Ok(rust_array)
}
