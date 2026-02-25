#![windows_subsystem = "windows"]
use slint::{Model, SharedString, VecModel};
use std::rc::Rc;
use std::path::Path;
use std::thread;

mod scanner;
mod extractor;
mod doc_gen;

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    // 适配 DPI：Slint 自动处理，我们建议逻辑像素尺寸
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();

    // 1. 目录浏览
    ui.on_select_dir({
        let ui_weak = ui_weak.clone();
        move || {
            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_root_path(folder.to_string_lossy().to_string().into());
                    ui.set_status_msg("目录已选择，请点击扫描分析项目结构".into());
                }
            }
        }
    });

    // 2. 扫描项目结构 (仅快速统计)
    ui.on_start_scan({
        let ui_weak = ui_weak.clone();
        move || {
            let ui = ui_weak.upgrade().unwrap();
            let root_path_str = ui.get_root_path().to_string();
            let rules_model = ui.get_exclude_rules();
            let custom_excludes: Vec<String> = rules_model.iter().map(|s| s.to_string()).collect();

            if !Path::new(&root_path_str).exists() {
                ui.set_status_msg("错误：目录不存在".into());
                return;
            }

            ui.set_status_msg("正在快速扫描统计...".into());
            ui.set_progress(0.2);

            let ui_weak_2 = ui_weak.clone();
            thread::spawn(move || {
                let scan_result = scanner::scan_project(Path::new(&root_path_str), &custom_excludes);
                
                let mut lang_models: Vec<LanguageStats> = Vec::new();
                let mut sorted_langs: Vec<_> = scan_result.language_counts.iter().collect();
                sorted_langs.sort_by(|a, b| b.1.cmp(a.1));
                
                for (name, count) in sorted_langs {
                    lang_models.push(LanguageStats {
                        name: name.into(),
                        count: *count as i32,
                        enabled: true,
                    });
                }

                let total_lines = scan_result.total_original_lines;
                let total_files = scan_result.total_files;

                slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_weak_2.upgrade() {
                        ui.set_languages(Rc::new(VecModel::from(lang_models)).into());
                        ui.set_total_lines(total_lines as i32);
                        ui.set_total_files(total_files as i32);
                        ui.set_status_msg("扫描成功，勾选语言后即可运行分析".into());
                        ui.set_progress(1.0);
                    }
                }).unwrap();
            });
        }
    });

    // 3. 运行深度提取与清洗
    ui.on_start_extract({
        let ui_weak = ui_weak.clone();
        move || {
            let ui = ui_weak.upgrade().unwrap();
            let root_path_str = ui.get_root_path().to_string();
            let rules_model = ui.get_exclude_rules();
            let custom_excludes: Vec<String> = rules_model.iter().map(|s| s.to_string()).collect();
            
            let lang_model = ui.get_languages();
            let mut enabled_exts = Vec::new();
            for i in 0..lang_model.row_count() {
                if let Some(l) = lang_model.row_data(i) {
                    if l.enabled { enabled_exts.push(l.name.to_string()); }
                }
            }

            if enabled_exts.is_empty() {
                ui.set_status_msg("提醒：请至少勾选一种后缀".into());
                return;
            }

            ui.set_status_msg("正在进行清洗与分析...".into());
            ui.set_progress(0.0);

            let ui_weak_2 = ui_weak.clone();
            thread::spawn(move || {
                // 重新执行扫描以确保规则最新
                let scan_result = scanner::scan_project(Path::new(&root_path_str), &custom_excludes);
                let config = extractor::ExtractionConfig {
                    remove_comments: true,
                    remove_imports: true,
                    compact_lines: true,
                };

                // 过滤
                let files_to_process: Vec<_> = scan_result.file_paths.into_iter()
                    .filter(|p| {
                        let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                        enabled_exts.contains(&ext)
                    }).collect();

                let mut total_cleaned = 0;
                let mut preview_acc = String::new();
                let count = files_to_process.len();

                for (idx, p) in files_to_process.iter().enumerate() {
                    if let Ok(res) = extractor::extract_and_clean(p, &config) {
                        total_cleaned += res.line_count;
                        if preview_acc.len() < 2000 {
                            preview_acc.push_str(&format!("--- FILE: {} ---\n", p.file_name().unwrap_or_default().to_string_lossy()));
                            preview_acc.push_str(&res.content);
                            preview_acc.push('\n');
                        }
                    }

                    if idx % 10 == 0 || idx == count-1 {
                        let prog = (idx as f32 / count as f32).max(0.01);
                        let ui_ptr = ui_weak_2.clone();
                        slint::invoke_from_event_loop(move || {
                           if let Some(ui) = ui_ptr.upgrade() { ui.set_progress(prog); }
                        }).unwrap();
                    }
                }

                slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_weak_2.upgrade() {
                        ui.set_cleaned_lines(total_cleaned as i32);
                        ui.set_estimated_pages((total_cleaned as i32 / 50).max(1));
                        ui.set_preview_text(if preview_acc.len() > 2000 { format!("{}...", &preview_acc[..2000]).into() } else { preview_acc.into() });
                        ui.set_status_msg("分析任务已就绪".into());
                        ui.set_progress(1.0);
                    }
                }).unwrap();
            });
        }
    });

    // 4. 规则管理回调
    ui.on_add_exclude_rule({
        let ui_weak = ui_weak.clone();
        move |rule| {
            if let Some(ui) = ui_weak.upgrade() {
                let current = ui.get_exclude_rules();
                let mut vec: Vec<SharedString> = current.iter().collect();
                vec.push(rule);
                ui.set_exclude_rules(Rc::new(VecModel::from(vec)).into());
            }
        }
    });

    ui.on_remove_exclude_rule({
        let ui_weak = ui_weak.clone();
        move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                let current = ui.get_exclude_rules();
                let mut vec: Vec<SharedString> = current.iter().collect();
                if (idx as usize) < vec.len() {
                    vec.remove(idx as usize);
                    ui.set_exclude_rules(Rc::new(VecModel::from(vec)).into());
                }
            }
        }
    });

    // 5. 语言勾选状态同步
    ui.on_toggle_language({
        let ui_weak = ui_weak.clone();
        move |name, enabled| {
            if let Some(ui) = ui_weak.upgrade() {
                let model = ui.get_languages();
                for i in 0..model.row_count() {
                    if let Some(mut data) = model.row_data(i) {
                        if data.name == name {
                            data.enabled = enabled;
                            model.set_row_data(i, data);
                            break;
                        }
                    }
                }
            }
        }
    });

    // 6. 一键导出 Word
    ui.on_start_export({
        let ui_weak = ui_weak.clone();
        move || {
            let ui = ui_weak.upgrade().unwrap();
            let software_name = ui.get_software_name().to_string();
            let app_version = ui.get_app_version().to_string();
            let root_path_str = ui.get_root_path().to_string();
            let rules_model = ui.get_exclude_rules();
            let custom_excludes: Vec<String> = rules_model.iter().map(|s| s.to_string()).collect();

            if software_name.is_empty() {
                ui.set_status_msg("错误：请输入软件全称".into());
                return;
            }

            // 获取选中的语言
            let lang_model = ui.get_languages();
            let mut enabled_exts = Vec::new();
            for i in 0..lang_model.row_count() {
                if let Some(l) = lang_model.row_data(i) {
                    if l.enabled { enabled_exts.push(l.name.to_string()); }
                }
            }

            if let Some(save_path) = rfd::FileDialog::new()
                .add_filter("Word 文档", &["docx"])
                .set_file_name(format!("{}_SourceCode.docx", software_name))
                .save_file() 
            {
                ui.set_status_msg("正在导出 Word 文档...".into());
                let ui_weak_red = ui_weak.clone();
                thread::spawn(move || {
                    let scan_result = scanner::scan_project(Path::new(&root_path_str), &custom_excludes);
                    let config = extractor::ExtractionConfig {
                        remove_comments: true,
                        remove_imports: true,
                        compact_lines: true,
                    };

                    let mut all_content = String::new();
                    let files_to_process: Vec<_> = scan_result.file_paths.into_iter()
                        .filter(|p| {
                            let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                            enabled_exts.contains(&ext)
                        }).collect();

                    for path in files_to_process {
                        if let Ok(res) = extractor::extract_and_clean(&path, &config) {
                            all_content.push_str(&res.content);
                        }
                    }

                    let doc_config = doc_gen::DocConfig {
                        software_name,
                        version: app_version,
                        total_pages_limit: 60,
                        lines_per_page: 50,
                    };

                    match doc_gen::generate_docx(&save_path, &all_content, &doc_config) {
                        Ok(_) => {
                            slint::invoke_from_event_loop(move || {
                                if let Some(ui) = ui_weak_red.upgrade() {
                                    ui.set_status_msg(format!("导出成功！路径: {}", save_path.display()).into());
                                }
                            }).unwrap();
                        }
                        Err(e) => {
                            slint::invoke_from_event_loop(move || {
                                if let Some(ui) = ui_weak_red.upgrade() {
                                    ui.set_status_msg(format!("导出失败：{}", e).into());
                                }
                            }).unwrap();
                        }
                    }
                });
            }
        }
    });

    ui.run()?;
    Ok(())
}
