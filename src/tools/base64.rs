use base64::{decode, encode};
use gloo_file::futures::read_as_bytes;
use gloo_file::File as GlooFile;
use gloo_timers;
use js_sys;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlInputElement, Event, FileList, Blob, BlobPropertyBag, Url, Document, Element, HtmlElement, MouseEvent, DragEvent};
use yew::prelude::*;
use crate::components::tool_category::ToolCategoryManager;

#[derive(Clone, PartialEq)]
pub enum Base64Mode {
    Standard,   // RFC 4648 표준 Base64
    UrlSafe,    // URL-safe Base64 (+ → -, / → _)
    NoPadding,  // 패딩 없는 Base64
}

#[derive(Clone, PartialEq)]
pub enum OutputFormat {
    Continuous,     // 연속된 문자열
    LineBreaks76,   // 76자마다 줄바꿈 (MIME)
    LineBreaks64,   // 64자마다 줄바꿈
    Chunks4,        // 4자마다 공백
    Chunks8,        // 8자마다 공백
    DataUrlImg,     // data:image/[type];base64,[data] for <img>
    DataUrlCss,     // url(data:image/[type];base64,[data]) for CSS
}

pub struct ToolBase64 {
    input_string: String,
    output_base64: String,
    input_base64: String,
    output_string: String,
    convert: bool,
    error_message: Option<String>, // 에러 메시지 추가
    base64_mode: Base64Mode,
    output_format: OutputFormat,
    // 파일 업로드 관련
    uploaded_file: Option<GlooFile>,
    file_content: Option<Vec<u8>>,
    file_info: Option<FileInfo>,
    is_loading: bool,
    // 프로그레스 관련
    processing_progress: f32,
    is_processing: bool,
    processing_chunks: Option<ProcessingState>,
    // 포맷팅 관련
    formatting_state: Option<FormattingState>,
    is_formatting: bool,
    // 디코딩된 이미지 관련
    decoded_image_data: Option<String>, // Data URL for image preview
    decoded_image_mime: Option<String>, // MIME type of decoded image
    decoded_binary_data: Option<Vec<u8>>, // Raw binary data
    // 드래그 앤 드롭 관련
    is_drag_over: bool, // 드래그 오버 상태
}

#[derive(Clone, PartialEq)]
pub struct FileInfo {
    pub name: String,
    pub size: usize,
    pub mime_type: String,
}

#[derive(Clone)]
pub struct ProcessingState {
    pub data: Vec<u8>,
    pub current_chunk: usize,
    pub total_chunks: usize,
    pub chunk_size: usize,
    pub result: String,
}

#[derive(Clone)]
pub struct FormattingState {
    pub base64_result: String,
    pub current_position: usize,
    pub formatted_result: String,
    pub format_type: OutputFormat,
    pub file_info: Option<FileInfo>,
    pub chunks_processed: usize,
    pub max_chunks: usize,
}

pub enum Msg {
    UpdateInput(String),
    UpdateBase64(String),
    Convert,
    CopyToClipboard(String),
    ModeChanged(Base64Mode),
    FormatChanged(OutputFormat),
    FileSelected(Vec<GlooFile>),
    FileProcessed(Vec<u8>, FileInfo),
    ClearFile,
    StartChunkedProcessing(Vec<u8>, FileInfo),
    ProcessNextChunk,
    ChunkedProcessingComplete(String),
    StartChunkedFormatting(String, OutputFormat, Option<FileInfo>),
    ProcessNextFormatChunk,
    FormattingComplete(String),
    DownloadDecodedImage,
    DragOver,
    DragLeave,
    FileDrop(Vec<GlooFile>),
}

impl ToolBase64 {
    fn encode_with_mode(&self, input: &str) -> String {
        let encoded = match self.base64_mode {
            Base64Mode::Standard => encode(input),
            Base64Mode::UrlSafe => {
                // URL-safe Base64: + → -, / → _
                encode(input)
                    .chars()
                    .map(|c| match c {
                        '+' => '-',
                        '/' => '_',
                        other => other,
                    })
                    .collect()
            }
            Base64Mode::NoPadding => {
                // 패딩 제거
                encode(input).trim_end_matches('=').to_string()
            }
        };

        self.format_output_with_file_info(&encoded, None)
    }

    fn decode_with_mode(&self, input: &str) -> Result<Vec<u8>, String> {
        // 포맷팅 제거 (공백, 줄바꿈 등)
        let cleaned_input = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        let standardized_input = match self.base64_mode {
            Base64Mode::Standard => cleaned_input,
            Base64Mode::UrlSafe => {
                // URL-safe를 표준으로 변환: - → +, _ → /
                cleaned_input
                    .chars()
                    .map(|c| match c {
                        '-' => '+',
                        '_' => '/',
                        other => other,
                    })
                    .collect()
            }
            Base64Mode::NoPadding => {
                // 패딩 추가
                let mut padded = cleaned_input;
                while padded.len() % 4 != 0 {
                    padded.push('=');
                }
                padded
            }
        };

        decode(&standardized_input).map_err(|_| "Failed to decode Base64".to_string())
    }

    fn format_output(&self, input: &str) -> String {
        match self.output_format {
            OutputFormat::Continuous => input.to_string(),
            OutputFormat::LineBreaks76 => {
                input
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(76)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join("\n")
            }
            OutputFormat::LineBreaks64 => {
                input
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(64)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join("\n")
            }
            OutputFormat::Chunks4 => {
                input
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(4)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join(" ")
            }
            OutputFormat::Chunks8 => {
                input
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(8)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join(" ")
            }
            OutputFormat::DataUrlImg | OutputFormat::DataUrlCss => {
                // 파일 컨텍스트에서만 사용됨, 여기서는 기본 처리
                input.to_string()
            }
        }
    }

    fn format_output_with_file_info(&self, input: &str, file_info: Option<&FileInfo>) -> String {
        match self.output_format {
            OutputFormat::DataUrlImg => {
                if let Some(file_info) = file_info {
                    format!("data:{};base64,{}", file_info.mime_type, input)
                } else {
                    format!("data:text/plain;base64,{}", input)
                }
            }
            OutputFormat::DataUrlCss => {
                if let Some(file_info) = file_info {
                    format!("url(data:{};base64,{})", file_info.mime_type, input)
                } else {
                    format!("url(data:text/plain;base64,{})", input)
                }
            }
            _ => self.format_output(input)
        }
    }

    fn validate_base64_input(&self, input: &str) -> Result<(), String> {
        if input.trim().is_empty() {
            return Ok(());
        }

        // 포맷팅 제거 (공백, 줄바꿈 등)
        let cleaned_input = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        // Base64 모드별 유효한 문자 집합
        let valid_chars = match self.base64_mode {
            Base64Mode::Standard => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=",
            Base64Mode::UrlSafe => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_=",
            Base64Mode::NoPadding => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
        };

        // 길이 검증 (NoPadding 모드는 예외)
        if self.base64_mode != Base64Mode::NoPadding && cleaned_input.len() % 4 != 0 {
            return Err(format!("Invalid Base64 length: {} characters. Base64 strings must be multiples of 4 characters.", cleaned_input.len()));
        }

        // 문자 집합 검증
        for (index, ch) in cleaned_input.chars().enumerate() {
            if !valid_chars.contains(ch) {
                let mode_name = match self.base64_mode {
                    Base64Mode::Standard => "Standard Base64",
                    Base64Mode::UrlSafe => "URL-safe Base64",
                    Base64Mode::NoPadding => "Base64 without padding",
                };
                return Err(format!("Invalid character '{}' at position {} for {}. Allowed characters: {}", ch, index + 1, mode_name, valid_chars));
            }
        }

        // 패딩 검증 (Standard와 UrlSafe 모드)
        if self.base64_mode != Base64Mode::NoPadding {
            let padding_start = cleaned_input.find('=');
            if let Some(start_pos) = padding_start {
                let padding_part = &cleaned_input[start_pos..];
                if !padding_part.chars().all(|c| c == '=') {
                    return Err("Invalid padding: '=' characters can only appear at the end of Base64 string.".to_string());
                }
                
                if padding_part.len() > 2 {
                    return Err(format!("Invalid padding: too many '=' characters ({}). Maximum allowed is 2.", padding_part.len()));
                }
            }
        }

        Ok(())
    }

    fn get_detailed_base64_error(&self, input: &str) -> String {
        // 더 구체적인 에러 분석
        if input.trim().is_empty() {
            return "Input is empty.".to_string();
        }

        let input = input.trim();
        
        // 길이 검사
        if input.len() % 4 != 0 {
            let missing = 4 - (input.len() % 4);
            return format!("Invalid length: {} characters. Need {} more character(s) to make it a multiple of 4.", input.len(), missing);
        }

        // 잘못된 문자 찾기
        let valid_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
        for (index, ch) in input.chars().enumerate() {
            if !valid_chars.contains(ch) {
                if ch.is_whitespace() {
                    return format!("Whitespace character found at position {}. Remove all spaces, tabs, and newlines.", index + 1);
                } else if ch.is_ascii_punctuation() && ch != '+' && ch != '/' && ch != '=' {
                    return format!("Invalid punctuation '{}' at position {}. Only '+', '/', and '=' are allowed.", ch, index + 1);
                } else {
                    return format!("Invalid character '{}' at position {}. Use only A-Z, a-z, 0-9, +, /, =", ch, index + 1);
                }
            }
        }

        "Unknown Base64 format error.".to_string()
    }

    fn format_file_size(size: usize) -> String {
        if size < 1024 {
            format!("{} B", size)
        } else if size < 1024 * 1024 {
            format!("{:.1} KB", size as f64 / 1024.0)
        } else if size < 1024 * 1024 * 1024 {
            format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }

    fn get_file_icon(mime_type: &str) -> &str {
        if mime_type.starts_with("image/") {
            "🖼️"
        } else if mime_type == "application/pdf" {
            "📄"
        } else if mime_type.starts_with("text/") {
            "📝"
        } else if mime_type.starts_with("audio/") {
            "🎵"
        } else if mime_type.starts_with("video/") {
            "🎬"
        } else if mime_type.contains("zip") || mime_type.contains("rar") || mime_type.contains("tar") {
            "📦"
        } else {
            "📁"
        }
    }

    fn encode_file_content(&self, content: &[u8]) -> String {
        let encoded = match self.base64_mode {
            Base64Mode::Standard => encode(content),
            Base64Mode::UrlSafe => {
                encode(content)
                    .chars()
                    .map(|c| match c {
                        '+' => '-',
                        '/' => '_',
                        other => other,
                    })
                    .collect()
            }
            Base64Mode::NoPadding => {
                encode(content).trim_end_matches('=').to_string()
            }
        };

        self.format_output_with_file_info(&encoded, self.file_info.as_ref())
    }

    // 대용량 파일을 위한 청크 단위 처리
    const CHUNK_SIZE: usize = 64 * 1024; // 64KB 청크
    const LARGE_FILE_THRESHOLD: usize = 1024 * 1024; // 1MB 이상은 청크 처리
    const MAX_FILE_SIZE: usize = 2 * 1024 * 1024; // 2MB 최대 파일 크기 제한

    fn should_use_chunked_processing(&self, size: usize) -> bool {
        size > Self::LARGE_FILE_THRESHOLD
    }

    fn encode_chunk(&self, chunk: &[u8]) -> String {
        match self.base64_mode {
            Base64Mode::Standard => encode(chunk),
            Base64Mode::UrlSafe => {
                encode(chunk)
                    .chars()
                    .map(|c| match c {
                        '+' => '-',
                        '/' => '_',
                        other => other,
                    })
                    .collect()
            }
            Base64Mode::NoPadding => {
                encode(chunk).trim_end_matches('=').to_string()
            }
        }
    }

    fn create_processing_state(&self, data: Vec<u8>) -> ProcessingState {
        let total_chunks = (data.len() + Self::CHUNK_SIZE - 1) / Self::CHUNK_SIZE;
        ProcessingState {
            data,
            current_chunk: 0,
            total_chunks,
            chunk_size: Self::CHUNK_SIZE,
            result: String::new(),
        }
    }

    // 대용량 결과를 위한 청크 단위 포맷팅
    const FORMAT_CHUNK_SIZE: usize = 256 * 1024; // 256KB 청크로 포맷팅 (더 작게)
    const LARGE_RESULT_THRESHOLD: usize = 1024 * 1024; // 1MB 이상은 청크 포맷팅 (더 보수적)

    fn should_use_chunked_formatting(&self, size: usize) -> bool {
        size > Self::LARGE_RESULT_THRESHOLD && 
        (self.output_format == OutputFormat::LineBreaks76 || 
         self.output_format == OutputFormat::LineBreaks64 ||
         self.output_format == OutputFormat::Chunks4 ||
         self.output_format == OutputFormat::Chunks8)
    }

    fn create_formatting_state(&self, base64_result: String, format_type: OutputFormat, file_info: Option<FileInfo>) -> FormattingState {
        let max_chunks = (base64_result.len() + Self::FORMAT_CHUNK_SIZE - 1) / Self::FORMAT_CHUNK_SIZE;
        FormattingState {
            base64_result,
            current_position: 0,
            formatted_result: String::new(),
            format_type,
            file_info,
            chunks_processed: 0,
            max_chunks,
        }
    }

    fn format_chunk(&self, chunk: &str, format_type: &OutputFormat) -> String {
        match format_type {
            OutputFormat::LineBreaks76 => {
                chunk
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(76)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join("\n")
            }
            OutputFormat::LineBreaks64 => {
                chunk
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(64)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join("\n")
            }
            OutputFormat::Chunks4 => {
                chunk
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(4)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join(" ")
            }
            OutputFormat::Chunks8 => {
                chunk
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(8)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join(" ")
            }
            _ => chunk.to_string(),
        }
    }

    fn is_valid_image_data(&self, data: &[u8]) -> Option<String> {
        // 이미지 파일 시그니처 검사
        if data.len() < 4 {
            return None;
        }
        
        // PNG
        if data.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
            return Some("image/png".to_string());
        }
        
        // JPEG
        if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
            return Some("image/jpeg".to_string());
        }
        
        // GIF
        if data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a") {
            return Some("image/gif".to_string());
        }
        
        // WebP
        if data.len() >= 12 && &data[0..4] == b"RIFF" && &data[8..12] == b"WEBP" {
            return Some("image/webp".to_string());
        }
        
        // BMP
        if data.starts_with(b"BM") {
            return Some("image/bmp".to_string());
        }
        
        // ICO
        if data.starts_with(&[0x00, 0x00, 0x01, 0x00]) {
            return Some("image/x-icon".to_string());
        }
        
        None
    }
    
    fn create_image_data_url(&self, data: &[u8], mime_type: &str) -> String {
        let base64_data = encode(data);
        format!("data:{};base64,{}", mime_type, base64_data)
    }

    fn parse_data_url(&self, input: &str) -> Option<(String, String)> {
        // data: URL 형식 파싱
        // data:image/png;base64,iVBORw0KGgo... 또는
        // url(data:image/png;base64,iVBORw0KGgo...)
        
        let cleaned_input = input.trim();
        
        // CSS url() 래퍼 제거
        let data_part = if cleaned_input.starts_with("url(") && cleaned_input.ends_with(")") {
            &cleaned_input[4..cleaned_input.len()-1]
        } else {
            cleaned_input
        };
        
        // data: URL 형식 확인
        if !data_part.starts_with("data:") {
            return None;
        }
        
        // data: 이후 부분 파싱
        let without_data = &data_part[5..]; // "data:" 제거
        
        // MIME 타입과 base64 데이터 분리
        if let Some(comma_pos) = without_data.find(',') {
            let header = &without_data[..comma_pos];
            let base64_data = &without_data[comma_pos + 1..];
            
            // MIME 타입 추출 (;base64 제거)
            let mime_type = if let Some(semicolon_pos) = header.find(';') {
                header[..semicolon_pos].to_string()
            } else {
                header.to_string()
            };
            
            // base64 키워드 확인
            if header.contains("base64") {
                return Some((mime_type, base64_data.to_string()));
            }
        }
        
        None
    }
}

impl Component for ToolBase64 {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_string: String::new(),
            output_base64: String::new(),
            input_base64: String::new(),
            output_string: String::new(),
            convert: false,
            error_message: None,
            base64_mode: Base64Mode::Standard,
            output_format: OutputFormat::Continuous,
            // 파일 업로드 관련
            uploaded_file: None,
            file_content: None,
            file_info: None,
            is_loading: false,
            // 프로그레스 관련
            processing_progress: 0.0,
            is_processing: false,
            processing_chunks: None,
            // 포맷팅 관련
            formatting_state: None,
            is_formatting: false,
            // 디코딩된 이미지 관련
            decoded_image_data: None, // Data URL for image preview
            decoded_image_mime: None, // MIME type of decoded image
            decoded_binary_data: None, // Raw binary data
            // 드래그 앤 드롭 관련
            is_drag_over: false, // 드래그 오버 상태
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateInput(value) => {
                self.input_string = value;
                self.error_message = None; // 에러 메시지 초기화
                self.output_base64 = self.encode_with_mode(&self.input_string);
                true
            }
            Msg::UpdateBase64(value) => {
                self.input_base64 = value.clone();
                self.error_message = None; // 에러 메시지 초기화
                
                // 디코딩된 이미지 데이터 초기화
                self.decoded_image_data = None;
                self.decoded_image_mime = None;
                self.decoded_binary_data = None;

                // 입력값이 비어있으면 출력도 초기화
                if value.trim().is_empty() {
                    self.output_string = String::new();
                    return true;
                }

                // 먼저 데이터 URL 형식인지 확인
                if let Some((mime_type, base64_data)) = self.parse_data_url(&value) {
                    // 데이터 URL 형식인 경우
                    match self.decode_with_mode(&base64_data) {
                        Ok(decoded) => {
                            // MIME 타입이 이미지인지 확인하거나 바이너리 시그니처로 재확인
                            let detected_mime = if mime_type.starts_with("image/") {
                                Some(mime_type.clone())
                            } else {
                                self.is_valid_image_data(&decoded)
                            };
                            
                            if let Some(final_mime_type) = detected_mime {
                                // 이미지인 경우
                                self.decoded_image_mime = Some(final_mime_type.clone());
                                self.decoded_binary_data = Some(decoded.clone());
                                self.decoded_image_data = Some(self.create_image_data_url(&decoded, &final_mime_type));
                                self.output_string = format!("Data URL Image detected: {} ({} bytes)", final_mime_type, decoded.len());
                            } else {
                                // 텍스트인 경우
                                self.output_string = format!("Data URL decoded: {} ({} bytes)\n{}", 
                                    mime_type, 
                                    decoded.len(),
                                    String::from_utf8_lossy(&decoded).to_string()
                                );
                            }
                        }
                        Err(_) => {
                            self.error_message = Some("Invalid Base64 data in Data URL".to_string());
                            self.output_string = String::new();
                        }
                    }
                } else {
                    // 일반 Base64 문자열인 경우 기존 로직 사용
                    // 실시간 Base64 검증
                    match self.validate_base64_input(&value) {
                        Ok(_) => {
                            // 검증 통과 시 디코딩 시도
                            match self.decode_with_mode(&value) {
                                Ok(decoded) => {
                                    // 이미지 데이터인지 확인
                                    if let Some(mime_type) = self.is_valid_image_data(&decoded) {
                                        // 이미지인 경우
                                        self.decoded_image_mime = Some(mime_type.clone());
                                        self.decoded_binary_data = Some(decoded.clone());
                                        self.decoded_image_data = Some(self.create_image_data_url(&decoded, &mime_type));
                                        self.output_string = format!("Image detected: {} ({} bytes)", mime_type, decoded.len());
                                    } else {
                                        // 텍스트인 경우
                                        self.output_string = String::from_utf8_lossy(&decoded).to_string();
                                    }
                                }
                                Err(_) => {
                                    // 디코딩 실패 시 구체적인 에러 메시지
                                    self.error_message = Some(self.get_detailed_base64_error(&value));
                                    self.output_string = String::new();
                                }
                            }
                        }
                        Err(error_msg) => {
                            // 검증 실패 시 에러 메시지 설정
                            self.error_message = Some(error_msg);
                            self.output_string = String::new();
                        }
                    }
                }
                
                true
            }
            Msg::Convert => {
                self.convert = !self.convert;
                self.error_message = None; // 모드 변경 시 에러 메시지 초기화
                
                // 디코딩된 이미지 데이터 초기화
                self.decoded_image_data = None;
                self.decoded_image_mime = None;
                self.decoded_binary_data = None;
                
                true
            }
            Msg::ModeChanged(mode) => {
                self.base64_mode = mode;
                self.error_message = None;
                
                // 디코딩된 이미지 데이터 초기화
                self.decoded_image_data = None;
                self.decoded_image_mime = None;
                self.decoded_binary_data = None;
                
                // 청크 처리 또는 포맷팅 중이면 처리하지 않음
                if self.is_processing || self.is_formatting {
                    return true;
                }
                
                // 현재 입력값에 따라 재변환
                if !self.convert {
                    // Text to Base64 모드
                    if !self.input_string.is_empty() {
                        self.output_base64 = self.encode_with_mode(&self.input_string);
                    } else if let Some(content) = &self.file_content {
                        // 파일이 업로드된 경우
                        if self.should_use_chunked_processing(content.len()) {
                            // 대용량 파일은 청크 처리
                            let file_info = self.file_info.clone().unwrap();
                            _ctx.link().send_message(Msg::StartChunkedProcessing(content.clone(), file_info));
                        } else {
                            self.output_base64 = self.encode_file_content(content);
                        }
                    }
                } else if self.convert && !self.input_base64.is_empty() {
                    // Base64 to Text 모드 - 재검증 및 변환
                    // 먼저 데이터 URL 형식인지 확인
                    if let Some((mime_type, base64_data)) = self.parse_data_url(&self.input_base64) {
                        // 데이터 URL 형식인 경우
                        match self.decode_with_mode(&base64_data) {
                            Ok(decoded) => {
                                // MIME 타입이 이미지인지 확인하거나 바이너리 시그니처로 재확인
                                let detected_mime = if mime_type.starts_with("image/") {
                                    Some(mime_type.clone())
                                } else {
                                    self.is_valid_image_data(&decoded)
                                };
                                
                                if let Some(final_mime_type) = detected_mime {
                                    // 이미지인 경우
                                    self.decoded_image_mime = Some(final_mime_type.clone());
                                    self.decoded_binary_data = Some(decoded.clone());
                                    self.decoded_image_data = Some(self.create_image_data_url(&decoded, &final_mime_type));
                                    self.output_string = format!("Data URL Image detected: {} ({} bytes)", final_mime_type, decoded.len());
                                } else {
                                    // 텍스트인 경우
                                    self.output_string = format!("Data URL decoded: {} ({} bytes)\n{}", 
                                        mime_type, 
                                        decoded.len(),
                                        String::from_utf8_lossy(&decoded).to_string()
                                    );
                                }
                            }
                            Err(_) => {
                                self.error_message = Some("Invalid Base64 data in Data URL".to_string());
                                self.output_string = String::new();
                            }
                        }
                    } else {
                        // 일반 Base64 문자열인 경우 기존 로직 사용
                        match self.decode_with_mode(&self.input_base64) {
                            Ok(decoded) => {
                                // 이미지 데이터인지 확인
                                if let Some(mime_type) = self.is_valid_image_data(&decoded) {
                                    // 이미지인 경우
                                    self.decoded_image_mime = Some(mime_type.clone());
                                    self.decoded_binary_data = Some(decoded.clone());
                                    self.decoded_image_data = Some(self.create_image_data_url(&decoded, &mime_type));
                                    self.output_string = format!("Image detected: {} ({} bytes)", mime_type, decoded.len());
                                } else {
                                    // 텍스트인 경우
                                    self.output_string = String::from_utf8_lossy(&decoded).to_string();
                                }
                            }
                            Err(error_msg) => {
                                self.error_message = Some(error_msg);
                                self.output_string = String::new();
                            }
                        }
                    }
                }
                true
            }
            Msg::FormatChanged(format) => {
                self.output_format = format;
                
                // 청크 처리 또는 포맷팅 중이면 처리하지 않음
                if self.is_processing || self.is_formatting {
                    return true;
                }
                
                // Encode to Base64 모드일 때만 출력 포맷 재적용
                if !self.convert && !self.input_string.is_empty() {
                    self.output_base64 = self.encode_with_mode(&self.input_string);
                } else if !self.convert && self.file_content.is_some() {
                    // 파일이 업로드된 상태에서 포맷 변경
                    if let Some(content) = &self.file_content {
                        if self.should_use_chunked_processing(content.len()) {
                            // 대용량 파일은 청크 처리
                            let file_info = self.file_info.clone().unwrap();
                            _ctx.link().send_message(Msg::StartChunkedProcessing(content.clone(), file_info));
                        } else {
                            self.output_base64 = self.encode_file_content(content);
                        }
                    }
                }
                true
            }
            Msg::FileSelected(files) => {
                if let Some(file) = files.into_iter().next() {
                    // 파일 크기 검사
                    let file_size = file.size() as usize;
                    if file_size > Self::MAX_FILE_SIZE {
                        self.error_message = Some(format!(
                            "File size too large. Maximum {} supported. (Current file: {})",
                            Self::format_file_size(Self::MAX_FILE_SIZE),
                            Self::format_file_size(file_size)
                        ));
                        return true;
                    }
                    
                    let file_info = FileInfo {
                        name: file.name(),
                        size: file_size,
                        mime_type: file.raw_mime_type(),
                    };
                    
                    self.uploaded_file = Some(file.clone());
                    self.file_info = Some(file_info.clone());
                    self.is_loading = true;
                    self.error_message = None;
                    
                    // 파일을 비동기로 읽기
                    let link = _ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match read_as_bytes(&file).await {
                            Ok(bytes) => {
                                link.send_message(Msg::FileProcessed(bytes, file_info));
                            }
                            Err(_) => {
                                // 에러 처리는 FileProcessed에서 빈 벡터로 처리
                                link.send_message(Msg::FileProcessed(vec![], file_info));
                            }
                        }
                    });
                }
                true
            }
            Msg::FileProcessed(bytes, file_info) => {
                self.is_loading = false;
                
                if bytes.is_empty() {
                    self.error_message = Some("Failed to read file".to_string());
                    self.uploaded_file = None;
                    self.file_info = None;
                    self.file_content = None;
                } else {
                    self.file_content = Some(bytes.clone());
                    self.file_info = Some(file_info.clone());
                    
                    // Encode to Base64 모드에서 파일 인코딩
                    if !self.convert {
                        // 대용량 파일인지 확인
                        if self.should_use_chunked_processing(bytes.len()) {
                            // 청크 처리 시작
                            _ctx.link().send_message(Msg::StartChunkedProcessing(bytes, file_info));
                        } else {
                            // 작은 파일은 즉시 처리
                            self.output_base64 = self.encode_file_content(&bytes);
                        }
                        // 텍스트 입력 초기화
                        self.input_string.clear();
                    }
                }
                true
            }
            Msg::ClearFile => {
                self.uploaded_file = None;
                self.file_content = None;
                self.file_info = None;
                self.is_loading = false;
                self.error_message = None;
                
                // 프로그레스 상태 초기화
                self.is_processing = false;
                self.processing_progress = 0.0;
                self.processing_chunks = None;
                
                // 포맷팅 상태 초기화
                self.is_formatting = false;
                self.formatting_state = None;
                
                // Encode to Base64 모드에서 출력 초기화
                if !self.convert {
                    self.output_base64.clear();
                }
                true
            }
            Msg::StartChunkedProcessing(data, file_info) => {
                self.is_processing = true;
                self.processing_progress = 0.0;
                self.file_info = Some(file_info);
                self.processing_chunks = Some(self.create_processing_state(data));
                
                // 첫 번째 청크 처리 시작
                _ctx.link().send_message(Msg::ProcessNextChunk);
                true
            }
            Msg::ProcessNextChunk => {
                if let Some(mut state) = self.processing_chunks.take() {
                    if state.current_chunk < state.total_chunks {
                        // 현재 청크 처리
                        let start = state.current_chunk * state.chunk_size;
                        let end = std::cmp::min(start + state.chunk_size, state.data.len());
                        let chunk = &state.data[start..end];
                        
                        let encoded_chunk = self.encode_chunk(chunk);
                        state.result.push_str(&encoded_chunk);
                        
                        state.current_chunk += 1;
                        self.processing_progress = (state.current_chunk as f32) / (state.total_chunks as f32);
                        
                        if state.current_chunk < state.total_chunks {
                            // 더 처리할 청크가 있으면 상태 저장 후 다음 청크 예약
                            self.processing_chunks = Some(state);
                            
                            // requestAnimationFrame을 사용하여 브라우저가 렌더링할 시간을 줌
                            let link = _ctx.link().clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                gloo_timers::future::TimeoutFuture::new(1).await; // 1ms 지연
                                link.send_message(Msg::ProcessNextChunk);
                            });
                        } else {
                            // 모든 청크 처리 완료
                            _ctx.link().send_message(Msg::ChunkedProcessingComplete(state.result));
                        }
                    }
                }
                true
            }
            Msg::ChunkedProcessingComplete(result) => {
                self.is_processing = false;
                self.processing_progress = 1.0;
                self.processing_chunks = None;
                
                // 포맷팅이 필요하지 않은 경우 즉시 완료
                if self.output_format == OutputFormat::Continuous ||
                   self.output_format == OutputFormat::DataUrlImg ||
                   self.output_format == OutputFormat::DataUrlCss ||
                   !self.should_use_chunked_formatting(result.len()) {
                    
                    // 즉시 포맷 적용
                    let formatted_result = if let Some(file_info) = &self.file_info {
                        self.format_output_with_file_info(&result, Some(file_info))
                    } else {
                        self.format_output(&result)
                    };
                    self.output_base64 = formatted_result;
                } else {
                    // 결과가 큰 경우에만 청크 포맷팅 시작
                    _ctx.link().send_message(Msg::StartChunkedFormatting(
                        result, 
                        self.output_format.clone(), 
                        self.file_info.clone()
                    ));
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
            Msg::StartChunkedFormatting(base64_result, format_type, file_info) => {
                self.is_formatting = true;
                self.processing_progress = 0.95; // 95%에서 시작 (인코딩은 완료됨)
                self.formatting_state = Some(self.create_formatting_state(base64_result, format_type, file_info));
                
                // 첫 번째 포맷 청크 처리 시작
                _ctx.link().send_message(Msg::ProcessNextFormatChunk);
                true
            }
            Msg::ProcessNextFormatChunk => {
                if let Some(mut state) = self.formatting_state.take() {
                    let remaining = state.base64_result.len() - state.current_position;
                    
                    // 안전장치: 무한 루프 방지
                    if remaining == 0 || 
                       state.current_position >= state.base64_result.len() ||
                       state.chunks_processed >= state.max_chunks * 2 { // 최대 청크의 2배로 제한
                        
                        // 포맷팅 완료 또는 안전장치 발동
                        let final_result = if let Some(file_info) = &state.file_info {
                            // Data URL 포맷팅이 필요한 경우
                            match state.format_type {
                                OutputFormat::DataUrlImg => {
                                    format!("data:{};base64,{}", file_info.mime_type, state.formatted_result)
                                }
                                OutputFormat::DataUrlCss => {
                                    format!("url(data:{};base64,{})", file_info.mime_type, state.formatted_result)
                                }
                                _ => state.formatted_result
                            }
                        } else {
                            state.formatted_result
                        };
                        
                        _ctx.link().send_message(Msg::FormattingComplete(final_result));
                        return true;
                    }
                    
                    // 청크 카운터 증가
                    state.chunks_processed += 1;
                    
                    // 현재 청크 크기 결정 (더 작게)
                    let chunk_size = std::cmp::min(Self::FORMAT_CHUNK_SIZE, remaining);
                    let end_pos = state.current_position + chunk_size;
                    
                    // 청크 추출 및 포맷팅
                    let chunk = &state.base64_result[state.current_position..end_pos];
                    let formatted_chunk = self.format_chunk(chunk, &state.format_type);
                    
                    // 구분자 추가 (필요한 경우만)
                    if !state.formatted_result.is_empty() {
                        match state.format_type {
                            OutputFormat::LineBreaks76 | OutputFormat::LineBreaks64 => {
                                if !formatted_chunk.is_empty() {
                                    state.formatted_result.push('\n');
                                }
                            }
                            OutputFormat::Chunks4 | OutputFormat::Chunks8 => {
                                if !formatted_chunk.is_empty() {
                                    state.formatted_result.push(' ');
                                }
                            }
                            _ => {}
                        }
                    }
                    
                    state.formatted_result.push_str(&formatted_chunk);
                    state.current_position = end_pos;
                    
                    // 진행률 업데이트 (95% ~ 100%)
                    let format_progress = (state.current_position as f32) / (state.base64_result.len() as f32);
                    self.processing_progress = 0.95 + (format_progress * 0.05);
                    
                    if state.current_position < state.base64_result.len() {
                        // 더 처리할 데이터가 있으면 상태 저장 후 다음 청크 예약
                        self.formatting_state = Some(state);
                        
                        // 더 긴 지연 시간으로 브라우저에 더 많은 시간 제공
                        let link = _ctx.link().clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            gloo_timers::future::TimeoutFuture::new(10).await; // 10ms로 더 증가
                            link.send_message(Msg::ProcessNextFormatChunk);
                        });
                    } else {
                        // 포맷팅 완료
                        let final_result = if let Some(file_info) = &state.file_info {
                            // Data URL 포맷팅이 필요한 경우
                            match state.format_type {
                                OutputFormat::DataUrlImg => {
                                    format!("data:{};base64,{}", file_info.mime_type, state.formatted_result)
                                }
                                OutputFormat::DataUrlCss => {
                                    format!("url(data:{};base64,{})", file_info.mime_type, state.formatted_result)
                                }
                                _ => state.formatted_result
                            }
                        } else {
                            state.formatted_result
                        };
                        
                        _ctx.link().send_message(Msg::FormattingComplete(final_result));
                    }
                }
                true
            }
            Msg::FormattingComplete(formatted_result) => {
                self.is_formatting = false;
                self.processing_progress = 1.0;
                self.formatting_state = None;
                self.output_base64 = formatted_result;
                true
            }
            Msg::DownloadDecodedImage => {
                if let (Some(binary_data), Some(mime_type)) = (&self.decoded_binary_data, &self.decoded_image_mime) {
                    // 파일 확장자 결정
                    let extension = match mime_type.as_str() {
                        "image/png" => "png",
                        "image/jpeg" => "jpg",
                        "image/gif" => "gif",
                        "image/webp" => "webp",
                        "image/bmp" => "bmp",
                        "image/x-icon" => "ico",
                        _ => "bin",
                    };
                    
                    let filename = format!("decoded_image.{}", extension);
                    
                    if let Some(window) = window() {
                        // Uint8Array 생성
                        let uint8_array = js_sys::Uint8Array::new_with_length(binary_data.len() as u32);
                        uint8_array.copy_from(binary_data);
                        
                        // Blob 생성
                        let blob_parts = js_sys::Array::new();
                        blob_parts.push(&uint8_array);
                        
                        let mut blob_options = BlobPropertyBag::new();
                        blob_options.set_type(mime_type);
                        
                        if let Ok(blob) = Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_options) {
                            // URL 생성
                            if let Ok(url) = Url::create_object_url_with_blob(&blob) {
                                // 다운로드 링크 생성 및 클릭
                                if let Some(document) = window.document() {
                                    if let Ok(anchor) = document.create_element("a") {
                                        if let Ok(anchor) = anchor.dyn_into::<Element>() {
                                            let _ = anchor.set_attribute("href", &url);
                                            let _ = anchor.set_attribute("download", &filename);
                                            let _ = anchor.set_attribute("style", "display: none;");
                                            
                                            if let Some(body) = document.body() {
                                                let _ = body.append_child(&anchor);
                                                
                                                // HTMLElement로 캐스팅하여 click 메서드 호출
                                                if let Ok(html_anchor) = anchor.clone().dyn_into::<web_sys::HtmlElement>() {
                                                    html_anchor.click();
                                                }
                                                
                                                let _ = body.remove_child(&anchor);
                                            }
                                            
                                            // URL 정리
                                            let _ = Url::revoke_object_url(&url);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                false // 리렌더링 필요 없음
            }
            Msg::DragOver => {
                self.is_drag_over = true;
                true
            }
            Msg::DragLeave => {
                self.is_drag_over = false;
                true
            }
            Msg::FileDrop(files) => {
                self.is_drag_over = false;
                
                // FileSelected와 동일한 로직 사용
                if let Some(file) = files.into_iter().next() {
                    // 파일 크기 검사
                    let file_size = file.size() as usize;
                    if file_size > Self::MAX_FILE_SIZE {
                        self.error_message = Some(format!(
                            "File size too large. Maximum {} supported. (Current file: {})",
                            Self::format_file_size(Self::MAX_FILE_SIZE),
                            Self::format_file_size(file_size)
                        ));
                        return true;
                    }
                    
                    let file_info = FileInfo {
                        name: file.name(),
                        size: file_size,
                        mime_type: file.raw_mime_type(),
                    };
                    
                    self.uploaded_file = Some(file.clone());
                    self.file_info = Some(file_info.clone());
                    self.is_loading = true;
                    self.error_message = None;
                    
                    // 파일을 비동기로 읽기
                    let link = _ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match read_as_bytes(&file).await {
                            Ok(bytes) => {
                                link.send_message(Msg::FileProcessed(bytes, file_info));
                            }
                            Err(_) => {
                                // 에러 처리는 FileProcessed에서 빈 벡터로 처리
                                link.send_message(Msg::FileProcessed(vec![], file_info));
                            }
                        }
                    });
                }
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let convert = self.convert.clone();
        let on_convert = _ctx.link().callback(|_| Msg::Convert);

        html! {
            <>
                        <h1 class="tool-title">
                            { "Base64 Encoder/Decoder" }
                        </h1>
                <div class="tool-wrapper">
                        <div class="tool-intro">
                        <div class="content-section">
                            <h2>{"🔤 What is Base64?"}</h2>
                            <p>{"Base64 is a binary-to-text encoding scheme that represents binary data in an ASCII string format by translating it into a radix-64 representation. It uses 64 printable ASCII characters (A-Z, a-z, 0-9, +, /) to encode any type of binary data, making it safe for transmission over text-based protocols."}</p>
                            <p>{"Base64 encoding is essential in modern web development for embedding images directly in HTML/CSS, transmitting files through APIs, storing binary data in JSON/XML, and ensuring safe email attachments via MIME encoding. It's also widely used in authentication tokens, data URLs, and configuration files."}</p>
                            <div class="example-box" style="margin-top: 15px;">
                                <p><strong>{"Real-world applications:"}</strong></p>
                                <ul style="margin: 5px 0; padding-left: 20px;">
                                    <li>{"Embedding images in emails and web pages"}</li>
                                    <li>{"Storing files in databases as text"}</li>
                                    <li>{"API data transmission and JWT tokens"}</li>
                                    <li>{"Configuration files and data serialization"}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"⚙️ How This Base64 Encoder/Decoder Works"}</h2>
                            <p>{"This comprehensive Base64 tool provides bidirectional conversion with advanced features for both text and binary data. It supports real-time processing, multiple Base64 variants, and intelligent handling of large files with progress tracking."}</p>
                            
                            <h3>{"🚀 Core Features:"}</h3>
                            <ul>
                                <li><strong>{"Bidirectional Conversion:"}</strong> {"Seamlessly encode any data to Base64 or decode Base64 back to original format."}</li>
                                <li><strong>{"Multi-Format Support:"}</strong> {"Handle text, images, documents, audio, video, and any binary file type."}</li>
                                <li><strong>{"Real-Time Processing:"}</strong> {"Instant conversion as you type or upload files with live validation."}</li>
                                <li><strong>{"Drag & Drop Interface:"}</strong> {"Modern file upload with visual feedback and smart file detection."}</li>
                            </ul>

                            <h3>{"📁 File Processing Capabilities:"}</h3>
                            <ul>
                                <li><strong>{"Universal File Support:"}</strong> {"Images (PNG, JPG, GIF, WebP, BMP, ICO), documents (PDF, DOC, TXT), archives (ZIP, RAR), media files, and more."}</li>
                                <li><strong>{"Smart File Detection:"}</strong> {"Automatic MIME type recognition with appropriate file icons and metadata display."}</li>
                                <li><strong>{"Large File Optimization:"}</strong> {"Intelligent chunked processing for files >1MB with real-time progress bars to prevent browser freezing."}</li>
                                <li><strong>{"Size Validation:"}</strong> {"Built-in 2MB file size limit with clear error messaging for optimal performance."}</li>
                            </ul>

                            <h3>{"🔧 Advanced Base64 Features:"}</h3>
                            <ul>
                                <li><strong>{"Multiple Base64 Variants:"}</strong> {"Standard RFC 4648, URL-Safe (- and _ instead of + and /), and No-Padding formats."}</li>
                                <li><strong>{"Flexible Output Formatting:"}</strong> {"Continuous string, MIME (76-char lines), 64-char lines, or chunked (4/8-char) output."}</li>
                                <li><strong>{"Data URL Generation:"}</strong> {"Automatic creation of ready-to-use data URLs for HTML <img> tags and CSS background properties."}</li>
                                <li><strong>{"Smart Input Parsing:"}</strong> {"Handles existing data URLs (data:image/...;base64,... or url(data:...)) and extracts Base64 content."}</li>
                            </ul>

                            <h3>{"🎯 Intelligent Decoding:"}</h3>
                            <ul>
                                <li><strong>{"Auto-Format Detection:"}</strong> {"Automatically detects and handles Base64 strings, data URLs, and CSS url() formats."}</li>
                                <li><strong>{"Image Recognition:"}</strong> {"Smart detection of decoded images with automatic preview and download capabilities."}</li>
                                <li><strong>{"Binary vs Text Analysis:"}</strong> {"Intelligently determines whether decoded data is text or binary with appropriate display."}</li>
                                <li><strong>{"Error Recovery:"}</strong> {"Robust error handling with detailed validation messages and position-specific guidance."}</li>
                            </ul>

                            <h3>{"⚡ Performance & User Experience:"}</h3>
                            <ul>
                                <li><strong>{"Chunked Processing:"}</strong> {"Large files are processed in 64KB chunks to maintain browser responsiveness."}</li>
                                <li><strong>{"Progress Tracking:"}</strong> {"Real-time progress bars for encoding (0-95%) and formatting (95-100%) phases."}</li>
                                <li><strong>{"Local Processing:"}</strong> {"All operations happen in your browser - no data is sent to external servers."}</li>
                                <li><strong>{"Copy-to-Clipboard:"}</strong> {"One-click copying of results with visual feedback for instant workflow integration."}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"💡 Common Use Cases & Examples"}</h2>
                            
                            <div class="use-case">
                                <h3>{"1. 🖼️ Web Development & Image Embedding"}</h3>
                                <ul>
                                    <li><strong>{"Inline Image Embedding:"}</strong> {"Convert images to data URLs for immediate display without separate HTTP requests."}</li>
                                    <li><strong>{"CSS Background Images:"}</strong> {"Generate CSS-ready data URLs for background images and icons."}</li>
                                    <li><strong>{"Email Templates:"}</strong> {"Embed images directly in HTML emails to ensure consistent display across clients."}</li>
                                    <li><strong>{"Offline Applications:"}</strong> {"Bundle images as Base64 for applications that work without internet connectivity."}</li>
                                </ul>
                                <div class="example-box">
                                    <p><strong>{"Example: Image to HTML/CSS"}</strong></p>
                                    <ul>
                                        <li>{"Upload: logo.png (2.1KB)"}</li>
                                        <li>{"HTML: <img src=\"data:image/png;base64,iVBORw0KGgo...\" />"}</li>
                                        <li>{"CSS: background-image: url(data:image/png;base64,iVBORw0KGgo...);"}</li>
                                    </ul>
                                </div>
                            </div>

                            <div class="use-case">
                                <h3>{"2. 🔗 API Development & Data Transfer"}</h3>
                                <ul>
                                    <li><strong>{"File Upload APIs:"}</strong> {"Send files through JSON APIs by encoding them as Base64 strings."}</li>
                                    <li><strong>{"Database Storage:"}</strong> {"Store binary files in text-based database fields."}</li>
                                    <li><strong>{"Configuration Files:"}</strong> {"Embed binary resources in JSON, YAML, or XML configurations."}</li>
                                    <li><strong>{"Authentication Tokens:"}</strong> {"Handle JWT tokens and OAuth credentials that use Base64 encoding."}</li>
                                </ul>
                                <div class="example-box">
                                    <p><strong>{"Example: File Upload API"}</strong></p>
                                    <ul>
                                        <li>
                                            {"{\"filename\": \"document.pdf\", \"content\": \"JVBERi0xLjQK...\", \"mime_type\": \"application/pdf\"}"}
                                        </li>
                                    </ul>
                                </div>
                            </div>

                            <div class="use-case">
                                <h3>{"3. 📧 Email & Communication"}</h3>
                                <ul>
                                    <li><strong>{"MIME Attachments:"}</strong> {"Encode file attachments for email transmission using MIME format."}</li>
                                    <li><strong>{"Rich Text Emails:"}</strong> {"Embed images and documents directly in email content."}</li>
                                    <li><strong>{"Cross-Platform Messaging:"}</strong> {"Ensure binary data integrity across different messaging systems."}</li>
                                    <li><strong>{"Newsletter Graphics:"}</strong> {"Include images that display reliably across all email clients."}</li>
                                </ul>
                            </div>

                            <div class="use-case">
                                <h3>{"4. 🔧 Development & Testing"}</h3>
                                <ul>
                                    <li><strong>{"Quick Data Conversion:"}</strong> {"Convert test files and sample data for development workflows."}</li>
                                    <li><strong>{"Debugging Data URLs:"}</strong> {"Decode existing data URLs to inspect their content and format."}</li>
                                    <li><strong>{"Mock Data Creation:"}</strong> {"Generate Base64 content for testing and prototyping."}</li>
                                    <li><strong>{"Cross-Platform Compatibility:"}</strong> {"Ensure data integrity when moving between different systems."}</li>
                                </ul>
                            </div>

                            <div class="use-case">
                                <h3>{"5. 📱 Mobile & Progressive Web Apps"}</h3>
                                <ul>
                                    <li><strong>{"Offline Resources:"}</strong> {"Bundle essential images and files as Base64 for offline functionality."}</li>
                                    <li><strong>{"App Store Assets:"}</strong> {"Convert icons and splash screens for app deployment."}</li>
                                    <li><strong>{"Reduced HTTP Requests:"}</strong> {"Improve app performance by embedding small assets directly."}</li>
                                    <li><strong>{"PWA Manifests:"}</strong> {"Include encoded icons in Progressive Web App configurations."}</li>
                                </ul>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"📚 Step-by-Step Tutorial"}</h2>
                            
                            <div class="tutorial-step">
                                <h3>{"Example 1: 📝 Encoding Text to Base64"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Convert text content to Base64 with different formatting options."}</p>
                                <ol>
                                    <li>{"Ensure the tool is in 'Encode to Base64' mode (default)."}</li>
                                    <li>{"Enter your text in the 'Data Input' field: 'Hello, World! 🌍'"}</li>
                                    <li>{"Select your preferred Base64 mode (Standard, URL-Safe, or No Padding)."}</li>
                                    <li>{"Choose output format (Continuous, MIME, or chunked)."}</li>
                                    <li>{"View the encoded result instantly and click to copy."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Input:"}</strong> {"Hello, World! 🌍"}</p>
                                    <p><strong>{"Standard Base64:"}</strong> {"SGVsbG8sIFdvcmxkISDwn42N"}</p>
                                    <p><strong>{"URL-Safe Base64:"}</strong> {"SGVsbG8sIFdvcmxkISDwn42N"}</p>
                                    <p><strong>{"MIME Format (76 chars):"}</strong> {"SGVsbG8sIFdvcmxkISDwn42N"}</p>
                                </div>
                            </div>

                            <div class="tutorial-step">
                                <h3>{"Example 2: 🖼️ Converting Images to Data URLs"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Create ready-to-use HTML and CSS data URLs from image files."}</p>
                                <ol>
                                    <li>{"Ensure you're in 'Encode to Base64' mode."}</li>
                                    <li>{"Drag and drop an image file (PNG, JPG, GIF, etc.) into the upload area."}</li>
                                    <li>{"Wait for file processing (progress bar shows for large files)."}</li>
                                    <li>{"Select 'Data URL (for <img>)' from the Output Format dropdown."}</li>
                                    <li>{"Copy the complete data URL for direct use in HTML."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"File:"}</strong> {"logo.png (2.3 KB) 🖼️"}</p>
                                    <p><strong>{"HTML Data URL:"}</strong> {"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQ..."}</p>
                                    <p><strong>{"CSS Data URL:"}</strong> {"url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQ...)"}</p>
                                    <p><strong>{"Usage in HTML:"}</strong></p>
                                    <ul>
                                        <li>
                                            {"<img src=\"data:image/png;base64,iVBORw0KGgo...\" alt=\"Logo\" />"}
                                        </li>
                                    </ul>
                                </div>
                            </div>

                            <div class="tutorial-step">
                                <h3>{"Example 3: 🔍 Decoding Base64 with Smart Detection"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Decode various Base64 formats including data URLs and detect content types."}</p>
                                <ol>
                                    <li>{"Switch to 'Decode from Base64' mode by clicking the rotate icon (⟲)."}</li>
                                    <li>{"Paste any Base64 string, data URL, or CSS url() format."}</li>
                                    <li>{"The tool automatically detects the format and extracts Base64 content."}</li>
                                    <li>{"View decoded text or image preview with download option."}</li>
                                    <li>{"For images, use the download button to save the decoded file."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Input Types Supported:"}</strong></p>
                                    <ul>
                                        <li>{"Pure Base64: 'SGVsbG8gV29ybGQ='"}</li>
                                        <li>{"Data URL: 'data:image/png;base64,iVBORw0KGgo...'"}</li>
                                        <li>{"CSS Format: 'url(data:image/png;base64,iVBORw0KGgo...)'"}</li>
                                    </ul>
                                    <p><strong>{"Smart Detection:"}</strong> {"🖼️ Images show preview + download | 📝 Text shows content"}</p>
                                </div>
                            </div>

                            <div class="tutorial-step">
                                <h3>{"Example 4: 📁 Processing Large Files with Progress Tracking"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Handle large files efficiently with real-time progress monitoring."}</p>
                                <ol>
                                    <li>{"Upload a large file (>1MB) via drag & drop or file selector."}</li>
                                    <li>{"Watch the file size validation (2MB limit for optimal performance)."}</li>
                                    <li>{"Monitor the progress bar: Processing (0-95%) → Formatting (95-100%)."}</li>
                                    <li>{"Large results are formatted in chunks to prevent browser freezing."}</li>
                                    <li>{"Copy the final result when processing completes."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Large File Example:"}</strong> {"document.pdf (1.8 MB) 📄"}</p>
                                    <p><strong>{"Processing:"}</strong> {"64KB chunks → Base64 encoding → Output formatting"}</p>
                                    <p><strong>{"Progress:"}</strong> {"Real-time percentage and status updates"}</p>
                                    <p><strong>{"Result:"}</strong> {"MIME-formatted Base64 ready for email/API transmission"}</p>
                                </div>
                            </div>

                            <div class="tutorial-step">
                                <h3>{"Example 5: 🔧 Advanced Features & Error Handling"}</h3>
                                <p><strong>{"Goal:"}</strong> {"Utilize advanced features and understand error recovery."}</p>
                                <ol>
                                    <li>{"Test different Base64 modes: Standard vs URL-Safe vs No-Padding."}</li>
                                    <li>{"Try various output formats: Continuous, MIME, Chunked."}</li>
                                    <li>{"Experience error handling with invalid Base64 input."}</li>
                                    <li>{"Use copy-to-clipboard functionality for workflow integration."}</li>
                                    <li>{"Observe file type detection and MIME type recognition."}</li>
                                </ol>
                                <div class="example-box">
                                    <p><strong>{"Error Examples:"}</strong></p>
                                    <ul>
                                        <li>{"Invalid character: 'SGVsbG8@V29ybGQ=' → Position 7 error"}</li>
                                        <li>{"Wrong length: 'SGVsbG8=' → Missing padding suggestion"}</li>
                                        <li>{"File too large: 'file.zip (3MB)' → Size limit warning"}</li>
                                    </ul>
                                    <p><strong>{"Smart Features:"}</strong> {"🔍 Auto-detection | ⚡ Real-time validation | 📋 One-click copy"}</p>
                                </div>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🔧 Technical Background"}</h2>
                            
                            <h3>{"📐 How Base64 Encoding Works"}</h3>
                            <p>{"Base64 encoding converts 3 bytes (24 bits) of binary data into 4 ASCII characters (6 bits each). Each 6-bit group maps to one of 64 printable characters: A-Z (0-25), a-z (26-51), 0-9 (52-61), + (62), / (63). Padding with '=' ensures output length is always a multiple of 4."}</p>
                            <div class="example-box">
                                <p><strong>{"Encoding Process Example:"}</strong></p>
                                <ul>
                                    <li>{"Text: 'Man' → ASCII: [77, 97, 110]"}</li>
                                    <li>{"Binary: 01001101 01100001 01101110"}</li>
                                    <li>{"6-bit groups: 010011 010110 000101 101110"}</li>
                                    <li>{"Base64 indices: [19, 22, 5, 46]"}</li>
                                    <li>{"Base64 result: 'TWFu'"}</li>
                                </ul>
                            </div>

                            <h3>{"⚡ Performance Optimizations"}</h3>
                            <ul>
                                <li><strong>{"Chunked Processing:"}</strong> {"Large files (>1MB) are processed in 64KB chunks to prevent browser freezing and maintain UI responsiveness."}</li>
                                <li><strong>{"Async Operations:"}</strong> {"File reading and encoding use Web Workers concepts via spawn_local for non-blocking processing."}</li>
                                <li><strong>{"Memory Management:"}</strong> {"Progressive processing reduces peak memory usage for large files."}</li>
                                <li><strong>{"Browser Compatibility:"}</strong> {"Uses modern Web APIs (File API, Blob, DataTransfer) with fallback handling."}</li>
                            </ul>

                            <h3>{"🔍 Smart Detection Algorithms"}</h3>
                            <ul>
                                <li><strong>{"File Type Recognition:"}</strong> {"Binary signature analysis detects PNG, JPEG, GIF, WebP, BMP, ICO, and other formats by examining file headers."}</li>
                                <li><strong>{"Data URL Parsing:"}</strong> {"Regex-based parsing handles data:, url(data:), and various MIME type formats automatically."}</li>
                                <li><strong>{"Format Validation:"}</strong> {"Real-time Base64 validation with position-specific error reporting and correction suggestions."}</li>
                                <li><strong>{"MIME Type Detection:"}</strong> {"Automatic MIME type assignment based on file signatures and extensions."}</li>
                            </ul>

                            <h3>{"🛡️ Security & Privacy"}</h3>
                            <ul>
                                <li><strong>{"Local Processing:"}</strong> {"All encoding/decoding happens entirely in your browser - no data transmitted to external servers."}</li>
                                <li><strong>{"No Data Persistence:"}</strong> {"Files and content are processed in memory only, not saved or cached."}</li>
                                <li><strong>{"Size Limitations:"}</strong> {"2MB file limit prevents potential memory exhaustion and ensures optimal performance."}</li>
                                <li><strong>{"Input Validation:"}</strong> {"Comprehensive validation prevents processing of malformed or potentially harmful inputs."}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"❓ Frequently Asked Questions"}</h2>
                            
                            <div class="faq-item">
                                <h3>{"Q: What file types can I upload and convert?"}</h3>
                                <p>{"A: You can upload ANY file type - images (PNG, JPG, GIF, WebP, BMP, ICO), documents (PDF, DOC, TXT), archives (ZIP, RAR), audio/video files, executables, and more. The tool automatically detects file types, displays appropriate icons, and handles all binary data correctly."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: What's the difference between Base64 modes?"}</h3>
                                <p>{"A: Standard Base64 uses +, / and = padding. URL-Safe replaces + with - and / with _ for safe use in URLs. No-Padding removes = characters entirely. Each mode produces different but valid Base64 output for the same input data."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: Can I decode data URLs and CSS url() formats?"}</h3>
                                <p>{"A: Yes! The tool intelligently parses data:image/png;base64,... and url(data:...) formats, automatically extracts the Base64 content, detects MIME types, and provides appropriate previews for images or text output for other data."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: Why does large file processing show a progress bar?"}</h3>
                                <p>{"A: Files over 1MB are processed in 64KB chunks to maintain browser responsiveness. The progress bar shows encoding (0-95%) and formatting (95-100%) phases. This prevents browser freezing and allows you to monitor processing of large files."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: Is there a file size limit?"}</h3>
                                <p>{"A: Yes, files are limited to 2MB maximum for optimal performance. This prevents browser memory issues and ensures consistent performance across all devices. Files >1MB are automatically processed with progress tracking."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: How do I use Data URLs in HTML and CSS?"}</h3>
                                <p>{"A: For HTML: <img src=\"data:image/png;base64,iVBORw0KGgo...\" />. For CSS: background-image: url(data:image/png;base64,iVBORw0KGgo...). The tool automatically generates the correct format when you select 'Data URL (for <img>)' or 'Data URL (for CSS)'."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: Are my files secure and private?"}</h3>
                                <p>{"A: Absolutely! All processing happens locally in your browser. No files or data are uploaded to external servers. Everything is processed in your device's memory and nothing is stored or transmitted elsewhere."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: What if I get Base64 validation errors?"}</h3>
                                <p>{"A: The tool provides detailed error messages with exact position information. Common issues: invalid characters (only A-Z, a-z, 0-9, +, /, = allowed), incorrect length (must be multiple of 4), or malformed padding. Follow the specific guidance provided."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: Can I download decoded images?"}</h3>
                                <p>{"A: Yes! When you decode Base64 that contains image data, the tool automatically detects it, shows a preview, and provides a download button to save the image file with the correct extension and MIME type."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: What output formats are available?"}</h3>
                                <p>{"A: Choose from Continuous (single line), MIME format (76 chars/line for email), 64 chars/line, Chunked (4 or 8 char groups), or Data URLs for direct HTML/CSS usage. Each format is optimized for different use cases."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: Why use Base64 instead of regular file uploads?"}</h3>
                                <p>{"A: Base64 is essential for: embedding resources directly in HTML/CSS/JSON, transmitting binary data through text-based APIs, storing files in databases as text, ensuring data integrity in email attachments, and creating self-contained documents."}</p>
                            </div>

                            <div class="faq-item">
                                <h3>{"Q: Does Base64 increase file size?"}</h3>
                                <p>{"A: Yes, Base64 encoding increases size by approximately 33% due to the encoding overhead. A 100KB file becomes ~133KB when Base64-encoded. Consider this when embedding large files in documents or APIs."}</p>
                            </div>
                        </div>

                        <div class="content-section">
                            <h2>{"🎯 Best Practices & Tips"}</h2>
                            <ul>
                                <li><strong>{"Optimal File Sizes:"}</strong> {"Use Data URLs for files <10KB to minimize HTTP requests. For larger files, consider traditional file serving for better caching."}</li>
                                <li><strong>{"Choose the Right Mode:"}</strong> {"Use Standard Base64 for general purposes, URL-Safe for URLs/filenames, and No-Padding for systems that don't handle padding well."}</li>
                                <li><strong>{"Output Format Selection:"}</strong> {"MIME format for emails, Continuous for APIs, Data URLs for direct embedding, Chunked for readability."}</li>
                                <li><strong>{"Error Prevention:"}</strong> {"Always validate Base64 input before using in production. The tool's real-time validation helps catch issues early."}</li>
                                <li><strong>{"Performance Considerations:"}</strong> {"Process large files during off-peak times. Use progress tracking for user feedback on lengthy operations."}</li>
                                <li><strong>{"Security Awareness:"}</strong> {"Base64 is encoding, not encryption. Don't use it for sensitive data protection - use proper encryption instead."}</li>
                                <li><strong>{"Browser Compatibility:"}</strong> {"Data URLs work in all modern browsers, but very long URLs (>2MB) may hit browser-specific limits."}</li>
                                <li><strong>{"Workflow Integration:"}</strong> {"Use the copy-to-clipboard feature for seamless integration with your development workflow and documentation."}</li>
                            </ul>
                        </div>

                        <div class="content-section">
                            <h2>{"🔗 Related Tools"}</h2>
                            <ul>
                                {
                                    ToolCategoryManager::get_related_tools("base64")
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
                                if !convert {
                                    {"Encode to Base64"}
                                } else {
                                    {"Decode from Base64"}
                                }
                            </div>
                            <div onclick={on_convert} class="tool-change" style="width: 10%; display: flex; justify-content: center;">
                                <i class="fa-solid fa-arrows-rotate"></i>
                            </div>
                        </div>
                        
                        // Base64 모드 선택
                        <div style="display: flex; align-items: center; margin-bottom: 10px;">
                            <div style="width: 50%; margin-right: 10px;">
                                <label style="margin-right: 8px; font-size: 14px;">{"Base64 Mode:"}</label>
                                <select
                                    style="width: 100%;"
                                    onchange={_ctx.link().callback(|e: Event| {
                                        let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                        match value.as_str() {
                                            "standard" => Msg::ModeChanged(Base64Mode::Standard),
                                            "urlsafe" => Msg::ModeChanged(Base64Mode::UrlSafe),
                                            "nopadding" => Msg::ModeChanged(Base64Mode::NoPadding),
                                            _ => unreachable!(),
                                        }
                                    })}>
                                    <option value="standard" selected={self.base64_mode == Base64Mode::Standard}>{ "Standard" }</option>
                                    <option value="urlsafe" selected={self.base64_mode == Base64Mode::UrlSafe}>{ "URL-Safe" }</option>
                                    <option value="nopadding" selected={self.base64_mode == Base64Mode::NoPadding}>{ "No Padding" }</option>
                                </select>
                            </div>
                            
                            // 출력 포맷 선택 (Encode to Base64 모드일 때만)
                            if !convert {
                                <div style="width: 50%;">
                                    <label style="margin-right: 8px; font-size: 14px;">{"Output Format:"}</label>
                                    <select
                                        style="width: 100%;"
                                        onchange={_ctx.link().callback(|e: Event| {
                                            let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                            match value.as_str() {
                                                "continuous" => Msg::FormatChanged(OutputFormat::Continuous),
                                                "linebreaks76" => Msg::FormatChanged(OutputFormat::LineBreaks76),
                                                "linebreaks64" => Msg::FormatChanged(OutputFormat::LineBreaks64),
                                                "chunks4" => Msg::FormatChanged(OutputFormat::Chunks4),
                                                "chunks8" => Msg::FormatChanged(OutputFormat::Chunks8),
                                                "dataurlimg" => Msg::FormatChanged(OutputFormat::DataUrlImg),
                                                "dataurlcss" => Msg::FormatChanged(OutputFormat::DataUrlCss),
                                                _ => unreachable!(),
                                            }
                                        })}>
                                        <option value="continuous" selected={self.output_format == OutputFormat::Continuous}>{ "Continuous" }</option>
                                        <option value="linebreaks76" selected={self.output_format == OutputFormat::LineBreaks76}>{ "MIME (76 chars)" }</option>
                                        <option value="linebreaks64" selected={self.output_format == OutputFormat::LineBreaks64}>{ "64 chars/line" }</option>
                                        <option value="chunks4" selected={self.output_format == OutputFormat::Chunks4}>{ "4-char chunks" }</option>
                                        <option value="chunks8" selected={self.output_format == OutputFormat::Chunks8}>{ "8-char chunks" }</option>
                                        <option value="dataurlimg" selected={self.output_format == OutputFormat::DataUrlImg}>{ "Data URL (for <img>)" }</option>
                                        <option value="dataurlcss" selected={self.output_format == OutputFormat::DataUrlCss}>{ "Data URL (for CSS)" }</option>
                                    </select>
                                </div>
                            }
                        </div>
                        if !convert {
                            <div class="tool-inner">
                                <div>
                                    <div class="tool-subtitle" style="margin-bottom: 5px;">{ "Data Input" }</div>
                                    
                                    // 파일 업로드 영역
                                    <div 
                                        style={format!("margin-bottom: 10px; border: 2px dashed {}; border-radius: 8px; padding: 15px; text-align: center; transition: all 0.2s ease;{}", 
                                            if self.is_drag_over { "var(--color-primary)" } else { "var(--color-border)" },
                                            if self.is_drag_over { " background-color: rgba(var(--color-primary-rgb), 0.1);" } else { "" }
                                        )}
                                        ondragover={_ctx.link().callback(|e: DragEvent| {
                                            e.prevent_default();
                                            Msg::DragOver
                                        })}
                                        ondragenter={_ctx.link().callback(|e: DragEvent| {
                                            e.prevent_default();
                                            Msg::DragOver
                                        })}
                                        ondragleave={_ctx.link().callback(|e: DragEvent| {
                                            e.prevent_default();
                                            Msg::DragLeave
                                        })}
                                        ondrop={_ctx.link().callback(|e: DragEvent| {
                                            e.prevent_default();
                                            let mut file_list = Vec::new();
                                            
                                            // wasm-bindgen을 통해 dataTransfer.files에 접근
                                            let event_obj = wasm_bindgen::JsValue::from(e);
                                            if let Ok(data_transfer) = js_sys::Reflect::get(&event_obj, &"dataTransfer".into()) {
                                                if let Ok(files) = js_sys::Reflect::get(&data_transfer, &"files".into()) {
                                                    if let Ok(file_list_obj) = files.dyn_into::<web_sys::FileList>() {
                                                        for i in 0..file_list_obj.length() {
                                                            if let Some(file) = file_list_obj.get(i) {
                                                                file_list.push(GlooFile::from(file));
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Msg::FileDrop(file_list)
                                        })}
                                    >
                                        if let Some(file_info) = &self.file_info {
                                            // 파일이 업로드된 상태
                                            <div style="display: flex; align-items: center; justify-content: space-between;">
                                                <div style="display: flex; align-items: center; flex: 1;">
                                                    <span style="font-size: 20px; margin-right: 8px;">
                                                        { Self::get_file_icon(&file_info.mime_type) }
                                                    </span>
                                                    <div style="text-align: left;">
                                                        <div style="font-weight: bold; color: var(--color-font);">
                                                            { &file_info.name }
                                                        </div>
                                                        <div style="font-size: 12px; color: var(--color-subfont);">
                                                            { format!("{} • {}", Self::format_file_size(file_info.size), &file_info.mime_type) }
                                                        </div>
                                                    </div>
                                                </div>
                                                <button 
                                                    type="button"
                                                    style="background: var(--color-error); color: white; border: none; border-radius: 4px; padding: 5px 10px; cursor: pointer;"
                                                    onclick={_ctx.link().callback(|_| Msg::ClearFile)}>
                                                    { "Remove" }
                                                </button>
                                            </div>
                                            if self.is_loading {
                                                <div style="margin-top: 10px; color: var(--color-subfont);">
                                                    <i class="fa-solid fa-spinner fa-spin"></i> { " Processing file..." }
                                                </div>
                                            }
                                            
                                            // 청크 처리 프로그레스바
                                            if self.is_processing || self.is_formatting {
                                                <div style="margin-top: 10px;">
                                                    <div style="display: flex; align-items: center; margin-bottom: 5px;">
                                                        <span style="color: var(--color-subfont); font-size: 12px; margin-right: 10px;">
                                                            if self.is_formatting {
                                                                { format!("Formatting... {:.0}%", self.processing_progress * 100.0) }
                                                            } else {
                                                                { format!("Processing... {:.0}%", self.processing_progress * 100.0) }
                                                            }
                                                        </span>
                                                    </div>
                                                    <div style="width: 100%; background-color: var(--color-border); border-radius: 4px; height: 8px; overflow: hidden;">
                                                        <div style={format!("width: {:.1}%; background-color: var(--color-primary); height: 100%; transition: width 0.2s ease;", self.processing_progress * 100.0)}></div>
                                                    </div>
                                                    <div style="color: var(--color-subfont); font-size: 11px; margin-top: 2px;">
                                                        if self.is_formatting {
                                                            { "Applying output formatting to large result - almost done!" }
                                                        } else {
                                                            { "Large file detected - processing in chunks to prevent browser freeze" }
                                                        }
                                                    </div>
                                                </div>
                                            }
                                        } else {
                                            // 파일 업로드 대기 상태
                                            <div>
                                                <div style="margin-bottom: 10px;">
                                                    <i class="fa-solid fa-cloud-upload-alt" style="font-size: 24px; color: var(--color-subfont);"></i>
                                                </div>
                                                <div style="margin-bottom: 8px; font-weight: bold; color: var(--color-primary);">
                                                    { "Drop files here or click to upload" }
                                                </div>
                                                <div style="font-size: 12px; color: var(--color-subfont); margin-bottom: 10px;">
                                                    { format!("Supports images, PDFs, text files, and more (Max: {})", Self::format_file_size(Self::MAX_FILE_SIZE)) }
                                                </div>
                                                <input
                                                    type="file"
                                                    id="file-upload"
                                                    style="display: none;"
                                                    onchange={_ctx.link().callback(|e: Event| {
                                                        let input: HtmlInputElement = e.target_unchecked_into();
                                                        let files = input.files();
                                                        let mut file_list = Vec::new();
                                                        
                                                        if let Some(files) = files {
                                                            for i in 0..files.length() {
                                                                if let Some(file) = files.get(i) {
                                                                    file_list.push(GlooFile::from(file));
                                                                }
                                                            }
                                                        }
                                                        Msg::FileSelected(file_list)
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

                                    // 파일 크기 제한 에러 메시지 표시
                                    if let Some(error_msg) = &self.error_message {
                                        if !self.convert && error_msg.contains("File size too large") {
                                            <div style="color: var(--color-error); font-size: 12px; margin-bottom: 10px; padding: 8px; background-color: rgba(255, 0, 0, 0.1); border-radius: 4px; line-height: 1.3;">
                                                <i class="fa-solid fa-exclamation-triangle" style="margin-right: 5px;"></i>
                                                { error_msg }
                                            </div>
                                        }
                                    }

                                    // 텍스트 입력 (파일이 없을 때만)
                                    if self.file_info.is_none() {
                                        <div style="margin-bottom: 5px; color: var(--color-subfont); font-size: 12px;">
                                            { "Or enter text manually:" }
                                        </div>
                                    <textarea
                                        type="text"
                                        style="overflow-y: auto;"
                                        value={self.input_string.clone()}
                                        placeholder={ "Enter text..."}
                                        oninput={_ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::UpdateInput(input.value())
                                        })}
                                    />
                                    }
                                </div>
                            </div>
                            <div class="tool-inner" style="margin-top: 10px;">
                                <div>
                                    <div class="tool-subtitle">{ "Base64 Output" }</div>
                                    <textarea
                                        type="text"
                                        readonly=true
                                        style="overflow-y: auto; cursor: pointer;"
                                        value={self.output_base64.clone()}
                                        onclick={_ctx.link().callback(|e: MouseEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::CopyToClipboard(input.value())
                                        })}
                                    />
                                </div>
                            </div>
                        } else {
                            <div class="tool-inner">
                                <div>
                                    <div class="tool-subtitle" style="margin-bottom: 5px;">{ "Base64 Input" }</div>
                                    <textarea
                                        type="text"
                                        style={if self.error_message.is_some() { 
                                            "overflow-y: auto; border: 2px solid var(--color-error);" 
                                        } else { 
                                            "overflow-y: auto;" 
                                        }}
                                        value={self.input_base64.clone()}
                                        placeholder={ "Enter base64 string or data URL..."}
                                        oninput={_ctx.link().callback(|e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::UpdateBase64(input.value())
                                        })}
                                    />
                                    if let Some(error_msg) = &self.error_message {
                                        <div style="color: var(--color-error); font-size: 12px; margin-top: 4px; line-height: 1.3;">
                                            { error_msg }
                                        </div>
                                    }
                                    <div style="color: var(--color-subfont); font-size: 11px; margin-top: 2px;">
                                        {"Supports: Base64 strings, data:image/...;base64,... or url(data:image/...;base64,...)"}
                                    </div>
                                </div>
                            </div>
                            <div class="tool-inner" style="margin-top: 10px;">
                                <div>
                                    <div class="tool-subtitle">{ "Decoded Output" }</div>
                                    
                                    // 이미지가 감지된 경우 미리보기 표시
                                    if let Some(image_data) = &self.decoded_image_data {
                                        <div style="margin-bottom: 15px;">
                                            <div style="margin-bottom: 8px; font-size: 12px; color: var(--color-subfont);">
                                                { "Image Preview:" }
                                            </div>
                                            <div style="border: 1px solid var(--color-border); border-radius: 8px; padding: 10px; background-color: var(--color-background-secondary); text-align: center;">
                                                <img 
                                                    src={image_data.clone()} 
                                                    alt="Decoded image"
                                                    style="max-width: 100%; max-height: 300px; border-radius: 4px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);"
                                                />
                                            </div>
                                            if let Some(mime_type) = &self.decoded_image_mime {
                                                <div style="margin-top: 5px; font-size: 11px; color: var(--color-subfont); text-align: center;">
                                                    { format!("Type: {}", mime_type) }
                                                    if let Some(binary_data) = &self.decoded_binary_data {
                                                        { format!(" • Size: {}", Self::format_file_size(binary_data.len())) }
                                                    }
                                                </div>
                                            }
                                            <div style="margin-top: 10px; text-align: center;">
                                                <button 
                                                    type="button"
                                                    style="background: var(--color-primary); color: white; border: none; border-radius: 4px; padding: 8px 16px; cursor: pointer; font-size: 12px;"
                                                    onclick={_ctx.link().callback(|_| Msg::DownloadDecodedImage)}>
                                                    <i class="fa-solid fa-download" style="margin-right: 5px;"></i>
                                                    { "Download Image" }
                                                </button>
                                            </div>
                                        </div>
                                    }
                                    
                                    <textarea
                                        type="text"
                                        readonly=true
                                        style="overflow-y: auto; cursor: pointer;"
                                        value={self.output_string.clone()}
                                        onclick={_ctx.link().callback(|e: MouseEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::CopyToClipboard(input.value())
                                        })}
                                    />
                                </div>
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
                    doc.set_title("Base64 Encoder/Decoder | CompuTools");

                    if let Some(meta_tag) =
                        doc.query_selector("meta[name=\"description\"]").unwrap()
                    {
                        meta_tag.set_attribute("content", "Free online Base64 encoder decoder tool. Convert text, images, files to Base64 and decode Base64 back to original format. Supports all file types, drag & drop upload, and multiple Base64 variants. Fast, secure, and works offline in your browser.").unwrap();
                    }
                }
            }
        }
    }
}
