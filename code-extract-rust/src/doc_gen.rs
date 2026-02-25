use docx_rs::*;
use std::fs::File;
use std::path::Path;

pub struct DocConfig {
    pub software_name: String,
    pub version: String,
    pub total_pages_limit: usize, // 默认 60 (前30+后30)
    pub lines_per_page: usize,    // 默认 50
}

pub fn generate_docx(
    output_path: &Path,
    content: &str, // 已清洗的完整代码
    config: &DocConfig,
) -> anyhow::Result<()> {
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();
    let max_lines = config.total_pages_limit * config.lines_per_page;

    let final_lines = if total_lines <= max_lines {
        lines
    } else {
        let mid = max_lines / 2;
        let mut combined = lines[0..mid].to_vec();
        combined.extend_from_slice(&lines[total_lines - mid..total_lines]);
        combined
    };

    let path = std::path::Path::new(output_path);
    let file = File::create(path)?;

    let mut doc = Docx::new();

    // 在文档开头加入标题行 (作为页眉的替代方案，确合规)
    doc = doc.add_paragraph(
        Paragraph::new()
            .add_run(Run::new().add_text(format!("软件名称: {}   版本: {}", config.software_name, config.version)))
            .align(AlignmentType::Center),
    );

    // 遍历行并分页
    let mut current_para = Paragraph::new();
    for (i, line) in final_lines.iter().enumerate() {
        if i > 0 && i % config.lines_per_page == 0 {
            doc = doc.add_paragraph(current_para);
            current_para = Paragraph::new().add_run(Run::new().add_break(BreakType::Page));
        }
        current_para = current_para.add_run(Run::new().add_text(line.to_string())).add_run(Run::new().add_break(BreakType::TextWrapping));
    }
    
    doc = doc.add_paragraph(current_para);

    doc.build().pack(file)?;

    Ok(())
}
