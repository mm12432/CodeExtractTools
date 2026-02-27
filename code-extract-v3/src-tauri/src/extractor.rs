use encoding_rs_io::DecodeReaderBytesBuilder;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use docx_rs::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExtractionConfig {
    pub remove_comments: bool,
    pub remove_imports: bool,
    pub compact_lines: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExtractedCode {
    pub content: String,
    pub line_count: usize,
}

pub fn extract_and_clean(path_str: &str, config: &ExtractionConfig) -> anyhow::Result<ExtractedCode> {
    let path = Path::new(path_str);
    let file = File::open(path)?;
    
    // Robust reading: supports auto detecting encodings (like GBK, UTF-16, UTF-8)
    let mut reader = DecodeReaderBytesBuilder::new()
        .encoding(None) 
        .build(file);

    let mut raw_content = String::new();
    reader.read_to_string(&mut raw_content)?;

    let mut cleaned_content = String::new();
    let mut line_count = 0;

    // 安全模式：获取文件扩展名，准备特定语言的精准匹配
    let ext = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();

    // 编译导入声明移除规则
    let re_import = Regex::new(r"^(import|using|include|require|from)\s+.*")?;

    for line in raw_content.lines() {
        let mut processed_line = line.to_string();
        let trimmed = line.trim();

        if config.remove_comments {
            // 抛弃危险的行内截断（防止因带有 'a 生命周期或 url 的 // 造成一半代码失踪并导致前端语法高亮引擎死循环）
            // 改为仅过滤纯注释行
            if trimmed.starts_with("//") 
                || trimmed.starts_with("/*") 
                || trimmed.starts_with("*") 
                || trimmed.starts_with("--") 
                || trimmed.starts_with("<!--") 
                || (ext == "py" && trimmed.starts_with("#")) {
                continue;
            }
        }

        if config.remove_imports && re_import.is_match(trimmed) {
            continue;
        }

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

#[tauri::command]
pub async fn execute_extraction(files: Vec<String>, config: ExtractionConfig) -> Result<ExtractedCode, String> {
    let mut total_content = String::new();
    let mut total_lines = 0;

    for file_path in files {
        let header = format!("\n\n// --- File: {} ---\n\n", Path::new(&file_path).file_name().unwrap_or_default().to_string_lossy());
        total_content.push_str(&header);
        total_lines += 3;

        match extract_and_clean(&file_path, &config) {
            Ok(ext_res) => {
                total_content.push_str(&ext_res.content);
                total_lines += ext_res.line_count;
            },
            Err(e) => {
                let err_msg = format!("// Error extracting {}: {}\n", file_path, e);
                total_content.push_str(&err_msg);
                total_lines += 1;
            }
        }
    }

    Ok(ExtractedCode {
        content: total_content,
        line_count: total_lines,
    })
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportConfig {
    pub software_name: String,
    pub software_version: String,
    pub save_path: String,
}

#[tauri::command]
pub async fn export_to_docx(content: String, config: ExportConfig) -> Result<String, String> {
    let path = Path::new(&config.save_path);
    let file = File::create(path).map_err(|e| format!("无法创建 Word 文件: {}", e))?;

    let header_text = if config.software_name.trim().is_empty() {
        String::from("源代码提取报告")
    } else {
        format!("{}_{}", config.software_name, config.software_version)
    };

    let header = Header::new().add_paragraph(
        Paragraph::new()
            .add_run(Run::new().add_text(header_text).size(20))
            .align(AlignmentType::Center),
    );

    let footer = Footer::new().add_paragraph(
        Paragraph::new()
            .add_page_num(PageNum::new())
            .align(AlignmentType::Center),
    );

    let mut doc = Docx::new()
        .header(header)
        .footer(footer);

    let mut current_para = Paragraph::new();

    // 抛弃所有人为斩断、分页的魔改操作，回归自然！
    let mut current_para = Paragraph::new();
    
    // 我们为了避免几万行的内容挤在一个 XML Paragraph 节点中导致 Word 崩溃，
    // 依然维持每隔 50 行新建一个普通 Paragraph 层级（但绝不加入强分页符）
    for (i, line) in content.lines().enumerate() {
        if i > 0 && i % 50 == 0 {
            doc = doc.add_paragraph(current_para);
            current_para = Paragraph::new();
        }

        current_para = current_para
            .add_run(
                Run::new()
                    .add_text(line.to_string())
                    .fonts(RunFonts::new().ascii("Consolas").east_asia("微软雅黑"))
                    .size(18) // 9 磅 / 小五
            )
            .add_run(Run::new().add_break(BreakType::TextWrapping));
    }
    
    // 把余下的末端段落塞进去
    doc = doc.add_paragraph(current_para);

    doc.build().pack(file).map_err(|e| format!("封装 Docx 失败: {}", e))?;

    Ok(format!("导出成功，文件已保存至: {}", config.save_path))
}
