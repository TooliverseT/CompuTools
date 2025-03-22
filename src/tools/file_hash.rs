use yew::prelude::*;
use web_sys::{File, window, HtmlInputElement};
use gloo_file::futures::read_as_bytes;
use gloo_file::File as GlooFile;
use sha2::{Digest as Sha2Digest, Sha256, Sha512};
use sha1::{Sha1, Digest as Sha1Digest};
use md5::{Md5, Digest as Md5Digest};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use wasm_bindgen::JsCast;
use std::rc::Rc;
use gloo_timers::future::TimeoutFuture;
use indexmap::IndexMap;

// 청크 처리를 위한 상수 - 성능 향상을 위해 청크 크기 증가
const CHUNK_SIZE: usize = 16 * 1024 * 1024;
const PROGRESS_UPDATE_INTERVAL: usize = 1;
const UI_UPDATE_DELAY_MS: u32 = 10;

pub struct ToolFileHash {
    file: Option<File>,
    file_name: String,
    file_size: String,
    hash_md5: String,
    hash_sha1: String,
    hash_sha256: String,
    hash_sha512: String,
    is_computing: bool,
    progress: f64,
    selected: IndexMap<String, bool>,
}

pub enum Msg {
    FileSelected(File),
    HashesComputed(String, String, String, String),
    CopyToClipboard(String),
    ComputeStarted,
    ProgressUpdate(f64),
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
            file: None,
            file_name: "No file selected".to_string(),
            file_size: "".to_string(),
            hash_md5: "".to_string(),
            hash_sha1: "".to_string(),
            hash_sha256: "".to_string(),
            hash_sha512: "".to_string(),
            is_computing: false,
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
                let file_reader = GlooFile::from(file.clone());
                
                // 계산 시작 상태로 변경
                link.send_message(Msg::ComputeStarted);
                
                // 선택된 해시 알고리즘 확인
                let selected = self.selected.clone();
                
                // 해시 계산을 청크 단위로 수행하여 UI 블로킹 방지
                spawn_local(async move {
                    if let Ok(bytes) = read_as_bytes(&file_reader).await {
                        let total_size = bytes.len();
                        let chunks = (total_size + CHUNK_SIZE - 1) / CHUNK_SIZE; // 올림 계산
                        
                        let mut md5_hasher = selected.get("md5").copied().unwrap_or(false).then(Md5::new);
                        let mut sha1_hasher = selected.get("sha1").copied().unwrap_or(false).then(Sha1::new);
                        let mut sha256_hasher = selected.get("sha256").copied().unwrap_or(false).then(Sha256::new);
                        let mut sha512_hasher = selected.get("sha512").copied().unwrap_or(false).then(Sha512::new);
                        
                        // 청크 단위로 처리
                        for (i, chunk) in bytes.chunks(CHUNK_SIZE).enumerate() {
                            if let Some(h) = &mut md5_hasher { h.update(chunk); }
                            if let Some(h) = &mut sha1_hasher { h.update(chunk); }
                            if let Some(h) = &mut sha256_hasher { h.update(chunk); }
                            if let Some(h) = &mut sha512_hasher { h.update(chunk); }
                            
                            // 진행 상황 업데이트 - 특정 간격으로만 업데이트하여 성능 향상
                            if i % PROGRESS_UPDATE_INTERVAL == 0 || i == chunks - 1 {
                                let progress = (i + 1) as f64 / chunks as f64;
                                link.send_message(Msg::ProgressUpdate(progress));
                                
                                // UI 업데이트를 위한 짧은 지연
                                // 이 지연은 렌더링 스레드가 진행률을 업데이트할 수 있게 함
                                TimeoutFuture::new(UI_UPDATE_DELAY_MS).await;
                            }
                        }
                        
                        // 최종 해시 값 계산
                        let md5_result = md5_hasher.map(|h| format!("{:x}", h.finalize()));
                        let sha1_result = sha1_hasher.map(|h| format!("{:x}", h.finalize()));
                        let sha256_result = sha256_hasher.map(|h| format!("{:x}", h.finalize()));
                        let sha512_result = sha512_hasher.map(|h| format!("{:x}", h.finalize()));

                        // None 값을 빈 문자열로 변환하여 Msg 전송
                        link.send_message(Msg::HashesComputed(
                            md5_result.unwrap_or_default(),
                            sha1_result.unwrap_or_default(),
                            sha256_result.unwrap_or_default(),
                            sha512_result.unwrap_or_default(),
                        ));
                    }
                });
                
                self.file = Some(file);
                self.file_name = file_name;
                self.file_size = file_size;
                true
            }
            Msg::ComputeStarted => {
                self.is_computing = true;
                self.progress = 0.0;
                true
            }
            Msg::ProgressUpdate(progress) => {
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
            Msg::NoOp => {
                false
            }
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
                <div class="tool-wrapper ver2">
                    <div>
                        <h1 class="tool-title">
                            { "File Hash Generator" }
                        </h1>
                        <div class="tool-intro">
                            <p>
                                {"This tool allows you to calculate cryptographic hash values for files, ensuring data integrity and security. It supports multiple hash algorithms commonly used for verification and authentication purposes."}
                            </p>
                            <p> {"With this tool, you can:"} </p>
                            <ul>
                                <li>{"Compute hash values for any file using MD5, SHA-1, SHA-256, and SHA-512."}</li>
                                <li>{"Verify file integrity by comparing calculated hashes with expected values."}</li>
                                <li>{"Monitor hashing progress in real-time for large files."}</li>
                            </ul>
                            <p>
                                {"The tool processes files locally on your device, ensuring privacy and security by avoiding any data uploads."}
                            </p>
                            <p>
                                {"Use cases include:"}
                            </p>
                            <ul>
                                <li>{"Checking file integrity after downloads or transfers."}</li>
                                <li>{"Generating unique file identifiers for verification."}</li>
                                <li>{"Enhancing security through cryptographic hashing."}</li>
                            </ul>
                            <p>
                                <strong>{"Note:"}</strong> 
                                {" Since the hashing process runs entirely on your device, performance may vary depending on file size and system resources. Larger files may take longer to process, and high CPU usage can temporarily slow down other tasks."}
                            </p>
                            <p>
                                {"Simply select a file, and the tool will compute its hash values efficiently."}
                            </p>
                        </div>
                    </div>
                    <div class="tool-container ver3" style="flex-direction: column;">
                        <div class="tool-inner" style="width: 100%; margin-bottom: 10px;">
                            <div>
                                <div style="display: flex; flex-wrap: wrap; justify-content: space-between; align-items: center;">
                                    <div class="tool-subtitle" style="margin-bottom: 5px;">{ "Choose File" }</div>
                                    <div style="display: flex; flex-wrap: wrap; gap: 40px; align-items: center; justify-content: right;">
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
                                        { if self.is_computing { "Computing..." } else { "Choose File" } }
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
                                    { format!("Processing: {:.1}%", self.progress * 100.0) }
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