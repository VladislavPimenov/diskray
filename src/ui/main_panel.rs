use eframe::egui;
use super::super::app::ViewMode;
use crate::scanner::FileSystemScanner;
use std::sync::Arc;
use parking_lot::RwLock;
use crate::scanner::ScanResult;

/// Main panel with menu and controls
#[derive(Default)]
pub struct MainPanel {
    pub show_settings: bool,
    pub show_about: bool,
    pub dark_mode: bool,
    scan_path_input: String,
}

impl MainPanel {
    pub fn new() -> Self {
        Self {
            show_settings: false,
            show_about: false,
            dark_mode: true,
            scan_path_input: std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("/"))
                .to_string_lossy()
                .to_string(),
        }
    }
    
    pub fn render_menu(
        &mut self, 
        ui: &mut egui::Ui, 
        view_mode: &mut ViewMode,
        is_scanning: &mut bool,
        scanner: &mut Option<FileSystemScanner>,
        _selected_path: &mut Option<std::path::PathBuf>,
        current_path: &mut std::path::PathBuf,
        _scan_result: Arc<RwLock<Option<ScanResult>>>, // Добавили подчеркивание
    ) {
        ui.horizontal(|ui| {
            // File menu
            ui.menu_button("File", |ui| {
                if ui.button("📁 Select Directory...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.scan_path_input = path.to_string_lossy().to_string();
                        *current_path = path;
                    }
                    ui.close();
                }
                
                if ui.button("📁 Scan Selected Directory").clicked() {
                    let path = std::path::PathBuf::from(&self.scan_path_input);
                    if path.exists() {
                        *current_path = path.clone();
                        *scanner = Some(FileSystemScanner::new(path));
                        scanner.as_mut().unwrap().start();
                        *is_scanning = true;
                    }
                    ui.close();
                }
                
                if ui.button("📊 Export Report...").clicked() {
                    self.export_report();
                    ui.close();
                }
                
                ui.separator();
                
                if ui.button("🚪 Exit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            
            // View menu
            ui.menu_button("View", |ui| {
                if ui.radio_value(view_mode, ViewMode::Tree, "🌳 Tree View").clicked() {
                    ui.close();
                }
                if ui.button("📊 Chart View").clicked() {
                    *view_mode = ViewMode::Chart;
                    ui.close();
                }
                if ui.button("📋 Details View").clicked() {
                    *view_mode = ViewMode::Details;
                    ui.close();
                }
                if ui.button("💽 Disks Info").clicked() {
                    *view_mode = ViewMode::Disks;
                    ui.close();
                }
                
                ui.separator();
                
                ui.checkbox(&mut self.dark_mode, "Dark Mode");
                if ui.button("Reset Layout").clicked() {
                    // Reset UI layout if needed
                }
            });
            
            // Tools menu
            ui.menu_button("Tools", |ui| {
                if ui.button("🔍 Find Large Files").clicked() {
                    ui.close();
                }
                
                if ui.button("🔄 Find Duplicates").clicked() {
                    ui.close();
                }
                
                if ui.button("🗑️ Cleanup Suggestions").clicked() {
                    ui.close();
                }
            });
            
            // Help menu
            ui.menu_button("Help", |ui| {
                if ui.button("📚 Documentation").clicked() {
                    ui.close();
                }
                
                if ui.button("ℹ️ About DiskRay").clicked() {
                    self.show_about = true;
                    ui.close();
                }
            });
            
            // Spacer and status
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if *is_scanning {
                    ui.spinner();
                    ui.label("Scanning...");
                }
                
                if ui.button("⚙️").clicked() {
                    self.show_settings = !self.show_settings;
                }
            });
        });
        
        // Path selection panel
        ui.horizontal(|ui| {
            ui.label("Scan path:");
            ui.add(egui::TextEdit::singleline(&mut self.scan_path_input)
                .desired_width(ui.available_width() * 0.7)
                .hint_text("Enter path or click Browse..."));
            
            if ui.button("📁 Browse...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.scan_path_input = path.to_string_lossy().to_string();
                    *current_path = path;
                }
            }
            
            if ui.button("▶️ Scan").clicked() && !self.scan_path_input.trim().is_empty() {
                let path = std::path::PathBuf::from(&self.scan_path_input);
                if path.exists() {
                    *current_path = path.clone();
                    *scanner = Some(FileSystemScanner::new(path));
                    scanner.as_mut().unwrap().start();
                    *is_scanning = true;
                }
            }
            
            if ui.button("⏹️ Stop").clicked() && *is_scanning {
                if let Some(scanner) = scanner {
                    scanner.stop();
                    *is_scanning = false;
                }
            }
        });
        
        // Quick disk selection buttons
        ui.horizontal(|ui| {
            ui.label("Quick scan:");
            
            // Common Windows drives
            let drives = ['C', 'D', 'E', 'F', 'G', 'H'];
            for &drive in &drives {
                let drive_path = format!("{}:\\", drive);
                let button_text = format!("{}:", drive);
                
                if ui.button(button_text).clicked() {
                    let path = std::path::PathBuf::from(&drive_path);
                    if path.exists() {
                        self.scan_path_input = drive_path.clone();
                        *current_path = path.clone();
                        *scanner = Some(FileSystemScanner::new(path));
                        scanner.as_mut().unwrap().start();
                        *is_scanning = true;
                    }
                }
            }
            
            // Home directory
            if let Some(home) = dirs::home_dir() {
                if ui.button("🏠 Home").clicked() {
                    self.scan_path_input = home.to_string_lossy().to_string();
                    *current_path = home.clone();
                    *scanner = Some(FileSystemScanner::new(home));
                    scanner.as_mut().unwrap().start();
                    *is_scanning = true;
                }
            }
            
            // Desktop
            if let Some(desktop) = dirs::desktop_dir() {
                if ui.button("🖥️ Desktop").clicked() {
                    self.scan_path_input = desktop.to_string_lossy().to_string();
                    *current_path = desktop.clone();
                    *scanner = Some(FileSystemScanner::new(desktop));
                    scanner.as_mut().unwrap().start();
                    *is_scanning = true;
                }
            }
        });
        
        // Dialogs
        if self.show_settings {
            self.render_settings(ui.ctx());
        }
        
        if self.show_about {
            self.render_about(ui.ctx());
        }
    }
    
    pub fn render_status(
        &self, 
        ui: &mut egui::Ui, 
        current_path: &std::path::PathBuf,
        scan_result: Arc<RwLock<Option<ScanResult>>>,
    ) {
        ui.horizontal(|ui| {
            ui.label(format!("📁 Current path: {}", current_path.display()));
            ui.separator();
            
            if let Some(scan_result) = &*scan_result.read() {
                ui.label(format!(
                    "📊 Files: {}, Size: {}",
                    scan_result.file_count,
                    humansize::format_size(scan_result.total_size, humansize::DECIMAL)
                ));
                ui.separator();
                ui.label(format!(
                    "⏱️ Scan time: {:.2}s",
                    scan_result.scan_duration.as_secs_f32()
                ));
            }
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("DiskRay v0.2.0");
            });
        });
    }
    
    fn export_report(&self) {
        println!("Export report");
    }
    
    fn render_settings(&mut self, ctx: &egui::Context) {
        let mut settings_open = self.show_settings;
        
        let response = egui::Window::new("Settings")
            .open(&mut settings_open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                egui::Grid::new("settings_grid")
                    .num_columns(2)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label("Theme:");
                        ui.horizontal(|ui| {
                            ui.radio_value(&mut self.dark_mode, true, "Dark");
                            ui.radio_value(&mut self.dark_mode, false, "Light");
                        });
                        ui.end_row();
                        
                        ui.label("Update UI theme:");
                        if ui.button("Apply Theme").clicked() {
                            if self.dark_mode {
                                ctx.set_visuals(egui::Visuals::dark());
                            } else {
                                ctx.set_visuals(egui::Visuals::light());
                            }
                        }
                        ui.end_row();
                    });
                
                ui.separator();
                
                let mut should_close = false;
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        should_close = true;
                    }
                    if ui.button("Cancel").clicked() {
                        should_close = true;
                    }
                });
                
                should_close
            });
        
        if response.is_some() {
            self.show_settings = settings_open;
            if let Some(response) = response {
                if let Some(should_close) = response.inner {
                    if should_close {
                        self.show_settings = false;
                    }
                }
            }
        }
    }
    
    fn render_about(&mut self, ctx: &egui::Context) {
        let mut about_open = self.show_about;
        
        let response = egui::Window::new("About DiskRay")
            .open(&mut about_open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("DiskRay");
                    ui.label("Advanced Disk Space Analyzer");
                    ui.add_space(10.0);
                    ui.label("Version 0.2.0");
                    ui.label("Built with Rust and egui");
                    ui.add_space(20.0);
                    ui.hyperlink("https://github.com/yourusername/diskray");
                    ui.add_space(20.0);
                    ui.label("© 2024 Your Name");
                });
                
                ui.separator();
                
                let mut should_close = false;
                ui.horizontal(|ui| {
                    if ui.button("Close").clicked() {
                        should_close = true;
                    }
                });
                
                should_close
            });
        
        if response.is_some() {
            self.show_about = about_open;
            if let Some(response) = response {
                if let Some(should_close) = response.inner {
                    if should_close {
                        self.show_about = false;
                    }
                }
            }
        }
    }
}