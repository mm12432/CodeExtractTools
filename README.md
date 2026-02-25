# 软著代码提取助手 Pro | Software Copyright Code Extractor Pro

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Slint](https://img.shields.io/badge/UI-Slint-blue.svg)](https://slint.dev/)
[![Version](https://img.shields.io/badge/version-2.0-green.svg)](https://github.com/)

[中文](#中文说明) | [English](#english-description)

---

## 中文说明

### 🌟 简介
**CodeExtractTools** 是一个专门用于为中国软件著作权申请提取和清洗源代码的工具集。本项目包含两个版本：
- **v1.0 (Legacy)**: 基于 C# .NET WinForm 构建的初始版本（位于 `CodeExtractTools/` 目录下）。
- **v2.0 (Pro/Active)**: 基于 Rust 和 Slint 构建的高性能原生版本（位于 `code-extract-rust/` 目录下）。

v2.0 提供了更强的性能、更低的资源占用以及丝滑的异步交互体验，是目前推荐使用的版本。

### 🚀 核心特性 (v2.0 Pro)
- **极速性能**：基于 Rust 核心，支持异步文件扫描，处理万级文件不卡死。
- **三栏宽屏 UI**：全新的 1200px 逻辑像素布局，完美适配 4K 及高 DPI 缩放。
- **智能过滤系统**：自动识别源码语言，剔除多媒体、二进制及无关配置文件。
- **深度清洗**：自动去除多种语言注释 (`//`, `#`, `--`, `'`)、导包语句并压缩空行。
- **合规导出**：自动截取前后各 30 页，按每页 50 行标准生成符合要求的 .docx 文档。
- **编码防乱码**：内置自动编码识别转换，完美支持 GBK 与 UTF-8。

### 📊 版本对比 (Comparison)
| 特性 | v1.0 (C# WinForm) | v2.0 (Rust Slint) |
| :--- | :--- | :--- |
| **内存占用** | ~200MB+ | **~20MB** |
| **UI 响应** | 扫描时易假死 | **全异步，始终流畅** |
| **DPI 适配** | 较差 (易模糊) | **原生高清适配** |
| **文件过滤** | 手动正则 | **智能白名单 + 自定义规则** |
| **跨平台** | Windows 仅限 | Windows / Linux / macOS |

---

## English Description

### 🌟 Introduction
**Software Copyright Code Extractor Pro (v2.0)** is a high-performance, native desktop utility built with Rust and Slint. It is specifically designed to extract and clean source code for Chinese Software Copyright applications. Migrating from v1.0 (C# .NET), this version offers superior performance, lower memory footprint, and a seamless asynchronous user experience.

### 🚀 Key Features (v2.0 Pro)
- **Rust-Powered Engine**: Blazing fast file traversal and cleaning without UI freezing.
- **Modern Wide Layout**: 1200px logical pixel design, optimized for 4K and High-DPI displays.
- **Smart Filtering**: Automatic language detection; shields out multimedia, binaries, and irrelevant configs.
- **Deep Cleaning**: Auto-removes comments (`//`, `#`, `--`, `'`), import statements, and compacts empty lines.
- **Compliance Export**: Auto-extracts the first and last 30 pages into standard 50 lines-per-page `.docx` files.
- **Encoding Friendly**: Built-in auto-detection for GBK/UTF-8 to prevent garbled text.

---

## 🛠️ 安装与运行 | Installation & Usage

### 运行程序 (Local Run)
```bash
# 需已安装 Rust 环境 (Rust environment required)
git clone <repo-url>
cd code-extract-rust
cargo run
```

### 编译发布 (Release Build)
```bash
cargo build --release
```

## 📝 待办事项 | Roadmap
- [ ] 可视化目录树显示 (Visual Directory Tree)
- [ ] 鼠标右键快捷排除功能 (Right-click Exclude)
- [ ] 敏感信息泄露检测 (Sensitive Info Detection)

## 📄 开源协议 | License
MIT License
