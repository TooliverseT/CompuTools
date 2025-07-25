use gloo_timers::future::TimeoutFuture;
use indexmap::IndexMap;
use md5::{Digest as Md5Digest, Md5};
use sha1::{Digest as Sha1Digest, Sha1};
use sha2::{Digest as Sha2Digest, Sha256, Sha512};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::Blob;
use web_sys::{window, HtmlInputElement, DragEvent, Storage};
use web_sys::{File, FileReader as WebFileReader, ProgressEvent};
use yew::prelude::*;
use crate::components::tool_category::ToolCategoryManager;
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use hex;
use hmac::{Hmac, Mac};
use crc::{Crc, CRC_32_ISO_HDLC};

// 청크 처리를 위한 상수 - 성능 향상을 위해 청크 크기 증가
const CHUNK_SIZE: usize = 16 * 1024 * 1024;
const PROGRESS_UPDATE_INTERVAL: u32 = 1;
const UI_UPDATE_DELAY_MS: u32 = 10;
const PROGRESS_UPDATE_RETURN: u32 = 20;
const UPDATE_DELAY_MS: u32 = 5;

// Local Storage 키 상수들
const STORAGE_KEY_HASH_ALGORITHMS: &str = "file_hash_algorithms";
const STORAGE_KEY_OUTPUT_FORMAT: &str = "file_hash_output_format";
const STORAGE_KEY_HMAC_ENABLED: &str = "file_hash_hmac_enabled";

// 파일 크기 제한 및 검증을 위한 상수들
const MAX_FILE_SIZE: usize = 2 * 1024 * 1024 * 1024; // 2GB 제한
const ALLOWED_MIME_TYPES: &[&str] = &[
    // 일반적으로 안전한 파일 타입들
    "application/octet-stream",
    "application/pdf",
    "application/zip",
    "application/x-zip-compressed",
    "application/x-rar-compressed",
    "application/x-7z-compressed",
    "application/gzip",
    "application/x-tar",
    "application/json",
    "application/xml",
    "application/msword",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "application/vnd.ms-excel",
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "application/vnd.ms-powerpoint",
    "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    "text/plain",
    "text/csv",
    "text/html",
    "text/css",
    "text/javascript",
    "text/xml",
    "image/jpeg",
    "image/png",
    "image/gif",
    "image/bmp",
    "image/webp",
    "image/svg+xml",
    "image/tiff",
    "audio/mpeg",
    "audio/wav",
    "audio/ogg",
    "audio/mp4",
    "video/mp4",
    "video/mpeg",
    "video/quicktime",
    "video/x-msvideo",
    "video/webm",
];

// 출력 포맷 옵션
#[derive(Clone, PartialEq)]
pub enum OutputFormat {
    Lowercase,     // d85e5d59b4d49efce3398dc6b6d4b91b
    Uppercase,     // D85E5D59B4D49EFCE3398DC6B6D4B91B
    ColonSeparated, // d8:5e:5d:59:b4:d4:9e:fc:e3:39:8d:c6:b6:d4:b9:1b
    Base64,        // 2F5dWbTUnvzjOY3GttS5Gw==
    CStyleArray,   // {0xd8, 0x5e, 0x5d, 0x59, 0xb4, 0xd4, 0x9e, 0xfc, 0xe3, 0x39, 0x8d, 0xc6, 0xb6, 0xd4, 0xb9, 0x1b}
}

// 검증 타입 옵션
#[derive(Clone, PartialEq)]
pub enum VerificationType {
    Hash,  // 일반 해시 검증
    Hmac,  // HMAC 검증
}

pub struct ToolFileHash {
    file_info: Option<FileInfo>, // 파일 정보
    file_data: Option<Vec<u8>>, // 파일 데이터 저장
    hash_md5: String,
    hash_sha1: String,
    hash_sha256: String,
    hash_sha512: String,
    hmac_md5: String,
    hmac_sha1: String,
    hmac_sha256: String,
    hmac_sha512: String,
    crc32: String,
    is_computing: bool,
    step: bool,
    progress: f64,
    selected: IndexMap<String, bool>,
    error_message: Option<String>, // 에러 메시지 추가
    is_dragging: bool, // 드래그 상태
    expected_hash: String, // 예상 해시값
    hash_comparison: Option<HashComparison>, // 해시 비교 결과
    show_hash_verification: bool, // 해시 검증 섹션 표시 여부
    output_format: OutputFormat, // 출력 포맷
    show_file_metadata: bool, // 파일 메타데이터 표시 여부
    hmac_key: String, // HMAC 키
    show_hmac_section: bool, // HMAC 섹션 표시 여부
    verification_type: VerificationType, // 검증 타입 (일반 해시 vs HMAC)
    verification_hmac_key: String, // 검증용 HMAC 키
}

// 파일 정보를 저장하는 구조체
#[derive(Clone)]
pub struct FileInfo {
    pub name: String,
    pub size: usize,
    pub mime_type: String,
    pub last_modified: Option<f64>,
}

// 해시 비교 결과
#[derive(Clone, PartialEq)]
pub struct HashComparison {
    pub algorithm: String,
    pub matches: bool,
    pub expected: String,
    pub actual: String,
}

pub enum Msg {
    FileSelected(File),
    HashesComputed(String, String, String, String, String, String, String, String, String), // CRC32와 HMAC들 추가
    CopyToClipboard(String),
    ComputeStarted,
    ProgressUpdate(bool, f64),
    Toggle(String),
    FileValidationError(String), // 파일 검증 에러 메시지
    DragEnter,
    DragLeave,
    DragOver,
    Drop(File),
    ExpectedHashChanged(String),
    ToggleHashVerification,
    ClearError,
    ClearFile, // 파일 제거
    OutputFormatChanged(OutputFormat), // 출력 포맷 변경
    ToggleFileMetadata, // 파일 메타데이터 토글
    HmacKeyChanged(String), // HMAC 키 변경
    ToggleHmacSection, // HMAC 섹션 토글
    FileDataLoaded(Vec<u8>), // 파일 데이터 로드 완료 메시지
    VerificationTypeChanged(VerificationType), // 검증 타입 변경
    VerificationHmacKeyChanged(String), // 검증용 HMAC 키 변경
    NoOp,
}

impl Component for ToolFileHash {
    type Message = Msg;
    type Properties = (); // No props needed

    fn create(_ctx: &Context<Self>) -> Self {
        Self::load_from_storage()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FileSelected(file) => {
                // 파일 검증
                if let Err(error) = self.validate_file(&file) {
                    self.error_message = Some(error);
                    return true;
                }

                self.error_message = None;
                self.hash_comparison = None; // 기존 비교 결과 초기화
                
                let file_size_bytes = file.size() as usize;
                let file_name = file.name();
                let mime_type = file.type_();
                let last_modified = file.last_modified();
                
                // 파일 정보 저장
                self.file_info = Some(FileInfo {
                    name: file_name.clone(),
                    size: file_size_bytes,
                    mime_type: mime_type.clone(),
                    last_modified: Some(last_modified),
                });
                
                let link = _ctx.link().clone();
                
                // 계산 시작 상태로 변경
                link.send_message(Msg::ComputeStarted);
                
                // 선택된 해시 알고리즘 확인
                let selected = self.selected.clone();
                let hmac_enabled = self.show_hmac_section;
                let hmac_key = self.hmac_key.clone();
                
                // 해시 계산을 청크 단위로 수행하여 UI 블로킹 방지
                spawn_local(async move {
                    // 전체 파일 데이터를 먼저 읽어서 저장 (작은 파일용)
                    let file_data = match read_slice_as_array_buffer(&file).await {
                        Ok(data) => data,
                        Err(_) => {
                            link.send_message(Msg::HashesComputed(
                                String::new(),
                                String::new(),
                                String::new(),
                                String::new(),
                                String::new(),
                                String::new(),
                                String::new(),
                                String::new(),
                                String::new()
                            ));
                            return;
                        }
                    };
                    
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
                    
                    // CRC32 계산
                    let mut crc32_result = String::new();
                    if selected.get("crc32").copied().unwrap_or(false) {
                        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
                        let checksum = crc.checksum(&file_data);
                        crc32_result = format!("{:08x}", checksum);
                    }
                    
                    // HMAC 계산 (옵션이 활성화되어 있고 키가 있는 경우)
                    let mut hmac_md5_result = String::new();
                    let mut hmac_sha1_result = String::new();
                    let mut hmac_sha256_result = String::new();
                    let mut hmac_sha512_result = String::new();
                    
                    if hmac_enabled && !hmac_key.trim().is_empty() {
                        let hmac_key_bytes = hmac_key.as_bytes();
                        
                        // 선택된 알고리즘에 대해서만 HMAC 계산
                        if selected.get("md5").copied().unwrap_or(false) {
                            if let Ok(calculated_hmac) = Self::calculate_hmac_md5(hmac_key_bytes, &file_data) {
                                hmac_md5_result = calculated_hmac;
                            }
                        }
                        
                        if selected.get("sha1").copied().unwrap_or(false) {
                            if let Ok(calculated_hmac) = Self::calculate_hmac_sha1(hmac_key_bytes, &file_data) {
                                hmac_sha1_result = calculated_hmac;
                            }
                        }
                        
                        if selected.get("sha256").copied().unwrap_or(false) {
                            if let Ok(calculated_hmac) = Self::calculate_hmac_sha256(hmac_key_bytes, &file_data) {
                                hmac_sha256_result = calculated_hmac;
                            }
                        }
                        
                        if selected.get("sha512").copied().unwrap_or(false) {
                            if let Ok(calculated_hmac) = Self::calculate_hmac_sha512(hmac_key_bytes, &file_data) {
                                hmac_sha512_result = calculated_hmac;
                            }
                        }
                    }
                    
                    link.send_message(Msg::ProgressUpdate(false, 0.0));
                    
                    // 해시 계산
                        if let Some(h) = &mut md5_hasher {
                        h.update(&file_data);
                        }
                        if let Some(h) = &mut sha1_hasher {
                        h.update(&file_data);
                        }
                        if let Some(h) = &mut sha256_hasher {
                        h.update(&file_data);
                        }
                        if let Some(h) = &mut sha512_hasher {
                        h.update(&file_data);
                    }
                    
                    // 최종 진행률 업데이트
                    link.send_message(Msg::ProgressUpdate(true, 1.0));
                    
                    // 최종 해시 값 계산
                    let md5_result = md5_hasher.map(|h| format!("{:x}", h.finalize()));
                    let sha1_result = sha1_hasher.map(|h| format!("{:x}", h.finalize()));
                    let sha256_result = sha256_hasher.map(|h| format!("{:x}", h.finalize()));
                    let sha512_result = sha512_hasher.map(|h| format!("{:x}", h.finalize()));
                    
                    // 파일 데이터와 해시 값 전송
                    link.send_message(Msg::FileDataLoaded(file_data));
                    link.send_message(Msg::HashesComputed(
                        md5_result.unwrap_or_default(),
                        sha1_result.unwrap_or_default(),
                        sha256_result.unwrap_or_default(),
                        sha512_result.unwrap_or_default(),
                        hmac_md5_result,
                        hmac_sha1_result,
                        hmac_sha256_result,
                        hmac_sha512_result,
                        crc32_result,
                    ));
                });
                
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
            Msg::HashesComputed(md5, sha1, sha256, sha512, hmac_md5, hmac_sha1, hmac_sha256, hmac_sha512, crc32) => {
                self.hash_md5 = md5;
                self.hash_sha1 = sha1;
                self.hash_sha256 = sha256;
                self.hash_sha512 = sha512;
                self.hmac_md5 = hmac_md5;
                self.hmac_sha1 = hmac_sha1;
                self.hmac_sha256 = hmac_sha256;
                self.hmac_sha512 = hmac_sha512;
                self.crc32 = crc32;
                self.is_computing = false;
                self.progress = 1.0;
                
                // 예상 해시가 있다면 비교 수행
                if !self.expected_hash.trim().is_empty() {
                    self.perform_hash_comparison();
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
            Msg::Toggle(key) => {
                if let Some(value) = self.selected.get_mut(&key) {
                    *value = !*value;
                }
                self.save_to_storage();
                true // 상태가 변경되었으므로 리렌더링
            }
            Msg::FileValidationError(error) => {
                self.error_message = Some(error);
                true
            }
            Msg::DragEnter => {
                self.is_dragging = true;
                true
            }
            Msg::DragLeave => {
                self.is_dragging = false;
                true
            }
            Msg::DragOver => {
                // DragOver는 상태 변경 없이 기본 동작만 방지
                false
            }
            Msg::Drop(file) => {
                self.is_dragging = false;
                // 파일 선택과 동일한 로직 수행
                _ctx.link().send_message(Msg::FileSelected(file));
                true
            }
            Msg::ExpectedHashChanged(hash) => {
                self.expected_hash = hash;
                // 해시 계산이 완료된 상태라면 즉시 비교 수행
                if self.progress == 1.0 && !self.is_computing {
                    self.perform_hash_comparison();
                }
                true
            }
            Msg::ToggleHashVerification => {
                self.show_hash_verification = !self.show_hash_verification;
                if !self.show_hash_verification {
                    self.hash_comparison = None;
                    self.expected_hash.clear();
                }
                true
            }
            Msg::ClearError => {
                self.error_message = None;
                true
            }
            Msg::ClearFile => {
                self.file_info = None;
                self.file_data = None;
                self.hash_md5 = "".to_string();
                self.hash_sha1 = "".to_string();
                self.hash_sha256 = "".to_string();
                self.hash_sha512 = "".to_string();
                self.hmac_md5 = "".to_string();
                self.hmac_sha1 = "".to_string();
                self.hmac_sha256 = "".to_string();
                self.hmac_sha512 = "".to_string();
                self.crc32 = "".to_string();
                self.is_computing = false;
                self.progress = 0.0;
                self.hash_comparison = None;
                self.expected_hash.clear();
                self.verification_hmac_key.clear();
                self.verification_type = VerificationType::Hash;
                self.show_file_metadata = false;
                self.show_hash_verification = false;
                self.error_message = None;
                true
            }
            Msg::OutputFormatChanged(format) => {
                self.output_format = format;
                self.save_to_storage();
                true
            }
            Msg::ToggleFileMetadata => {
                self.show_file_metadata = !self.show_file_metadata;
                true
            }
            Msg::HmacKeyChanged(key) => {
                self.hmac_key = key;
                true
            }
            Msg::ToggleHmacSection => {
                self.show_hmac_section = !self.show_hmac_section;
                self.save_to_storage();
                true
            }
            Msg::FileDataLoaded(data) => {
                self.file_data = Some(data);
                true
            }
            Msg::VerificationTypeChanged(new_type) => {
                self.verification_type = new_type;
                true
            }
            Msg::VerificationHmacKeyChanged(key) => {
                self.verification_hmac_key = key;
                true
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
            ("crc32", "CRC32", &self.crc32),
        ];
        
        let hmac_hashes = vec![
            ("hmac_md5", "HMAC-MD5", &self.hmac_md5),
            ("hmac_sha1", "HMAC-SHA1", &self.hmac_sha1),
            ("hmac_sha256", "HMAC-SHA256", &self.hmac_sha256),
            ("hmac_sha512", "HMAC-SHA512", &self.hmac_sha512),
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
                                <li><strong>{"Multiple Algorithms:"}</strong> {"MD5, SHA-1, SHA-256, SHA-512, and CRC32 supported with selective computation."}</li>
                                <li><strong>{"Real-time Progress:"}</strong> {"See progress for large files as they are processed in chunks."}</li>
                                <li><strong>{"Selective Hashing:"}</strong> {"Choose which hash algorithms to compute to save time and resources."}</li>
                                <li><strong>{"Copy with Notification:"}</strong> {"Click any output field to copy results with visual feedback."}</li>
                                <li><strong>{"Local Processing:"}</strong> {"No data is uploaded—everything runs in your browser."}</li>
                                <li><strong>{"Drag & Drop Support:"}</strong> {"Simply drag files into the drop zone for easy processing."}</li>
                                <li><strong>{"File Size Validation:"}</strong> {"Automatic validation with size limits and security checks."}</li>
                                <li><strong>{"Hash Verification:"}</strong> {"Compare computed hashes with expected values for integrity verification."}</li>
                                <li><strong>{"Multiple Output Formats:"}</strong> {"Choose from lowercase, uppercase, colon-separated, Base64, or C-style array formats."}</li>
                                <li><strong>{"File Metadata Display:"}</strong> {"View detailed file information including size, MIME type, and modification date."}</li>
                                <li><strong>{"HMAC Support:"}</strong> {"Generate Hash-based Message Authentication Codes for secure authentication."}</li>
                                <li><strong>{"CRC32 Checksum:"}</strong> {"Fast error detection commonly used in ZIP files and network protocols."}</li>
                                <li><strong>{"Smart Input Recognition:"}</strong> {"Verification supports hex, colon-separated, Base64, and C-style array input formats."}</li>
                                <li><strong>{"Settings Persistence:"}</strong> {"Your algorithm selections, output format, and HMAC preferences are automatically saved."}</li>
                            </ul>
                            <h3>{"Input Example:"}</h3>
                            <div class="example-box">
                                <p><strong>{"File input example:"}</strong></p>
                                <ul>
                                    <li>{"Select any file from your device (e.g., image.jpg, document.pdf, archive.zip)"}</li>
                                    <li>{"Drag and drop files directly into the drop zone"}</li>
                                    <li>{"Files up to 2GB in size are supported"}</li>
                                    <li>{"Your preferred algorithms and output format will be remembered for next time"}</li>
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
                                    <li><strong>{"Software Distribution:"}</strong> {"Verify the integrity of downloaded software, drivers, or firmware before installation."}</li>
                                    <li><strong>{"Cloud Storage Sync:"}</strong> {"Ensure files uploaded to cloud storage maintain their integrity during transfer."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"2. Security & Forensics"}</h3>
                                <ul>
                                    <li><strong>{"Malware Detection:"}</strong> {"Identify known malware by comparing file hashes to threat databases like VirusTotal."}</li>
                                    <li><strong>{"Digital Evidence:"}</strong> {"Prove file authenticity in digital investigations and legal proceedings."}</li>
                                    <li><strong>{"Incident Response:"}</strong> {"Verify system files haven't been modified during security incidents."}</li>
                                    <li><strong>{"Chain of Custody:"}</strong> {"Maintain evidence integrity throughout investigation processes."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"3. Data Management"}</h3>
                                <ul>
                                    <li><strong>{"Deduplication:"}</strong> {"Detect duplicate files by comparing their hashes across storage systems."}</li>
                                    <li><strong>{"Unique File IDs:"}</strong> {"Generate unique identifiers for files in databases or content management systems."}</li>
                                    <li><strong>{"Version Control:"}</strong> {"Track file changes and identify modifications in document management."}</li>
                                    <li><strong>{"Content Delivery:"}</strong> {"Verify cached content integrity in CDN systems."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"4. Development & DevOps"}</h3>
                                <ul>
                                    <li><strong>{"Build Verification:"}</strong> {"Ensure compiled artifacts match expected checksums in CI/CD pipelines."}</li>
                                    <li><strong>{"Container Images:"}</strong> {"Verify Docker image integrity using SHA-256 layer checksums."}</li>
                                    <li><strong>{"Package Management:"}</strong> {"Validate npm, pip, or Maven packages before installation."}</li>
                                    <li><strong>{"Configuration Management:"}</strong> {"Detect unauthorized changes to critical configuration files."}</li>
                                </ul>
                            </div>
                            <div class="use-case">
                                <h3>{"5. API Authentication & Security"}</h3>
                                <ul>
                                    <li><strong>{"Request Signing:"}</strong> {"Use HMAC to sign API requests and verify sender authenticity."}</li>
                                    <li><strong>{"Webhook Verification:"}</strong> {"Validate webhook payloads from services like GitHub, Stripe, or PayPal."}</li>
                                    <li><strong>{"JWT Token Validation:"}</strong> {"Verify JWT signature integrity using HMAC algorithms."}</li>
                                    <li><strong>{"Password Hashing:"}</strong> {"Generate secure password hashes for storage (though bcrypt is preferred)."}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"📚 Step-by-Step Tutorial"}</h2>
                            <div class="tutorial-step">
                                <h3>{"Example 1: Verifying a Downloaded File"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Verify the integrity of a downloaded file using SHA-256."}</p>
                                <ol>
                                    <li>{"Select the file you downloaded (e.g., installer.exe)."}</li>
                                    <li>{"Ensure 'SHA-256' is checked in the algorithm list."}</li>
                                    <li>{"Wait for the hash to be computed (progress bar will show for large files)."}</li>
                                    <li>{"Compare the computed SHA-256 hash with the one provided by the publisher."}</li>
                                    <li>{"Use the 'Hash Verification' feature to automatically compare values."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"File:"}</strong> {"installer.exe"}</p>
                                    <p><strong>{"SHA-256 Output:"}</strong> {"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"}</p>
                                    <p><strong>{"Publisher Hash:"}</strong> {"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"}</p>
                                    <p style="color: green;"><strong>{"Result:"}</strong> {"✅ Verification passed! File is authentic."}</p>
                                </div>
                            </div>

                            <div class="tutorial-step">
                                <h3>{"Example 2: Working with Different Output Formats"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Generate hash in various formats for different use cases."}</p>
                                <ol>
                                    <li>{"Upload a file and compute its SHA-256 hash."}</li>
                                    <li>{"Try different output formats from the dropdown:"}</li>
                                    <ul>
                                        <li>{"Lowercase: for standard usage and comparison"}</li>
                                        <li>{"Uppercase: for compatibility with some legacy systems"}</li>
                                        <li>{"Colon-separated: for MAC address-like display"}</li>
                                        <li>{"Base64: for embedding in URLs or JSON"}</li>
                                        <li>{"C-style array: for use in programming"}</li>
                                    </ul>
                                    <li>{"Click any output field to copy the result."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Same SHA-256 hash in different formats:"}</strong></p>
                                    <ul>
                                        <li>{"Lowercase: d85e5d59b4d49efce3398dc6b6d4b91b"}</li>
                                        <li>{"Uppercase: D85E5D59B4D49EFCE3398DC6B6D4B91B"}</li>
                                        <li>{"Colon-separated: d8:5e:5d:59:b4:d4:9e:fc:e3:39:8d:c6:b6:d4:b9:1b"}</li>
                                        <li>{"Base64: 2F5dWbTUnvzjOY3GttS5Gw=="}</li>
                                        <li>{"C-style: {0xd8, 0x5e, 0x5d, 0x59, 0xb4, 0xd4, 0x9e, 0xfc}"}</li>
                                    </ul>
                                </div>
                            </div>

                            <div class="tutorial-step">
                                <h3>{"Example 3: HMAC Authentication for API Security"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Generate HMAC for secure API request signing."}</p>
                                <ol>
                                    <li>{"Check 'Enable HMAC Generation' before uploading."}</li>
                                    <li>{"Enter your API secret key (e.g., 'mySecretKey123')."}</li>
                                    <li>{"Select SHA-256 algorithm for the HMAC."}</li>
                                    <li>{"Upload your request payload file or data file."}</li>
                                    <li>{"Copy the HMAC-SHA256 result for your API header."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Example scenario:"}</strong></p>
                                    <p>{"Secret Key: mySecretKey123"}</p>
                                    <p>{"File: request-payload.json"}</p>
                                    <p>{"HMAC-SHA256: a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"}</p>
                                    <p>{"HTTP Header: Authorization: HMAC-SHA256 a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"}</p>
                                </div>
                            </div>

                            <div class="tutorial-step">
                                <h3>{"Example 4: Using Multiple Input Formats for Verification"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Verify a file hash when you have the expected value in different formats."}</p>
                                <ol>
                                    <li>{"Upload your file and compute its hash."}</li>
                                    <li>{"Click 'Show Hash Verification' after computation."}</li>
                                    <li>{"Try entering the expected hash in various formats:"}</li>
                                    <ul>
                                        <li>{"Standard hex: d85e5d59b4d49efce3398dc6b6d4b91b"}</li>
                                        <li>{"Colon-separated: d8:5e:5d:59:b4:d4:9e:fc:e3:39:8d:c6:b6:d4:b9:1b"}</li>
                                        <li>{"Base64: 2F5dWbTUnvzjOY3GttS5Gw=="}</li>
                                        <li>{"C-style: {0xd8, 0x5e, 0x5d, 0x59, 0xb4, 0xd4, 0x9e, 0xfc}"}</li>
                                    </ul>
                                    <li>{"The tool automatically detects the format and performs comparison."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Smart format detection:"}</strong></p>
                                    <p>{"All these inputs represent the same hash and will match:"}</p>
                                    <ul>
                                        <li>{"d85e5d59b4d49efc (hex)"}</li>
                                        <li>{"d8:5e:5d:59:b4:d4:9e:fc (colon-separated)"}</li>
                                        <li>{"2F5dWbTUnvzj (Base64)"}</li>
                                        <li>{"{0xd8, 0x5e, 0x5d, 0x59} (C-style)"}</li>
                                    </ul>
                                </div>
                            </div>

                            <div class="tutorial-step">
                                <h3>{"Example 5: Persistent Settings for Workflow Efficiency"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Customize the tool for your regular workflow."}</p>
                                <ol>
                                    <li>{"Uncheck algorithms you don't need (e.g., disable MD5 and SHA-1 for security work)."}</li>
                                    <li>{"Select your preferred output format (e.g., uppercase for documentation)."}</li>
                                    <li>{"Enable HMAC if you regularly work with authenticated data."}</li>
                                    <li>{"Your settings are automatically saved and will persist across browser sessions."}</li>
                                    <li>{"The tool remembers your preferences for efficient repeated use."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Saved preferences example:"}</strong></p>
                                    <ul>
                                        <li>{"Algorithms: SHA-256, SHA-512 only (security-focused)"}</li>
                                        <li>{"Output format: Uppercase (for reports)"}</li>
                                        <li>{"HMAC: Enabled (for API work)"}</li>
                                        <li>{"Next session: All preferences automatically restored"}</li>
                                    </ul>
                                </div>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🔧 Technical Background"}</h2>
                            
                            <h3>{"Algorithm Comparison & Selection Guide"}</h3>
                            <div class="example-box">
                                <table style="width: 100%; border-collapse: collapse; margin: 10px 0;">
                                    <thead>
                                        <tr style="background-color: var(--color-fourth); color: white;">
                                            <th style="padding: 8px; border: 1px solid #ddd;">{"Algorithm"}</th>
                                            <th style="padding: 8px; border: 1px solid #ddd;">{"Hash Size"}</th>
                                            <th style="padding: 8px; border: 1px solid #ddd;">{"Security"}</th>
                                            <th style="padding: 8px; border: 1px solid #ddd;">{"Speed"}</th>
                                            <th style="padding: 8px; border: 1px solid #ddd;">{"Use Case"}</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td style="padding: 8px; border: 1px solid #ddd; font-weight: bold;">{"MD5"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd;">{"128-bit (32 hex)"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd; color: var(--color-error);">{"Broken"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd; color: green;">{"Very Fast"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd;">{"Legacy, checksums only"}</td>
                                        </tr>
                                        <tr>
                                            <td style="padding: 8px; border: 1px solid #ddd; font-weight: bold;">{"SHA-1"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd;">{"160-bit (40 hex)"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd; color: orange;">{"Deprecated"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd; color: green;">{"Fast"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd;">{"Git, legacy systems"}</td>
                                        </tr>
                                        <tr>
                                            <td style="padding: 8px; border: 1px solid #ddd; font-weight: bold;">{"SHA-256"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd;">{"256-bit (64 hex)"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd; color: green;">{"Secure"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd; color: green;">{"Fast"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd;">{"Bitcoin, TLS, general use"}</td>
                                        </tr>
                                        <tr>
                                            <td style="padding: 8px; border: 1px solid #ddd; font-weight: bold;">{"SHA-512"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd;">{"512-bit (128 hex)"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd; color: green;">{"Very Secure"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd; color: orange;">{"Moderate"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd;">{"High security, certificates"}</td>
                                        </tr>
                                        <tr>
                                            <td style="padding: 8px; border: 1px solid #ddd; font-weight: bold;">{"CRC32"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd;">{"32-bit (8 hex)"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd; color: var(--color-error);">{"None"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd; color: green;">{"Fastest"}</td>
                                            <td style="padding: 8px; border: 1px solid #ddd;">{"Error detection, ZIP files"}</td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>
                            
                            <h3>{"How File Hashing Works"}</h3>
                            <p>
                                {"A hash function takes the contents of a file and produces a fixed-size string (the hash). Even a tiny change in the file will result in a completely different hash. Common algorithms include MD5 (128 bits), SHA-1 (160 bits), SHA-256 (256 bits), and SHA-512 (512 bits)."}
                            </p>
                            <div class="example-box">
                                <p><strong>{"Example for 'hello.txt':"}</strong></p>
                                <ul>
                                    <li>{"MD5: 5d41402abc4b2a76b9719d911017c592"}</li>
                                    <li>{"SHA-1: aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"}</li>
                                    <li>{"SHA-256: 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"}</li>
                                    <li>{"CRC32: 0x3610a686"}</li>
                                    <li>{"HMAC-SHA256 (key='secret'): a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"}</li>
                                </ul>
                            </div>
                            
                            <h3>{"Understanding Different Algorithms"}</h3>
                            <ul>
                                <li><strong>{"MD5 & SHA-1:"}</strong> {"Legacy algorithms, fast but not cryptographically secure. Use only for non-security purposes like file deduplication."}</li>
                                <li><strong>{"SHA-256 & SHA-512:"}</strong> {"Modern, secure algorithms recommended for security-sensitive applications."}</li>
                                <li><strong>{"CRC32:"}</strong> {"Fast checksum algorithm designed for error detection, not security. Commonly used in ZIP files and network protocols."}</li>
                                <li><strong>{"HMAC:"}</strong> {"Hash-based Message Authentication Code using a secret key. Provides both integrity and authenticity verification."}</li>
                            </ul>
                            
                            <h3>{"Output Format Applications"}</h3>
                            <div class="example-box">
                                <ul>
                                    <li><strong>{"Lowercase Hex:"}</strong> {"Standard format, most widely compatible"}</li>
                                    <li><strong>{"Uppercase Hex:"}</strong> {"Used in documentation, certificates, some legacy systems"}</li>
                                    <li><strong>{"Colon-separated:"}</strong> {"Network administration, forensics, readable format"}</li>
                                    <li><strong>{"Base64:"}</strong> {"URLs, JSON APIs, email attachments, web applications"}</li>
                                    <li><strong>{"C-style Array:"}</strong> {"Embedded systems, firmware, C/C++ source code"}</li>
                                </ul>
                            </div>
                            
                            <h3>{"HMAC (Hash-based Message Authentication Code)"}</h3>
                            <p>
                                {"HMAC combines a cryptographic hash function with a secret key to provide both data integrity and authenticity. Unlike regular hashes, HMAC requires a key, making it impossible for attackers to forge valid signatures without the key."}
                            </p>
                            <div class="example-box">
                                <p><strong>{"HMAC Use Cases:"}</strong></p>
                                <ul>
                                    <li>{"API authentication and request signing"}</li>
                                    <li>{"JWT token verification"}</li>
                                    <li>{"Message authentication in secure protocols"}</li>
                                    <li>{"File integrity with tamper detection"}</li>
                                    <li>{"Digital signatures in blockchain applications"}</li>
                                    <li>{"Webhook payload verification (GitHub, Stripe, PayPal)"}</li>
                                    <li>{"Password-based key derivation"}</li>
                                    <li>{"Secure session management"}</li>
                                </ul>
                            </div>
                            
                            <h3>{"CRC32 (Cyclic Redundancy Check)"}</h3>
                            <p>
                                {"CRC32 is a fast, 32-bit checksum algorithm primarily designed for error detection in data transmission and storage. It's excellent at detecting common transmission errors but is not suitable for security purposes."}
                            </p>
                            <div class="example-box">
                                <p><strong>{"CRC32 Applications:"}</strong></p>
                                <ul>
                                    <li>{"ZIP and archive file integrity checking"}</li>
                                    <li>{"Network packet error detection (Ethernet, TCP)"}</li>
                                    <li>{"Database corruption detection"}</li>
                                    <li>{"File transfer verification"}</li>
                                    <li>{"Storage device error detection"}</li>
                                    <li>{"PNG image chunk validation"}</li>
                                    <li>{"Git object integrity (along with SHA-1)"}</li>
                                </ul>
                            </div>
                            
                            <h3>{"Browser Storage & Privacy"}</h3>
                            <p>
                                {"This tool uses browser Local Storage to save your preferences (selected algorithms, output format, HMAC settings). This data stays on your device and never leaves your browser. You can clear this data anytime through your browser's storage management."}
                            </p>
                            <div class="example-box">
                                <p><strong>{"Stored preferences:"}</strong></p>
                                <ul>
                                    <li>{"Selected hash algorithms (MD5, SHA-1, SHA-256, etc.)"}</li>
                                    <li>{"Preferred output format (lowercase, uppercase, etc.)"}</li>
                                    <li>{"HMAC generation toggle state"}</li>
                                    <li>{"Note: Secret keys and file data are never stored"}</li>
                                </ul>
                            </div>
                            
                            <h3>{"Why Use File Hashes?"}</h3>
                            <ul>
                                <li>{"Detects accidental or malicious file changes."}</li>
                                <li>{"Enables secure file verification and authentication."}</li>
                                <li>{"Widely supported in security, backup, and data management tools."}</li>
                                <li>{"Provides unique fingerprints for file identification."}</li>
                                <li>{"Essential for digital forensics and evidence integrity."}</li>
                                <li>{"Enables efficient deduplication in storage systems."}</li>
                                <li>{"Supports compliance with security standards and regulations."}</li>
                            </ul>
                            
                            <h3>{"Performance & Implementation"}</h3>
                            <ul>
                                <li><strong>{"Efficient Processing:"}</strong> {"Files are processed efficiently with optimized algorithms for maximum performance."}</li>
                                <li><strong>{"Memory Optimization:"}</strong> {"Smart memory usage prevents browser crashes with large files."}</li>
                                <li><strong>{"Local Processing:"}</strong> {"All computation happens locally - no file data ever leaves your device."}</li>
                                <li><strong>{"Real-time Feedback:"}</strong> {"Progress tracking and instant results for better user experience."}</li>
                                <li><strong>{"Flexible Input Formats:"}</strong> {"Verification supports multiple input formats: hex, colon-separated, Base64, and C-style arrays for maximum compatibility."}</li>
                                <li><strong>{"WebAssembly Speed:"}</strong> {"Built with Rust and compiled to WebAssembly for near-native performance."}</li>
                                <li><strong>{"Chunked Processing:"}</strong> {"Large files are processed in chunks to maintain UI responsiveness."}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"❓ Frequently Asked Questions"}</h2>
                            
                            <div class="faq-item">
                                <h3>{"Q: What's the difference between MD5, SHA-1, SHA-256, and SHA-512?"}</h3>
                                <p>
                                    {"A: They are different cryptographic hash algorithms with varying output lengths and security levels. MD5 (32 hex chars) and SHA-1 (40 hex chars) are considered broken for security use but still acceptable for checksums. SHA-256 (64 hex chars) and SHA-512 (128 hex chars) are modern, secure algorithms recommended for all security applications."}
                                </p>
                            </div>
                            
                            <div class="faq-item">
                                <h3>{"Q: Can I hash very large files?"}</h3>
                                <p>
                                    {"A: Yes, this tool processes files in chunks and shows progress for large files. Files up to 2GB are supported. For even larger files, consider using command-line tools like 'sha256sum' or 'certutil' which don't have browser memory limitations."}
                                </p>
                            </div>
                            
                            <div class="faq-item">
                                <h3>{"Q: Is file hashing secure?"}</h3>
                                <p>
                                    {"A: File hashing itself is secure when using modern algorithms (SHA-256+). However, hashing is for integrity verification, not encryption. Don't use hashes alone for storing sensitive data. For passwords, use dedicated functions like bcrypt or Argon2."}
                                </p>
                            </div>
                            
                            <div class="faq-item">
                                <h3>{"Q: Why are there checkboxes for each algorithm?"}</h3>
                                <p>
                                    {"A: You can select which hash algorithms to compute, saving time and resources if you only need one or two. For example, if you only need SHA-256 for verification, uncheck the others to speed up processing. Your selections are automatically saved for future use."}
                                </p>
                            </div>
                            
                            <div class="faq-item">
                                <h3>{"Q: What if I select a huge file?"}</h3>
                                <p>
                                    {"A: The tool will process it efficiently with a progress bar. However, very large files (approaching 2GB) may use significant memory and take longer. The browser might become less responsive during processing, but won't crash."}
                                </p>
                            </div>
                            
                            <div class="faq-item">
                                <h3>{"Q: What is HMAC and when should I use it?"}</h3>
                                <p>
                                    {"A: HMAC (Hash-based Message Authentication Code) uses a secret key to create authenticated hashes. Use it for API authentication, secure communications, or when you need to verify both integrity and authenticity. Enter a secret key in the HMAC section to generate these values. The key should be kept secret and shared only with authorized parties."}
                                </p>
                            </div>
                            
                            <div class="faq-item">
                                <h3>{"Q: What's the difference between CRC32 and cryptographic hashes?"}</h3>
                                <p>
                                    {"A: CRC32 is designed for error detection (like in ZIP files) and is very fast but not secure. Cryptographic hashes (MD5, SHA-256, etc.) are designed for security and can detect malicious tampering. Use CRC32 for simple integrity checks and SHA-256+ for security. CRC32 is also much shorter (8 hex characters vs 64 for SHA-256)."}
                                </p>
                            </div>
                            
                            <div class="faq-item">
                                <h3>{"Q: Can I use any string as an HMAC key?"}</h3>
                                <p>
                                    {"A: Yes, you can enter any text as an HMAC key. For security applications, use strong, random keys (at least 32 characters). The tool accepts both text strings and hexadecimal values. Avoid predictable keys like 'password' or '123456' in production environments."}
                                </p>
                            </div>
                            
                            <div class="faq-item">
                                <h3>{"Q: What hash formats can I use for verification?"}</h3>
                                <p>
                                    {"A: The verification feature supports multiple input formats: standard hex (d85e5d59b4d4), colon-separated (d8:5e:5d:59:b4:d4), Base64 (2F5dWbTUnvzj), and C-style arrays ({0xd8, 0x5e, 0x5d, 0x59}). The tool automatically detects and converts the format for comparison, so you don't need to worry about format matching."}
                                </p>
                            </div>
                            
                            <div class="faq-item">
                                <h3>{"Q: How does the automatic format detection work?"}</h3>
                                <p>
                                    {"A: The tool analyzes the input pattern to detect the format: colons indicate colon-separated format, curly braces with 0x prefixes indicate C-style arrays, Base64-compatible characters suggest Base64 encoding, and plain hex digits are treated as standard hexadecimal. Invalid formats show helpful error messages with suggestions."}
                                </p>
                            </div>
                            
                            <div class="faq-item">
                                <h3>{"Q: Are my settings and preferences saved?"}</h3>
                                <p>
                                    {"A: Yes! Your algorithm selections, output format preference, and HMAC toggle state are automatically saved in your browser's Local Storage. This data never leaves your device and persists across browser sessions. You can clear it anytime through your browser's storage settings."}
                                </p>
                            </div>
                            
                            <div class="faq-item">
                                <h3>{"Q: What data is stored locally vs. sent to servers?"}</h3>
                                <p>
                                    {"A: Only your preferences (algorithm selections, output format) are stored locally. NO FILE DATA, HASHES, OR KEYS are ever stored or sent anywhere. All file processing happens entirely in your browser using WebAssembly. This ensures complete privacy and security."}
                                </p>
                            </div>
                            
                            <div class="faq-item">
                                <h3>{"Q: Can I verify files from different sources with different hash formats?"}</h3>
                                <p>
                                    {"A: Absolutely! If a software vendor provides a hash as 'A1:B2:C3:D4', a database stores it as Base64, and your code uses C-style arrays, you can verify all of them without conversion. Just paste the expected value in any supported format, and the tool handles the comparison automatically."}
                                </p>
                            </div>
                            
                            <div class="faq-item">
                                <h3>{"Q: Which algorithms should I choose for different use cases?"}</h3>
                                <p>
                                    {"A: For security: SHA-256 or SHA-512. For legacy compatibility: SHA-1 (if required). For speed/deduplication: MD5 (non-security use only). For error detection: CRC32. For authentication: HMAC with SHA-256+. When in doubt, SHA-256 is the most widely supported secure option."}
                                </p>
                            </div>
                            
                            <div class="faq-item">
                                <h3>{"Q: How do I verify webhook signatures from GitHub, Stripe, etc.?"}</h3>
                                <p>
                                    {"A: Enable HMAC generation, enter your webhook secret key, select the algorithm specified by the service (usually SHA-256), then 'upload' your webhook payload as a file. Compare the generated HMAC with the signature in the webhook headers. This verifies the payload came from the authentic source."}
                                </p>
                            </div>
                            
                            <div class="faq-item">
                                <h3>{"Q: What's the difference between Hash and HMAC verification?"}</h3>
                                <p>
                                    {"A: Hash verification compares file integrity (did the file change?). HMAC verification checks both integrity AND authenticity (did the file change AND did it come from someone with the secret key?). Use Hash for general file verification, HMAC when you need to verify the source is trusted."}
                                </p>
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
                            <ul>
                                {
                                    ToolCategoryManager::get_related_tools("file-hash")
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
                        // 에러 메시지 표시
                        if let Some(error) = &self.error_message {
                            <div style="background-color: #fee; border: 1px solid #fcc; color: #c33; padding: 10px; border-radius: 5px; margin-bottom: 10px; display: flex; justify-content: space-between; align-items: center;">
                                <span>{ error }</span>
                                <button 
                                    onclick={_ctx.link().callback(|_| Msg::ClearError)}
                                    style="background: none; border: none; color: #c33; cursor: pointer; font-size: 16px; padding: 0;"
                                >
                                    {"×"}
                                </button>
                            </div>
                        }
                        
                        // Choose Hash Algorithms - tool-inner 밖으로 이동
                        <div style="display: flex; flex-wrap: wrap; justify-content: space-between; align-items: center; margin-bottom: 10px;">
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
                        
                        // Output Format - tool-inner 밖으로 이동
                        <div style="margin-bottom: 10px;">
                            <label style="width: 70%;">{"Output Format:"}</label>
                            <select
                                style="width: 30%;"
                                onchange={_ctx.link().callback(|e: Event| {
                                    let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                    let format = match value.as_str() {
                                        "lowercase" => OutputFormat::Lowercase,
                                        "uppercase" => OutputFormat::Uppercase,
                                        "colon_separated" => OutputFormat::ColonSeparated,
                                        "base64" => OutputFormat::Base64,
                                        "c_style_array" => OutputFormat::CStyleArray,
                                        _ => OutputFormat::Lowercase,
                                    };
                                    Msg::OutputFormatChanged(format)
                                })}
                            >
                                <option value="lowercase" selected={self.output_format == OutputFormat::Lowercase}>
                                    { "Lowercase (d85e5d59b4d4...)" }
                                </option>
                                <option value="uppercase" selected={self.output_format == OutputFormat::Uppercase}>
                                    { "Uppercase (D85E5D59B4D4...)" }
                                </option>
                                <option value="colon_separated" selected={self.output_format == OutputFormat::ColonSeparated}>
                                    { "Colon Separated (d8:5e:5d:59...)" }
                                </option>
                                <option value="base64" selected={self.output_format == OutputFormat::Base64}>
                                    { "Base64 (2F5dWbTUnvzj...)" }
                                </option>
                                <option value="c_style_array" selected={self.output_format == OutputFormat::CStyleArray}>
                                    { "C-Style Array ({0xd8, 0x5e...})" }
                                </option>
                            </select>
                        </div>
                        
                        // HMAC 옵션 - 파일 업로드 전에 설정
                        <div style="margin-bottom: 10px;">
                            <div style="display: flex; align-items: center; gap: 10px; margin-bottom: 5px;">
                                <input
                                    type="checkbox"
                                    id="hmac-enable"
                                    checked={self.show_hmac_section}
                                    onclick={_ctx.link().callback(|_| Msg::ToggleHmacSection)}
                                />
                                <label for="hmac-enable" style="cursor: pointer; margin-bottom: 0px; font-weight: bold;">
                                    {"Enable HMAC Generation"}
                                </label>
                            </div>
                            
                            if self.show_hmac_section {
                                <input
                                    type="text"
                                    placeholder="Enter HMAC secret key (e.g., 'secret' or hex: '48656c6c6f')"
                                    value={self.hmac_key.clone()}
                                    oninput={_ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::HmacKeyChanged(input.value())
                                    })}
                                    style="width: 100%;"
                                />
                                <div style="font-size: 12px; color: var(--color-subfont); margin-top: 2px;">
                                    {"HMAC will be calculated for all selected hash algorithms above"}
                                </div>
                            }
                        </div>
                        
                        <div class="tool-inner" style="width: 100%; margin-bottom: 10px;">
                            <div>
                                // 파일 업로드 영역
                                <div 
                                    style={format!("border: 2px dashed {}; border-radius: 8px; padding: 15px; text-align: center; transition: all 0.2s ease;{}", 
                                        if self.is_dragging { "var(--color-primary)" } else { "var(--color-border)" },
                                        if self.is_dragging { " background-color: rgba(var(--color-primary-rgb), 0.1);" } else { "" }
                                    )}
                                    ondragover={_ctx.link().callback(|e: DragEvent| {
                                        e.prevent_default();
                                        Msg::DragOver
                                    })}
                                    ondragenter={_ctx.link().callback(|e: DragEvent| {
                                        e.prevent_default();
                                        Msg::DragEnter
                                    })}
                                    ondragleave={_ctx.link().callback(|e: DragEvent| {
                                        e.prevent_default();
                                        Msg::DragLeave
                                    })}
                                    ondrop={_ctx.link().callback(|e: DragEvent| {
                                        e.prevent_default();
                                        
                                        // wasm-bindgen을 통해 dataTransfer.files에 접근
                                        let event_obj = wasm_bindgen::JsValue::from(e);
                                        if let Ok(data_transfer) = js_sys::Reflect::get(&event_obj, &"dataTransfer".into()) {
                                            if let Ok(files) = js_sys::Reflect::get(&data_transfer, &"files".into()) {
                                                if let Ok(file_list_obj) = files.dyn_into::<web_sys::FileList>() {
                                                    if let Some(file) = file_list_obj.get(0) {
                                                        return Msg::Drop(file);
                                                    }
                                                }
                                            }
                                        }
                                        Msg::DragLeave
                                    })}
                                >
                                    if let Some(file_info) = &self.file_info {
                                        // 파일이 업로드된 상태
                                        <div>
                                            // 첫 번째 행: 파일 정보
                                            <div style="display: flex; align-items: center; margin-bottom: 10px;">
                                                <span style="font-size: 20px; margin-right: 8px;">
                                                    { Self::get_file_icon(&file_info.mime_type) }
                                                </span>
                                                <div style="text-align: left; overflow-y: auto;">
                                                    <div style="font-weight: bold; color: var(--color-font);">
                                                        { &file_info.name }
                                                    </div>
                                                    <div style="font-size: 12px; color: var(--color-subfont);">
                                                        { format!("{} • {}", Self::format_file_size(file_info.size), &file_info.mime_type) }
                                                    </div>
                                                </div>
                                            </div>
                                            // 두 번째 행: Remove 버튼
                                            <div style="display: flex; justify-content: center;">
                                                <button 
                                                    type="button"
                                                    style="background: var(--color-error); color: white; border: none; border-radius: 4px; padding: 8px 16px; cursor: pointer;"
                                                    onclick={_ctx.link().callback(|_| Msg::ClearFile)}>
                                                    { "Remove" }
                                                </button>
                                            </div>
                                        </div>
                                        if self.is_computing {
                                            <div style="margin-top: 10px; color: var(--color-subfont);">
                                                <i class="fa-solid fa-spinner fa-spin"></i> { " Computing hashes..." }
                                            </div>
                                        }
                                    } else {
                                        // 파일 업로드 대기 상태
                                        <div>
                                            <div>
                                                <i class="fa-solid fa-cloud-upload-alt" style="font-size: 24px; color: var(--color-subfont);"></i>
                                            </div>
                                            <div style="margin-bottom: 8px; font-weight: bold; color: var(--color-primary);">
                                                { "Drop files here or click to upload" }
                                            </div>
                                            <div style="font-size: 12px; color: var(--color-subfont); margin-bottom: 10px;">
                                                { format!("Supports any file type (Max: {})", Self::format_file_size(MAX_FILE_SIZE)) }
                                            </div>
                                            <input
                                                type="file"
                                                id="file-upload"
                                                style="display: none;"
                                        onchange={_ctx.link().callback(|e: Event| {
                                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                            if let Some(files) = input.files() {
                                                if let Some(file) = files.get(0) {
                                                            let file_clone = file.clone();
                                                            input.set_value(""); // 같은 파일을 다시 선택할 수 있도록 초기화
                                                    return Msg::FileSelected(file_clone);
                                                }
                                                    }
                                            Msg::NoOp
                                                })}
                                            />
                                            <label 
                                                for="file-upload" 
                                                style="display: inline-block; background: var(--color-primary); color: white; padding: 8px 16px; border-radius: 4px; cursor: pointer; border: none;">
                                                { "Choose File" }
                                            </label>
                                        </div>
                                    }
                                </div>
                                
                            </div>
                        </div>
                        
                        // 파일 메타데이터 토글 버튼 및 표시
                        if let Some(file_info) = &self.file_info {
                            <div style="margin-bottom: 10px; text-align: center;">
                                    <button
                                        class="tool-btn"
                                    onclick={_ctx.link().callback(|_| Msg::ToggleFileMetadata)}
                                    style="background-color: var(--color-fourth); color: white; border: none; border-radius: 5px; padding: 8px 16px; cursor: pointer;"
                                >
                                    if self.show_file_metadata {
                                        {"Hide File Details"}
                                    } else {
                                        {"Show File Details"}
                                    }
                                    </button>
                                </div>
                            
                            if self.show_file_metadata {
                                <div class="tool-inner" style="width: 100%; margin-bottom: 10px;">
                                    <div>
                                        <div class="tool-subtitle" style="margin-bottom: 10px;">{"File Metadata"}</div>
                                        
                                        <div style="display: grid; gap: 8px;">
                                            <div style="display: flex; justify-content: space-between; align-items: center; padding: 8px; background-color: var(--color-third); border-radius: 4px;">
                                                <span style="font-weight: bold;">{"File Name:"}</span>
                                                <span style="word-break: break-all; text-align: right; max-width: 60%;">{ &file_info.name }</span>
                            </div>
                                            
                                            <div style="display: flex; justify-content: space-between; align-items: center; padding: 8px; background-color: var(--color-third); border-radius: 4px;">
                                                <span style="font-weight: bold;">{"File Size:"}</span>
                                                <span>{ Self::format_file_size(file_info.size) }</span>
                        </div>
                                            
                                            <div style="display: flex; justify-content: space-between; align-items: center; padding: 8px; background-color: var(--color-third); border-radius: 4px;">
                                                <span style="font-weight: bold;">{"MIME Type:"}</span>
                                                <span>{ &file_info.mime_type }</span>
                                            </div>
                                            
                                            if let Some(last_modified) = file_info.last_modified {
                                                <div style="display: flex; justify-content: space-between; align-items: center; padding: 8px; background-color: var(--color-third); border-radius: 4px;">
                                                    <span style="font-weight: bold;">{"Last Modified:"}</span>
                                                    <span>{ Self::format_timestamp(last_modified) }</span>
                                                </div>
                                            }
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                        
                        // 해시 검증 토글 버튼
                        if self.progress == 1.0 && !self.is_computing {
                            <div style="margin-bottom: 10px; text-align: center;">
                                <button 
                                    class="tool-btn"
                                    onclick={_ctx.link().callback(|_| Msg::ToggleHashVerification)}
                                    style="background-color: var(--color-fourth); color: white; border: none; border-radius: 5px; padding: 8px 16px; cursor: pointer;"
                                >
                                    if self.show_hash_verification {
                                        {"Hide Hash Verification"}
                                    } else {
                                        {"Show Hash Verification"}
                                    }
                                </button>
                            </div>
                        }
                        
                        // 해시 검증 섹션
                        if self.show_hash_verification {
                            <div class="tool-inner" style="width: 100%; margin-bottom: 10px;">
                                <div>
                                    <div class="tool-subtitle" style="margin-bottom: 5px;">{"Hash Verification"}</div>
                                    
                                    // 검증 타입 선택
                                    <div style="margin-bottom: 10px;">
                                        <label style="display: block; margin-bottom: 5px; font-weight: bold;">{"Verification Type:"}</label>
                                        <div style="display: flex; gap: 20px; align-items: center;">
                                            <div style="display: flex; align-items: center; gap: 5px;">
                                                <input
                                                    type="radio"
                                                    id="verification-hash"
                                                    name="verification-type"
                                                    checked={self.verification_type == VerificationType::Hash}
                                                    onclick={_ctx.link().callback(|_| Msg::VerificationTypeChanged(VerificationType::Hash))}
                                                />
                                                <label for="verification-hash" style="cursor: pointer; margin-bottom: 0px;">{"Hash"}</label>
                                            </div>
                                            <div style="display: flex; align-items: center; gap: 5px;">
                                                <input
                                                    type="radio"
                                                    id="verification-hmac"
                                                    name="verification-type"
                                                    checked={self.verification_type == VerificationType::Hmac}
                                                    onclick={_ctx.link().callback(|_| Msg::VerificationTypeChanged(VerificationType::Hmac))}
                                                />
                                                <label for="verification-hmac" style="cursor: pointer; margin-bottom: 0px;">{"HMAC"}</label>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    // HMAC 키 입력 (HMAC 타입일 때만 표시)
                                    if self.verification_type == VerificationType::Hmac {
                                        <div style="margin-bottom: 10px;">
                                            <label style="display: block; margin-bottom: 5px; font-weight: bold;">{"Secret Key:"}</label>
                                            <input
                                                type="text"
                                                placeholder="Enter secret key for HMAC verification..."
                                                value={self.verification_hmac_key.clone()}
                                                oninput={_ctx.link().callback(|e: InputEvent| {
                                                    let input: HtmlInputElement = e.target_unchecked_into();
                                                    Msg::VerificationHmacKeyChanged(input.value())
                                                })}
                                                style="width: 100%;"
                                            />
                                        </div>
                                    }
                                    
                                    // 예상 해시 입력
                                    <div style="margin-bottom: 10px;">
                                        <label style="display: block; margin-bottom: 5px; font-weight: bold;">
                                            if self.verification_type == VerificationType::Hash {
                                                {"Expected Hash:"}
                                            } else {
                                                {"Expected HMAC:"}
                                            }
                                        </label>
                                        <input
                                            type="text"
                                            placeholder={
                                                if self.verification_type == VerificationType::Hash {
                                                    "Enter expected hash value..."
                                                } else {
                                                    "Enter expected HMAC value..."
                                                }
                                            }
                                            value={self.expected_hash.clone()}
                                            oninput={_ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                Msg::ExpectedHashChanged(input.value())
                                            })}
                                            style="width: 100%;"
                                        />
                                        <div style="font-size: 11px; color: var(--color-subfont); margin-top: 3px; line-height: 1.3;">
                                            {"Supported formats: Hex (d85e5d59b4d4), Colon-separated (d8:5e:5d:59), Base64 (2F5dWbTU), C-style ({0xd8, 0x5e, 0x5d})"}
                                        </div>
                                    </div>
                                    
                                    if let Some(comparison) = &self.hash_comparison {
                                        <div style={format!("padding: 10px; border-radius: 5px; {}", 
                                            if comparison.matches { 
                                                "background-color: #d4edda; color: #155724; border: 1px solid #c3e6cb;" 
                                            } else { 
                                                "background-color: #f8d7da; color: #721c24; border: 1px solid #f5c6cb;" 
                                            })}>
                                            <div style="font-weight: bold; margin-bottom: 5px;">
                                                if comparison.matches {
                                                    {"✅ Verification passed!"}
                                                } else {
                                                    {"❌ Verification failed!"}
                                                }
                                            </div>
                                            <div style="font-size: 12px;">
                                                {format!("Algorithm: {}", comparison.algorithm)}
                                            </div>
                                            if !comparison.matches {
                                                <div style="font-size: 12px; margin-top: 5px;">
                                                    <div>{format!("Expected: {}", comparison.expected)}</div>
                                                    <div>{format!("Actual: {}", comparison.actual)}</div>
                                                </div>
                                            }
                                        </div>
                                    } else if !self.expected_hash.trim().is_empty() {
                                        <div style="color: var(--color-subfont); font-style: italic;">
                                            {"Waiting for hash calculation to complete..."}
                                        </div>
                                    }
                                </div>
                            </div>
                        }
                        
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
                                        value={
                                            if let Some(file_info) = &self.file_info {
                                                Self::format_file_size(file_info.size)
                                            } else {
                                                "".to_string()
                                            }
                                        }
                                        onclick={_ctx.link().callback(|e: MouseEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::CopyToClipboard(input.value())
                                        })}
                                    />
                                </div>
                            {
                                for hashes.iter().filter(|(key, _, _)| *self.selected.get(*key).unwrap_or(&false)).map(|(_key, label, value)| html! {
                                    <div>
                                        <div class="tool-subtitle" style="margin-bottom: 5px; margin-top: 10px;">{ *label } </div>
                                        <input
                                            type="text"
                                            readonly=true
                                            style="cursor: pointer;"
                                            value={self.format_hash_output(value)}
                                            onclick={_ctx.link().callback(|e: MouseEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                Msg::CopyToClipboard(input.value())
                                            })}
                                        />
                                    </div>
                                })
                            }
                            
                            // HMAC 값들 표시 (HMAC 옵션이 활성화되어 있고 키가 있는 경우)
                            if self.show_hmac_section && !self.hmac_key.trim().is_empty() {
                                {
                                    for hmac_hashes.iter().filter(|(key, _, value)| {
                                        // 해당 알고리즘이 선택되어 있고 HMAC 값이 있는 경우만 표시
                                        let base_algorithm = key.replace("hmac_", "");
                                        *self.selected.get(&base_algorithm).unwrap_or(&false) && !value.is_empty()
                                    }).map(|(_key, label, value)| html! {
                                        <div>
                                            <div class="tool-subtitle" style="margin-bottom: 5px; margin-top: 10px;">{ *label } </div>
                                            <input
                                                type="text"
                                                readonly=true
                                                style="cursor: pointer;"
                                                value={self.format_hash_output(value)}
                                                onclick={_ctx.link().callback(|e: MouseEvent| {
                                                    let input: HtmlInputElement = e.target_unchecked_into();
                                                    Msg::CopyToClipboard(input.value())
                                                })}
                                            />
                            </div>
                                    })
                                }
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
                        meta_tag.set_attribute("content", "Advanced File Hash Generator with drag & drop support, file validation, and hash verification. Compute MD5, SHA-1, SHA-256, SHA-512 hashes for any file up to 2GB. Features real-time progress tracking, chunked processing for large files, and comprehensive file integrity verification tools.").unwrap();
                    }
                }
            }
        }
    }
}

impl ToolFileHash {
    // 다양한 해시 입력 포맷을 표준 hex로 변환하는 함수
    fn normalize_hash_input(input: &str) -> Result<String, String> {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            return Err("Empty input".to_string());
        }
        
        // 1. Colon-separated format (d8:5e:5d:59:b4:d4...)
        if trimmed.contains(':') {
            let hex_string: String = trimmed.split(':')
                .map(|part| part.trim())
                .collect::<Vec<_>>()
                .join("");
            
            // 유효한 hex인지 확인
            if hex_string.chars().all(|c| c.is_ascii_hexdigit()) {
                return Ok(hex_string.to_lowercase());
            } else {
                return Err("Invalid colon-separated hex format".to_string());
            }
        }
        
        // 2. C-style array format ({0xd8, 0x5e, 0x5d...})
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            let inner = &trimmed[1..trimmed.len()-1];
            let hex_parts: Result<Vec<String>, String> = inner
                .split(',')
                .map(|part| {
                    let part = part.trim();
                    if part.starts_with("0x") || part.starts_with("0X") {
                        let hex_part = &part[2..];
                        if hex_part.len() == 2 && hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
                            Ok(hex_part.to_string())
                        } else {
                            Err("Invalid hex byte in C-style array".to_string())
                        }
                    } else {
                        Err("C-style array must contain 0x prefixed values".to_string())
                    }
                })
                .collect();
            
            match hex_parts {
                Ok(parts) => return Ok(parts.join("").to_lowercase()),
                Err(e) => return Err(e),
            }
        }
        
        // 3. Base64 format (길이와 문자셋으로 추정)
        if trimmed.chars().all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=') 
            && (trimmed.len() % 4 == 0 || trimmed.ends_with('=')) {
            
            match BASE64_STANDARD.decode(trimmed) {
                Ok(bytes) => {
                    let hex_string = bytes.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>();
                    return Ok(hex_string);
                }
                Err(_) => {
                    // Base64 디코딩 실패 시 일반 hex로 처리
                }
            }
        }
        
        // 4. 일반 hex format (기본값)
        let clean_hex = trimmed.to_lowercase();
        if clean_hex.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(clean_hex)
        } else {
            Err("Invalid hash format. Supported formats: hex, colon-separated (d8:5e:5d), Base64, or C-style array ({0xd8, 0x5e})".to_string())
        }
    }

    // HMAC 계산 헬퍼 함수들
    fn calculate_hmac_md5(key: &[u8], data: &[u8]) -> Result<String, String> {
        Hmac::<Md5>::new_from_slice(key)
            .map_err(|_| "Invalid key".to_string())
            .map(|mut hmac| {
                hmac.update(data);
                format!("{:x}", hmac.finalize().into_bytes())
            })
    }
    
    fn calculate_hmac_sha1(key: &[u8], data: &[u8]) -> Result<String, String> {
        Hmac::<Sha1>::new_from_slice(key)
            .map_err(|_| "Invalid key".to_string())
            .map(|mut hmac| {
                hmac.update(data);
                format!("{:x}", hmac.finalize().into_bytes())
            })
    }
    
    fn calculate_hmac_sha256(key: &[u8], data: &[u8]) -> Result<String, String> {
        Hmac::<Sha256>::new_from_slice(key)
            .map_err(|_| "Invalid key".to_string())
            .map(|mut hmac| {
                hmac.update(data);
                format!("{:x}", hmac.finalize().into_bytes())
            })
    }
    
    fn calculate_hmac_sha512(key: &[u8], data: &[u8]) -> Result<String, String> {
        Hmac::<Sha512>::new_from_slice(key)
            .map_err(|_| "Invalid key".to_string())
            .map(|mut hmac| {
                hmac.update(data);
                format!("{:x}", hmac.finalize().into_bytes())
            })
    }

    fn validate_file(&self, file: &File) -> Result<(), String> {
        let file_size = file.size() as usize;
        
        // 파일 크기 검증
        if file_size > MAX_FILE_SIZE {
            return Err(format!(
                "File size ({}) exceeds maximum allowed size ({}). Please select a smaller file.",
                Self::format_file_size(file_size),
                Self::format_file_size(MAX_FILE_SIZE)
            ));
        }
        
        // 빈 파일 검증
        if file_size == 0 {
            return Err("Cannot process empty files. Please select a file with content.".to_string());
        }
        
        // MIME 타입 검증 (너무 제한적이므로 경고만 표시)
        let mime_type = file.type_();
        if !mime_type.is_empty() && !ALLOWED_MIME_TYPES.contains(&mime_type.as_str()) {
            // 경고만 표시하고 처리는 계속 진행
            log::warn!("File type '{}' is not in the allowed list but will be processed", mime_type);
        }
        
        Ok(())
    }
    
    fn format_file_size(size: usize) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size_f = size as f64;
        let mut unit_index = 0;
        
        while size_f >= 1024.0 && unit_index < UNITS.len() - 1 {
            size_f /= 1024.0;
            unit_index += 1;
        }
        
        if unit_index == 0 {
            format!("{} {}", size, UNITS[unit_index])
        } else {
            format!("{:.2} {}", size_f, UNITS[unit_index])
        }
    }
    
    fn format_timestamp(timestamp: f64) -> String {
        let date = js_sys::Date::new(&JsValue::from(timestamp));
        let year = date.get_full_year();
        let month = format!("{:02}", date.get_month() + 1);
        let day = format!("{:02}", date.get_date());
        let hours = format!("{:02}", date.get_hours());
        let minutes = format!("{:02}", date.get_minutes());
        format!("{}-{}-{} {}:{}", year, month, day, hours, minutes)
    }

    fn format_hash_output(&self, hash: &str) -> String {
        if hash.is_empty() {
            return String::new();
        }
        
        match self.output_format {
            OutputFormat::Lowercase => hash.to_lowercase(),
            OutputFormat::Uppercase => hash.to_uppercase(),
            OutputFormat::ColonSeparated => {
                let lower_hash = hash.to_lowercase();
                let mut result = String::new();
                for (i, c) in lower_hash.chars().enumerate() {
                    if i > 0 && i % 2 == 0 {
                        result.push(':');
                    }
                    result.push(c);
                }
                result
            }
            OutputFormat::Base64 => {
                // hex 문자열을 바이트로 변환한 후 base64 인코딩
                if let Ok(bytes) = hex::decode(hash) {
                    BASE64_STANDARD.encode(&bytes)
                } else {
                    hash.to_string()
                }
            }
            OutputFormat::CStyleArray => {
                // hex 문자열을 바이트로 변환한 후 C 스타일 배열로 포맷팅
                if let Ok(bytes) = hex::decode(hash) {
                    let mut result = String::from("{");
                    for (i, byte) in bytes.iter().enumerate() {
                        if i > 0 {
                            result.push_str(", ");
                        }
                        result.push_str(&format!("0x{:02x}", byte));
                    }
                    result.push('}');
                    result
                } else {
                    hash.to_string()
                }
            }
        }
    }

    fn perform_hash_comparison(&mut self) {
        if self.expected_hash.trim().is_empty() {
            self.hash_comparison = None;
            return;
        }
        
        // 입력된 해시를 표준 hex 포맷으로 정규화
        let expected_clean = match Self::normalize_hash_input(&self.expected_hash) {
            Ok(normalized) => normalized,
            Err(error) => {
                self.hash_comparison = Some(HashComparison {
                    algorithm: "Format Error".to_string(),
                    matches: false,
                    expected: self.expected_hash.trim().to_string(),
                    actual: error,
                });
                return;
            }
        };
        
        match self.verification_type {
            VerificationType::Hash => {
                // 일반 해시 검증
                let hashes = vec![
                    ("MD5", &self.hash_md5),
                    ("SHA-1", &self.hash_sha1),
                    ("SHA-256", &self.hash_sha256),
                    ("SHA-512", &self.hash_sha512),
                    ("CRC32", &self.crc32),
                ];
                
                for (algorithm, actual_hash) in hashes {
                    if !actual_hash.is_empty() && actual_hash.to_lowercase() == expected_clean {
                        self.hash_comparison = Some(HashComparison {
                            algorithm: algorithm.to_string(),
                            matches: true,
                            expected: expected_clean.clone(),
                            actual: actual_hash.to_lowercase(),
                        });
                        return;
                    }
                }
                
                // 일치하는 해시가 없는 경우, 길이를 기준으로 가장 가능성 높은 알고리즘 추정
                let expected_len = expected_clean.len();
                let (algorithm, actual_hash) = match expected_len {
                    8 => ("CRC32", &self.crc32),
                    32 => ("MD5", &self.hash_md5),
                    40 => ("SHA-1", &self.hash_sha1), 
                    64 => ("SHA-256", &self.hash_sha256),
                    128 => ("SHA-512", &self.hash_sha512),
                    _ => {
                        // 길이가 일치하지 않는 경우, 비어있지 않은 첫 번째 해시 사용
                        if !self.hash_sha256.is_empty() {
                            ("SHA-256", &self.hash_sha256)
                        } else if !self.crc32.is_empty() {
                            ("CRC32", &self.crc32)
                        } else if !self.hash_md5.is_empty() {
                            ("MD5", &self.hash_md5)
                        } else if !self.hash_sha1.is_empty() {
                            ("SHA-1", &self.hash_sha1)
                        } else {
                            ("SHA-512", &self.hash_sha512)
                        }
                    }
                };
                
                if !actual_hash.is_empty() {
                    self.hash_comparison = Some(HashComparison {
                        algorithm: algorithm.to_string(),
                        matches: false,
                        expected: expected_clean,
                        actual: actual_hash.to_lowercase(),
                    });
                }
            }
            VerificationType::Hmac => {
                // HMAC 검증
                if self.verification_hmac_key.trim().is_empty() {
                    self.hash_comparison = Some(HashComparison {
                        algorithm: "HMAC".to_string(),
                        matches: false,
                        expected: expected_clean,
                        actual: "Secret key required for HMAC verification".to_string(),
                    });
                    return;
                }
                
                // 파일 데이터와 키가 있으면 HMAC 계산해서 비교
                if let Some(file_data) = &self.file_data {
                    let hmac_key_bytes = self.verification_hmac_key.as_bytes();
                    
                    // 각 HMAC 알고리즘과 비교
                    if let Ok(calculated_hmac) = Self::calculate_hmac_md5(hmac_key_bytes, file_data) {
                        if calculated_hmac.to_lowercase() == expected_clean {
                            self.hash_comparison = Some(HashComparison {
                                algorithm: "HMAC-MD5".to_string(),
                                matches: true,
                                expected: expected_clean.clone(),
                                actual: calculated_hmac.to_lowercase(),
                            });
                            return;
                        }
                    }
                    
                    if let Ok(calculated_hmac) = Self::calculate_hmac_sha1(hmac_key_bytes, file_data) {
                        if calculated_hmac.to_lowercase() == expected_clean {
                            self.hash_comparison = Some(HashComparison {
                                algorithm: "HMAC-SHA1".to_string(),
                                matches: true,
                                expected: expected_clean.clone(),
                                actual: calculated_hmac.to_lowercase(),
                            });
                            return;
                        }
                    }
                    
                    if let Ok(calculated_hmac) = Self::calculate_hmac_sha256(hmac_key_bytes, file_data) {
                        if calculated_hmac.to_lowercase() == expected_clean {
                            self.hash_comparison = Some(HashComparison {
                                algorithm: "HMAC-SHA256".to_string(),
                                matches: true,
                                expected: expected_clean.clone(),
                                actual: calculated_hmac.to_lowercase(),
                            });
                            return;
                        }
                    }
                    
                    if let Ok(calculated_hmac) = Self::calculate_hmac_sha512(hmac_key_bytes, file_data) {
                        if calculated_hmac.to_lowercase() == expected_clean {
                            self.hash_comparison = Some(HashComparison {
                                algorithm: "HMAC-SHA512".to_string(),
                                matches: true,
                                expected: expected_clean.clone(),
                                actual: calculated_hmac.to_lowercase(),
                            });
                            return;
                        }
                    }
                    
                    // 일치하는 HMAC이 없는 경우, 길이를 기준으로 가장 가능성 높은 알고리즘으로 비교
                    let expected_len = expected_clean.len();
                    let result = match expected_len {
                        32 => ("HMAC-MD5", Self::calculate_hmac_md5(hmac_key_bytes, file_data)),
                        40 => ("HMAC-SHA1", Self::calculate_hmac_sha1(hmac_key_bytes, file_data)),
                        64 => ("HMAC-SHA256", Self::calculate_hmac_sha256(hmac_key_bytes, file_data)),
                        128 => ("HMAC-SHA512", Self::calculate_hmac_sha512(hmac_key_bytes, file_data)),
                        _ => ("HMAC-SHA256", Self::calculate_hmac_sha256(hmac_key_bytes, file_data)),
                    };
                    
                    match result {
                        (algorithm, Ok(calculated_hmac)) => {
                            self.hash_comparison = Some(HashComparison {
                                algorithm: algorithm.to_string(),
                                matches: false,
                                expected: expected_clean,
                                actual: calculated_hmac.to_lowercase(),
                            });
                        }
                        (algorithm, Err(err)) => {
                            self.hash_comparison = Some(HashComparison {
                                algorithm: algorithm.to_string(),
                                matches: false,
                                expected: expected_clean,
                                actual: format!("Error: {}", err),
                            });
                        }
                    }
                } else {
                    self.hash_comparison = Some(HashComparison {
                        algorithm: "HMAC".to_string(),
                        matches: false,
                        expected: expected_clean,
                        actual: "File data not available for HMAC calculation".to_string(),
                    });
                }
            }
        }
    }

    fn get_file_icon(mime_type: &str) -> &'static str {
        match mime_type {
            t if t.starts_with("image/") => "🖼️",
            t if t.starts_with("video/") => "🎥",
            t if t.starts_with("audio/") => "🎵",
            t if t.starts_with("text/") => "📄",
            "application/pdf" => "📕",
            "application/zip" | "application/x-zip-compressed" => "📦",
            "application/json" => "📋",
            "application/xml" => "📰",
            t if t.contains("word") => "📝",
            t if t.contains("excel") | t.contains("sheet") => "📊",
            t if t.contains("powerpoint") | t.contains("presentation") => "📽️",
            _ => "🗂️",
        }
    }

    // Local Storage 관련 메서드들
    fn get_local_storage() -> Option<Storage> {
        window()?.local_storage().ok()?
    }

    fn load_from_storage() -> Self {
        let storage = Self::get_local_storage();
        
        // Hash algorithms 불러오기
        let mut selected = IndexMap::new();
        let default_items = vec!["md5", "sha1", "sha256", "sha512", "crc32"];
        
        if let Some(ref storage) = storage {
            if let Ok(Some(saved_algorithms)) = storage.get_item(STORAGE_KEY_HASH_ALGORITHMS) {
                // 저장된 설정을 파싱
                for item in default_items.iter() {
                    let is_selected = saved_algorithms.contains(&format!("{}:true", item));
                    selected.insert(item.to_string(), is_selected);
                }
            } else {
                // 저장된 설정이 없으면 기본값 사용
                for item in default_items {
                    selected.insert(item.to_string(), true);
                }
            }
        } else {
            // Local Storage가 없으면 기본값 사용
            for item in default_items {
                selected.insert(item.to_string(), true);
            }
        }

        // Output format 불러오기
        let output_format = storage
            .as_ref()
            .and_then(|s| s.get_item(STORAGE_KEY_OUTPUT_FORMAT).ok().flatten())
            .and_then(|s| match s.as_str() {
                "lowercase" => Some(OutputFormat::Lowercase),
                "uppercase" => Some(OutputFormat::Uppercase),
                "colon_separated" => Some(OutputFormat::ColonSeparated),
                "base64" => Some(OutputFormat::Base64),
                "c_style_array" => Some(OutputFormat::CStyleArray),
                _ => None,
            })
            .unwrap_or(OutputFormat::Lowercase);

        // HMAC enabled 불러오기
        let show_hmac_section = storage
            .as_ref()
            .and_then(|s| s.get_item(STORAGE_KEY_HMAC_ENABLED).ok().flatten())
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(false);

        Self {
            file_info: None,
            file_data: None,
            hash_md5: "".to_string(),
            hash_sha1: "".to_string(),
            hash_sha256: "".to_string(),
            hash_sha512: "".to_string(),
            hmac_md5: "".to_string(),
            hmac_sha1: "".to_string(),
            hmac_sha256: "".to_string(),
            hmac_sha512: "".to_string(),
            crc32: "".to_string(),
            is_computing: false,
            step: false,
            progress: 0.0,
            selected,
            error_message: None,
            is_dragging: false,
            expected_hash: String::new(),
            hash_comparison: None,
            show_hash_verification: false,
            output_format,
            show_file_metadata: false,
            hmac_key: String::new(),
            show_hmac_section,
            verification_type: VerificationType::Hash,
            verification_hmac_key: String::new(),
        }
    }

    fn save_to_storage(&self) {
        if let Some(storage) = Self::get_local_storage() {
            // Hash algorithms 저장
            let algorithms_string: String = self.selected
                .iter()
                .map(|(key, &value)| format!("{}:{}", key, value))
                .collect::<Vec<_>>()
                .join(",");
            let _ = storage.set_item(STORAGE_KEY_HASH_ALGORITHMS, &algorithms_string);

            // Output format 저장
            let format_str = match self.output_format {
                OutputFormat::Lowercase => "lowercase",
                OutputFormat::Uppercase => "uppercase",
                OutputFormat::ColonSeparated => "colon_separated",
                OutputFormat::Base64 => "base64",
                OutputFormat::CStyleArray => "c_style_array",
            };
            let _ = storage.set_item(STORAGE_KEY_OUTPUT_FORMAT, format_str);

            // HMAC enabled 저장
            let _ = storage.set_item(STORAGE_KEY_HMAC_ENABLED, &self.show_hmac_section.to_string());
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
