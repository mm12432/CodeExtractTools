use ignore::WalkBuilder;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub const DEFAULT_EXTENSIONS: &[&str] = &[
    "c", "cpp", "cc", "h", "hpp", "cs", "java", "js", "ts", "py", "rs", "go", "php", "rb", "swift",
    "m", "mm", "kt", "scala", "sql", "sh", "bat", "ps1", "html", "css", "vue", "tsx", "jsx",
    "dart", "fs", "vb", "asm", "s", "less", "scss",
];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileInfo {
    pub id: usize,
    pub absolute_path: String,
    pub relative_path: String,
    pub lines: usize,
    pub extension: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScanResult {
    pub total_files: usize,
    pub total_original_lines: usize,
    pub language_counts: HashMap<String, usize>,
    pub files: Vec<FileInfo>,
}

#[tauri::command]
pub async fn scan_project(root: String, custom_excludes: Vec<String>, extensions: Vec<String>) -> Result<ScanResult, String> {
    let root_path = Path::new(&root);
    if !root_path.exists() {
        return Err("Root path does not exist".to_string());
    }

    // 预编译全部正则规则，凡是有语法错误的给出警告并跳过
    let compiled_regexes: Vec<Regex> = custom_excludes.into_iter().filter_map(|r| {
        Regex::new(&r).ok()
    }).collect();

    // 构建后缀白名单哈希集，如果前端没传则使用默认集
    let ext_whitelist: Vec<String> = if extensions.is_empty() {
        DEFAULT_EXTENSIONS.iter().map(|&s| s.to_string()).collect()
    } else {
        extensions.into_iter().map(|e| e.to_lowercase().replace(".", "")).collect()
    };

    let mut language_counts = HashMap::new();
    let mut files = Vec::new();
    let mut total_files = 0;
    let mut total_original_lines = 0;
    let mut next_id = 0;

    let mut walk_builder = WalkBuilder::new(root_path);
    walk_builder.standard_filters(true); // Respects .gitignore
    walk_builder.hidden(true); // Ignores hidden files

    let walker = walk_builder.build();

    for entry in walker.flatten() {
        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            let path = entry.path();

            // 1. Check extension whitelist
            let ext = match path.extension().and_then(|e| e.to_str()) {
                Some(e) => e.to_lowercase(),
                None => continue,
            };

            if !ext_whitelist.contains(&ext) {
                continue;
            }

            // 2. Custom exclude rules (True Regex matched against relative path)
            let relative_path = path
                .strip_prefix(root_path)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string()
                .replace("\\", "/");

            let mut is_excluded = false;
            for re in &compiled_regexes {
                // 如果正则匹配中了绝对路径或相对路径，则直接杀掉
                if re.is_match(&relative_path) || re.is_match(&path.to_string_lossy()) {
                    is_excluded = true;
                    break;
                }
            }
            if is_excluded {
                continue;
            }

            // 3. Original lines counting
            let mut lines = 0;
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                lines = reader.lines().count();
                total_original_lines += lines;
            }

            // 4. Record
            let count = language_counts.entry(ext.clone()).or_insert(0);
            *count += 1;

            files.push(FileInfo {
                id: next_id,
                absolute_path: path.to_string_lossy().to_string(),
                relative_path,
                lines,
                extension: ext,
            });
            
            next_id += 1;
            total_files += 1;
        }
    }

    Ok(ScanResult {
        total_files,
        total_original_lines,
        language_counts,
        files,
    })
}
