use eframe::egui;
use sysinfo::Disks;
use humansize::{format_size, DECIMAL};

/// Disks information panel
pub struct DisksPanel {
    disks_info: Vec<DiskInfo>,
    last_update: std::time::Instant,
    update_interval: f32,
}

#[derive(Clone)]
struct DiskInfo {
    name: String,
    mount_point: String,
    total_space: u64,
    available_space: u64,
    used_space: u64,
    usage_percent: f32,
    disk_type: String,
    file_system: String,
    is_removable: bool,
}

impl DisksPanel {
    pub fn new() -> Self {
        let mut panel = Self {
            disks_info: Vec::new(),
            last_update: std::time::Instant::now(),
            update_interval: 2.0, // Update every 2 seconds
        };
        panel.update_disks_info();
        panel
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui) {
        // Update disks info periodically
        let now = std::time::Instant::now();
        if now.duration_since(self.last_update).as_secs_f32() >= self.update_interval {
            self.update_disks_info();
            self.last_update = now;
        }
        
        ui.heading("💽 Disks Information");
        ui.add_space(10.0);
        
        if self.disks_info.is_empty() {
            ui.vertical_centered(|ui| {
                ui.label("No disks found or accessible.");
                ui.label("Try running with administrator privileges.");
            });
            return;
        }
        
        // Total statistics
        let total_space: u64 = self.disks_info.iter().map(|d| d.total_space).sum();
        let total_used: u64 = self.disks_info.iter().map(|d| d.used_space).sum();
        let total_available: u64 = self.disks_info.iter().map(|d| d.available_space).sum();
        
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Total Disk Space:")
                .color(egui::Color32::from_gray(220)));
            ui.label(egui::RichText::new(format!("{}", format_size(total_space, DECIMAL)))
                .color(egui::Color32::WHITE));
            
            ui.separator();
            
            ui.label(egui::RichText::new("Used:")
                .color(egui::Color32::from_gray(220)));
            ui.label(egui::RichText::new(format!("{}", format_size(total_used, DECIMAL)))
                .color(egui::Color32::from_rgb(255, 100, 100)));
            
            ui.separator();
            
            ui.label(egui::RichText::new("Available:")
                .color(egui::Color32::from_gray(220)));
            ui.label(egui::RichText::new(format!("{}", format_size(total_available, DECIMAL)))
                .color(egui::Color32::from_rgb(100, 200, 100)));
        });
        
        ui.separator();
        ui.add_space(10.0);
        
        // Table of disks
        egui::ScrollArea::vertical()
            .max_height(ui.available_height() - 50.0)
            .show(ui, |ui| {
                egui::Grid::new("disks_grid")
                    .num_columns(7)
                    .striped(true)
                    .spacing([20.0, 5.0])
                    .show(ui, |ui| {
                        // Header
                        ui.label(egui::RichText::new("Drive").strong());
                        ui.label(egui::RichText::new("Mount Point").strong());
                        ui.label(egui::RichText::new("Type").strong());
                        ui.label(egui::RichText::new("File System").strong());
                        ui.label(egui::RichText::new("Total").strong());
                        ui.label(egui::RichText::new("Used").strong());
                        ui.label(egui::RichText::new("Usage").strong());
                        ui.end_row();
                        
                        // Disk rows
                        for disk in &self.disks_info {
                            // Drive letter/name
                            let drive_icon = if disk.is_removable {
                                "💾" // Removable
                            } else if disk.disk_type == "SSD" {
                                "⚡" // SSD
                            } else {
                                "💽" // HDD
                            };
                            
                            ui.horizontal(|ui| {
                                ui.label(drive_icon);
                                ui.label(&disk.name);
                            });
                            
                            // Mount point
                            ui.label(&disk.mount_point);
                            
                            // Type
                            let type_color = if disk.disk_type == "SSD" {
                                egui::Color32::from_rgb(100, 200, 255) // Blue for SSD
                            } else {
                                egui::Color32::from_gray(200) // Gray for HDD
                            };
                            ui.colored_label(type_color, &disk.disk_type);
                            
                            // File system
                            ui.label(&disk.file_system);
                            
                            // Total space
                            ui.label(format_size(disk.total_space, DECIMAL));
                            
                            // Used space
                            ui.label(format_size(disk.used_space, DECIMAL));
                            
                            // Usage percentage with color
                            let usage_color = if disk.usage_percent > 90.0 {
                                egui::Color32::RED
                            } else if disk.usage_percent > 75.0 {
                                egui::Color32::YELLOW
                            } else {
                                egui::Color32::GREEN
                            };
                            
                            ui.horizontal(|ui| {
                                // Progress bar
                                ui.add(
                                    egui::ProgressBar::new(disk.usage_percent / 100.0)
                                        .desired_width(100.0)
                                        .fill(usage_color)
                                );
                                
                                // Percentage text
                                ui.colored_label(usage_color, format!("{:.1}%", disk.usage_percent));
                            });
                            
                            ui.end_row();
                        }
                    });
            });
    }
    
    fn update_disks_info(&mut self) {
        self.disks_info.clear();
        let disks = Disks::new_with_refreshed_list();
        
        for disk in disks.list() {
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space.saturating_sub(available_space);
            let usage_percent = if total_space > 0 {
                (used_space as f64 / total_space as f64 * 100.0) as f32
            } else {
                0.0
            };
            
            let disk_type = match disk.kind() {
                sysinfo::DiskKind::SSD => "SSD".to_string(),
                sysinfo::DiskKind::HDD => "HDD".to_string(),
                _ => "Unknown".to_string(),
            };
            
            // Исправляем получение файловой системы
            let file_system = disk.file_system().to_string_lossy().to_string();
            let mount_point = disk.mount_point().to_string_lossy().to_string();
            let name = disk.name().to_string_lossy().to_string();
            
            self.disks_info.push(DiskInfo {
                name,
                mount_point,
                total_space,
                available_space,
                used_space,
                usage_percent,
                disk_type,
                file_system,
                is_removable: disk.is_removable(),
            });
        }
        
        // Sort by mount point (C:, D:, etc.)
        self.disks_info.sort_by(|a, b| a.mount_point.cmp(&b.mount_point));
    }
}