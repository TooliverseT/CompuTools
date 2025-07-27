#[derive(Clone, PartialEq, Debug)]
pub struct ToolInfo {
    pub route_name: String,      // URL에 사용되는 이름 (예: "crc", "base64")
    pub display_name: String,    // 사용자에게 보여지는 이름 (예: "CRC Tool", "Base64 Encoder")
    pub description: String,     // 툴 설명
    pub category: ToolCategory,  // 카테고리
    pub tags: Vec<String>,       // 검색용 태그들
    pub icon: String,           // FontAwesome 아이콘 클래스
}

#[derive(Clone, PartialEq, Debug)]
pub enum ToolCategory {
    SecurityHash,    // 보안 & 해시
    TextEncoding,    // 텍스트 & 인코딩
    Mathematical,    // 수학적 도구
    TimeDate,        // 시간 & 날짜
    Generators,      // 생성기 도구
    WebDevelopment,  // 웹 개발 도구
    // DataProcessing,  // 데이터 처리
}

impl ToolCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            ToolCategory::SecurityHash => "Security & Hash",
            ToolCategory::TextEncoding => "Text & Encoding", 
            ToolCategory::Mathematical => "Mathematical",
            ToolCategory::TimeDate => "Time & Date",
            ToolCategory::Generators => "Generators",
            ToolCategory::WebDevelopment => "Web Development",
            // ToolCategory::DataProcessing => "Data Processing",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ToolCategory::SecurityHash => "Tools for data integrity, hashing, and security verification",
            ToolCategory::TextEncoding => "Text conversion, encoding, and formatting utilities",
            ToolCategory::Mathematical => "Mathematical calculations and number system conversions",
            ToolCategory::TimeDate => "Time and date conversion utilities",
            ToolCategory::Generators => "Random data and identifier generation tools",
            ToolCategory::WebDevelopment => "Web development and URL handling utilities", 
            // ToolCategory::DataProcessing => "Data analysis and processing tools",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ToolCategory::SecurityHash => "fa-solid fa-shield-halved",
            ToolCategory::TextEncoding => "fa-solid fa-font",
            ToolCategory::Mathematical => "fa-solid fa-calculator",
            ToolCategory::TimeDate => "fa-solid fa-clock",
            ToolCategory::Generators => "fa-solid fa-wand-magic-sparkles",
            ToolCategory::WebDevelopment => "fa-solid fa-code",
            // ToolCategory::DataProcessing => "fa-solid fa-chart-line",
        }
    }
}

pub struct ToolCategoryManager;

impl ToolCategoryManager {
    /// 모든 툴의 정보를 반환
    pub fn get_all_tools() -> Vec<ToolInfo> {
        vec![
            // Security & Hash 카테고리
            ToolInfo {
                route_name: "crc".to_string(),
                display_name: "CRC Tool".to_string(),
                description: "Calculate CRC checksums with 100+ algorithms for data integrity verification".to_string(),
                category: ToolCategory::SecurityHash,
                tags: vec!["crc", "checksum", "integrity", "hash", "verify", "calculator", "decoder"].iter().map(|s| s.to_string()).collect(),
                icon: "fa-solid fa-shield-halved".to_string(),
            },
            ToolInfo {
                route_name: "file-hash".to_string(),
                display_name: "File Hash Calculator".to_string(),
                description: "Calculate MD5, SHA-1, SHA-256, SHA-512 hashes for files and text".to_string(),
                category: ToolCategory::SecurityHash,
                tags: vec!["file", "hash", "md5", "sha1", "sha256", "sha512", "integrity", "checksum"].iter().map(|s| s.to_string()).collect(),
                icon: "fa-solid fa-file-shield".to_string(),
            },

            // Text & Encoding 카테고리
            ToolInfo {
                route_name: "base64".to_string(),
                display_name: "Base64 Encoder/Decoder".to_string(),
                description: "Encode and decode Base64 data with support for files and URLs".to_string(),
                category: ToolCategory::TextEncoding,
                tags: vec!["base64", "encode", "decode", "transmission", "data", "file", "url"].iter().map(|s| s.to_string()).collect(),
                icon: "fa-solid fa-key".to_string(),
            },
            ToolInfo {
                route_name: "ascii".to_string(),
                display_name: "ASCII Converter".to_string(),
                description: "Convert text to ASCII codes and vice versa with multiple formats".to_string(),
                category: ToolCategory::TextEncoding,
                tags: vec!["ascii", "text", "code", "convert", "character", "decimal", "hex", "binary"].iter().map(|s| s.to_string()).collect(),
                icon: "fa-solid fa-font".to_string(),
            },
            ToolInfo {
                route_name: "json".to_string(),
                display_name: "JSON Formatter & Converter".to_string(),
                description: "Format, validate, minify and beautify JSON data with error detection".to_string(),
                category: ToolCategory::TextEncoding,
                tags: vec!["json", "format", "validate", "beautify", "parse", "minify", "pretty"].iter().map(|s| s.to_string()).collect(),
                icon: "fa-solid fa-code".to_string(),
            },
            ToolInfo {
                route_name: "html".to_string(),
                display_name: "HTML Entity Converter".to_string(),
                description: "Encode and decode HTML entities for safe web content display".to_string(),
                category: ToolCategory::TextEncoding,
                tags: vec!["html", "encode", "decode", "entities", "web", "escape", "unescape"].iter().map(|s| s.to_string()).collect(),
                icon: "fa-brands fa-html5".to_string(),
            },
            ToolInfo {
                route_name: "url".to_string(),
                display_name: "URL Encoder/Decoder".to_string(),
                description: "Encode and decode URLs for proper web transmission and parsing".to_string(),
                category: ToolCategory::WebDevelopment,
                tags: vec!["url", "encode", "decode", "web", "transmission", "percent", "escape"].iter().map(|s| s.to_string()).collect(),
                icon: "fa-solid fa-link".to_string(),
            },

            // Mathematical 카테고리  
            ToolInfo {
                route_name: "base".to_string(),
                display_name: "Number Base Converter".to_string(),
                description: "Convert numbers between binary, decimal, hexadecimal, and octal with precision".to_string(),
                category: ToolCategory::Mathematical,
                tags: vec!["base", "binary", "hex", "decimal", "octal", "convert", "number", "radix"].iter().map(|s| s.to_string()).collect(),
                icon: "fa-solid fa-calculator".to_string(),
            },
            ToolInfo {
                route_name: "quaternion".to_string(),
                display_name: "Quaternion Calculator".to_string(),
                description: "Convert between quaternions and Euler angles for 3D rotations and animations".to_string(),
                category: ToolCategory::Mathematical,
                tags: vec!["quaternion", "euler", "3d", "rotation", "math", "animation", "game", "graphics"].iter().map(|s| s.to_string()).collect(),
                icon: "fa-solid fa-cube".to_string(),
            },

            // Time & Date 카테고리
            ToolInfo {
                route_name: "unix-timestamp".to_string(),
                display_name: "Unix Timestamp Converter".to_string(),
                description: "Convert between Unix timestamps and human-readable dates with timezone support".to_string(),
                category: ToolCategory::TimeDate,
                tags: vec!["time", "date", "unix", "timestamp", "convert", "epoch", "timezone", "utc"].iter().map(|s| s.to_string()).collect(),
                icon: "fa-solid fa-clock".to_string(),
            },

            // Generators 카테고리
            ToolInfo {
                route_name: "uuid".to_string(),
                display_name: "UUID Generator".to_string(),
                description: "Generate RFC-compliant UUIDs (v4) for unique identification in applications".to_string(),
                category: ToolCategory::Generators,
                tags: vec!["uuid", "generate", "unique", "identifier", "random", "guid", "v4", "rfc"].iter().map(|s| s.to_string()).collect(),
                icon: "fa-solid fa-fingerprint".to_string(),
            },
        ]
    }

    /// 특정 카테고리의 모든 툴을 반환
    pub fn get_tools_by_category(category: &ToolCategory) -> Vec<ToolInfo> {
        Self::get_all_tools()
            .into_iter()
            .filter(|tool| &tool.category == category)
            .collect()
    }

    /// 특정 툴의 카테고리에 속한 다른 툴들을 반환 (자기 자신 제외)
    pub fn get_related_tools(current_tool_route: &str) -> Vec<ToolInfo> {
        let all_tools = Self::get_all_tools();
        
        // 현재 툴의 카테고리를 찾기
        if let Some(current_tool) = all_tools.iter().find(|tool| tool.route_name == current_tool_route) {
            let current_category = current_tool.category.clone();
            
            // 같은 카테고리의 다른 툴들 반환
            all_tools
                .into_iter()
                .filter(|tool| tool.category == current_category && tool.route_name != current_tool_route)
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 특정 툴 정보를 route_name으로 검색
    pub fn get_tool_by_route(route_name: &str) -> Option<ToolInfo> {
        Self::get_all_tools()
            .into_iter()
            .find(|tool| tool.route_name == route_name)
    }

    /// 검색 쿼리에 매칭되는 툴들을 반환
    pub fn search_tools(query: &str) -> Vec<ToolInfo> {
        let query_lower = query.to_lowercase();
        Self::get_all_tools()
            .into_iter()
            .filter(|tool| {
                let display_name_match = tool.display_name.to_lowercase().contains(&query_lower);
                let description_match = tool.description.to_lowercase().contains(&query_lower);
                let tags_match = tool.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower));
                let category_match = tool.category.display_name().to_lowercase().contains(&query_lower);
                
                display_name_match || description_match || tags_match || category_match
            })
            .collect()
    }

    /// 모든 카테고리 목록을 반환
    pub fn get_all_categories() -> Vec<ToolCategory> {
        vec![
            ToolCategory::SecurityHash,
            ToolCategory::TextEncoding,
            ToolCategory::Mathematical,
            ToolCategory::TimeDate,
            ToolCategory::Generators,
            ToolCategory::WebDevelopment,
            // ToolCategory::DataProcessing,
        ]
    }

    /// 카테고리별로 그룹화된 툴들을 반환
    pub fn get_tools_grouped_by_category() -> Vec<(ToolCategory, Vec<ToolInfo>)> {
        let all_tools = Self::get_all_tools();
        let categories = Self::get_all_categories();
        
        categories
            .into_iter()
            .map(|category| {
                let tools: Vec<ToolInfo> = all_tools
                    .iter()
                    .filter(|tool| tool.category == category)
                    .cloned()
                    .collect();
                (category, tools)
            })
            .filter(|(_, tools)| !tools.is_empty()) // 빈 카테고리는 제외
            .collect()
    }

    /// 인기 툴들을 반환 (수동으로 지정된 순서)
    pub fn get_popular_tools() -> Vec<ToolInfo> {
        let popular_routes = vec!["base64", "crc", "json", "ascii", "unix-timestamp", "file-hash"];
        let all_tools = Self::get_all_tools();
        
        popular_routes
            .into_iter()
            .filter_map(|route| all_tools.iter().find(|tool| tool.route_name == route).cloned())
            .collect()
    }

    /// 최근 추가된 툴들을 반환 (수동으로 지정된 순서)
    pub fn get_recent_tools() -> Vec<ToolInfo> {
        let recent_routes = vec!["quaternion", "uuid", "url", "html"];
        let all_tools = Self::get_all_tools();
        
        recent_routes
            .into_iter()
            .filter_map(|route| all_tools.iter().find(|tool| tool.route_name == route).cloned())
            .collect()
    }
} 