use ignore::WalkBuilder;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufRead, BufReader};

// 预定义的代码后缀白名单
pub const CODE_EXTENSIONS: &[&str] = &[
    "c", "cpp", "cc", "h", "hpp", "cs", "java", "js", "ts", "py", "rs", "go",
    "php", "rb", "swift", "m", "mm", "kt", "scala", "sql", "sh", "bat", "ps1",
    "html", "css", "vue", "tsx", "jsx", "dart", "fs", "vb", "asm", "s", "less", "scss"
];

pub struct ScanResult {
    pub total_files: usize,
    pub total_original_lines: usize,
    pub language_counts: HashMap<String, usize>,
    pub file_paths: Vec<PathBuf>,
}

pub fn scan_project(root: &Path, custom_excludes: &[String]) -> ScanResult {
    let mut language_counts = HashMap::new();
    let mut file_paths = Vec::new();
    let mut total_files = 0;
    let mut total_original_lines = 0;

    let mut walk_builder = WalkBuilder::new(root);
    walk_builder.standard_filters(true); // .gitignore
    walk_builder.hidden(true);

    // TODO: 这里可以进一步集成 custom_excludes 到 WalkBuilder 的 override 规则中
    // 暂时在遍历时手动过滤简单匹配

    let walker = walk_builder.build();

    for entry in walker.flatten() {
        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            let path = entry.path();
            
            // 1. 检查扩展名是否在白名单中
            let ext = match path.extension().and_then(|e| e.to_str()) {
                Some(e) => e.to_lowercase(),
                None => continue,
            };

            if !CODE_EXTENSIONS.contains(&ext.as_str()) {
                continue;
            }

            // 2. 自定义排除规则 (简单前缀/后缀匹配)
            let path_str = path.to_string_lossy();
            let mut is_excluded = false;
            for rule in custom_excludes {
                if path_str.contains(rule) {
                    is_excluded = true;
                    break;
                }
            }
            if is_excluded { continue; }

            // 3. 统计原始行数
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                let lines = reader.lines().count();
                total_original_lines += lines;
            }

            // 4. 记录结果
            let count = language_counts.entry(ext).or_insert(0);
            *count += 1;
            file_paths.push(path.to_path_buf());
            total_files += 1;
        }
    }

    ScanResult {
        total_files,
        total_original_lines,
        language_counts,
        file_paths,
    }
}
