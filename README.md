# diskray
# DiskRay ⚡

> Blazing fast disk space analyzer built with Rust

**DiskRay** is a high-performance disk space analyzer that leverages Rust's safety and speed to provide lightning-fast scanning and comprehensive file analysis.

## 🚀 Why DiskRay?

- **⚡ Extreme Performance** - Multi-threaded scanning using Rayon
- **🔍 Complete Visibility** - Finds files that other tools miss
- **🎨 Modern Interface** - Clean GUI built with egui framework
- **📊 Smart Analysis** - Visual charts, file type breakdowns, trend tracking
- **🦀 Memory Safe** - Written in Rust, no crashes or memory leaks

## ✨ Key Features

### 🔥 Performance Optimized
- **Parallel file system scanning** - Utilizes all CPU cores
- **Zero-cost abstractions** - Rust's efficiency at work
- **Optimized I/O operations** - Minimal system impact
- **Real-time progress** - See results as they appear

### 📁 Comprehensive Analysis
- **Complete file discovery** - No hidden files overlooked
- **File type categorization** - Group by extension, size, date
- **Visual tree maps** - Instant visual understanding
- **Export capabilities** - CSV, JSON, HTML reports

### 🎯 User Experience
- **Dark/Light themes** - Comfortable viewing
- **Intuitive navigation** - Easy folder browsing
- **Quick actions** - Delete, open, explore with one click
- **Portable** - Single executable, no installation needed

## 📦 Installation

### Download Binary (Windows)
1. Visit [Releases](../../releases)
2. Download `diskray.exe`
3. Run directly - no installation required!

### Build from Source
```bash
git clone https://github.com/VladislavPimenov/diskray.git
cd diskray
cargo build --release
# Binary: target/release/diskray.exe
