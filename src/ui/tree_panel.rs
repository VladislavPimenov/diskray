use eframe::egui;
use crate::scanner::FileEntry;
use crate::scanner::FileSystemScanner;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use crate::scanner::ScanResult;

/// Tree view panel for browsing file system
#[derive(Default)]
pub struct TreePanel {
    expanded_dirs: HashMap<std::path::PathBuf, bool>,
}

impl TreePanel {
    pub fn new() -> Self {
        Self {
            expanded_dirs: HashMap::new(),
        }
    }
    
    pub fn render(
        &mut self, 
        ui: &mut egui::Ui, 
        selected_path: &mut Option<std::path::PathBuf>,
        scan_result: Arc<RwLock<Option<ScanResult>>>,
        scanner: &mut Option<FileSystemScanner>,
        is_scanning: &mut bool,
        current_path: &mut std::path::PathBuf,
    ) {
        egui::TopBottomPanel::top("tree_panel_header")
            .exact_height(40.0)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("🌳 File Tree");
                    ui.label(format!("Path: {}", current_path.display()));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(egui::TextEdit::singleline(&mut String::new())
                            .hint_text("Search...")
                            .desired_width(200.0));
                    });
                });
            });
        
        egui::CentralPanel::default()
            .show_inside(ui, |ui| {
                if let Some(scan_result) = &*scan_result.read() {
                    let entry_map: HashMap<_, _> = scan_result.entries
                        .iter()
                        .map(|e| (e.path.clone(), e))
                        .collect();
                    
                    let root_entries: Vec<&FileEntry> = scan_result.entries
                        .iter()
                        .filter(|e| {
                            e.parent.as_ref().map_or(true, |parent| {
                                parent == &scan_result.root_path
                            })
                        })
                        .collect();
                    
                    // Temporary copy of selected_path for use in closure
                    let mut local_selected_path = selected_path.clone();
                    
                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            for entry in root_entries {
                                self.render_tree_node(ui, entry, &entry_map, &mut local_selected_path);
                            }
                        });
                    
                    // Update selected_path back to app
                    *selected_path = local_selected_path;
                } else {
                    // No scan data yet
                    ui.vertical_centered(|ui| {
                        ui.add_space(100.0);
                        ui.heading("No Scan Data");
                        ui.label("Select a directory to start analyzing disk usage");
                        ui.add_space(20.0);
                        if ui.button("📁 Scan Current Directory").clicked() {
                            *scanner = Some(FileSystemScanner::new(current_path.clone()));
                            scanner.as_mut().unwrap().start();
                            *is_scanning = true;
                        }
                    });
                }
            });
    }
    
    fn render_tree_node(
        &mut self,
        ui: &mut egui::Ui,
        entry: &FileEntry,
        entry_map: &HashMap<std::path::PathBuf, &FileEntry>,
        selected_path: &mut Option<std::path::PathBuf>,
    ) {
        let is_expanded = self.expanded_dirs
            .get(&entry.path)
            .copied()
            .unwrap_or(false);
        
        let is_selected = Some(&entry.path) == selected_path.as_ref();
        
        let response = ui.selectable_label(is_selected, self.format_entry(entry));
        
        if response.clicked() {
            *selected_path = Some(entry.path.clone());
        }
        
        if response.double_clicked() && entry.is_directory {
            let new_state = !is_expanded;
            self.expanded_dirs.insert(entry.path.clone(), new_state);
        }
        
        if entry.is_directory && is_expanded {
            ui.indent(egui::Id::new(&entry.path), |ui| {
                let mut children: Vec<&FileEntry> = entry.children
                    .iter()
                    .filter_map(|path| entry_map.get(path))
                    .copied()
                    .collect();
                
                children.sort_by(|a, b| b.size.cmp(&a.size));
                
                for child in children {
                    self.render_tree_node(ui, child, entry_map, selected_path);
                }
            });
        }
    }
    
    fn format_entry(&self, entry: &FileEntry) -> String {
        let size_str = humansize::format_size(entry.size, humansize::DECIMAL);
        let icon = if entry.is_directory { "📁" } else { "📄" };
        
        if entry.is_directory {
            format!("{} {} ({})", icon, entry.name, size_str)
        } else {
            format!("{} {} - {}", icon, entry.name, size_str)
        }
    }
}