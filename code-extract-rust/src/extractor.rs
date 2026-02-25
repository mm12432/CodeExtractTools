use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use encoding_rs_io::DecodeReaderBytesBuilder;

pub struct ExtractionConfig {
    pub remove_comments: bool,
    pub remove_imports: bool,
    pub compact_lines: bool,
}

pub struct ExtractedCode {
    pub content: String,
    pub line_count: usize,
}

pub fn extract_and_clean(path: &Path, config: &ExtractionConfig) -> anyhow::Result<ExtractedCode> {
    // 1. 鲁棒读取：支持自动检测编码 (如 GBK, UTF-16, UTF-8)
    let file = File::open(path)?;
    let mut reader = DecodeReaderBytesBuilder::new()
        .encoding(None) // 自动探测
        .build(file);
    
    let mut raw_content = String::new();
    reader.read_to_string(&mut raw_content)?;

    let mut cleaned_content = String::new();
    let mut line_count = 0;

    // 预编译正则
    // 匹配 //, #, --, ' 类型的单行注释
    let re_comment_single = Regex::new(r"((//|#|--|').*)$")?;
    // 过滤导入/引用
    let re_import = Regex::new(r"^(import|using|include|require|from)\s+.*")?;

    // 注意：目前的逻辑按行处理。暂不支持跨行注释清理（后续可按需扩展）
    for line in raw_content.lines() {
        let mut processed_line = line.to_string();

        // 1. 去注释 (单行末尾注释也一并去掉)
        if config.remove_comments {
            processed_line = re_comment_single.replace(&processed_line, "").to_string();
        }

        let trimmed = processed_line.trim();

        // 2. 去引用/导入
        if config.remove_imports && re_import.is_match(trimmed) {
            continue;
        }

        // 3. 压缩空行
        if config.compact_lines && trimmed.is_empty() {
            continue;
        }

        cleaned_content.push_str(&processed_line);
        cleaned_content.push('\n');
        line_count += 1;
    }

    Ok(ExtractedCode {
        content: cleaned_content,
        line_count,
    })
}
