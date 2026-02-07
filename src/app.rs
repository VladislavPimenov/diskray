use eframe::egui;
use crate::scanner::{FileSystemScanner, ScanResult, ScanProgress};
use crate::analyzer::{DiskAnalyzer, AnalysisFilters};
use crate::ui::{MainPanel, TreePanel, ChartPanel, DetailsPanel, DisksPanel};
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::RwLock;

/// Main application state
pub struct DiskRayApp {
    pub scanner: Option<FileSystemScanner>,
    pub scan_result: Arc<RwLock<Option<ScanResult>>>,
    pub scan_progress: Arc<RwLock<ScanProgress>>,
    pub analyzer: DiskAnalyzer,
    pub filters: AnalysisFilters,
    pub main_panel: MainPanel,
    pub tree_panel: TreePanel,
    pub chart_panel: ChartPanel,
    pub details_panel: DetailsPanel,
    pub disks_panel: DisksPanel,
    pub current_path: PathBuf,
    pub is_scanning: bool,
    pub selected_path: Option<PathBuf>,
    pub view_mode: ViewMode,
    pub sort_by: SortColumn,
    pub sort_descending: bool,
}

impl DiskRayApp {
    pub fn new() -> Self {
        Self {
            scanner: None,
            scan_result: Arc::new(RwLock::new(None)),
            scan_progress: Arc::new(RwLock::new(ScanProgress::default())),
            analyzer: DiskAnalyzer::new(),
            filters: AnalysisFilters::default(),
            main_panel: MainPanel::new(),
            tree_panel: TreePanel::new(),
            chart_panel: ChartPanel::new(),
            details_panel: DetailsPanel::new(),
            disks_panel: DisksPanel::new(),
            current_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            is_scanning: false,
            selected_path: None,
            view_mode: ViewMode::Tree,
            sort_by: SortColumn::Size,
            sort_descending: true,
        }
    }
}

impl eframe::App for DiskRayApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.update_scanning();
        self.render_ui(ctx);
    }
}

impl DiskRayApp {
    fn update_scanning(&mut self) {
        if self.is_scanning {
            if let Some(scanner) = &mut self.scanner {
                if scanner.is_finished() {
                    self.is_scanning = false;
                    if let Some(result) = scanner.take_result() {
                        *self.scan_result.write() = Some(result.clone());
                        self.analyzer.analyze(&result);
                    }
                }
            }
        }
    }
    
    fn render_ui(&mut self, ctx: &egui::Context) {
        // Main menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            let view_mode = &mut self.view_mode;
            let is_scanning = &mut self.is_scanning;
            let scanner = &mut self.scanner;
            let selected_path = &mut self.selected_path;
            let current_path = &mut self.current_path;
            let scan_result = self.scan_result.clone();
            
            self.main_panel.render_menu(ui, view_mode, is_scanning, scanner, selected_path, current_path, scan_result);
        });
        
        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.view_mode {
                ViewMode::Tree => {
                    let selected_path = &mut self.selected_path;
                    let scan_result = self.scan_result.clone();
                    let scanner = &mut self.scanner;
                    let is_scanning = &mut self.is_scanning;
                    let current_path = &mut self.current_path;
                    
                    self.tree_panel.render(ui, selected_path, scan_result, scanner, is_scanning, current_path);
                }
                ViewMode::Chart => {
                    // Просто рисуем панель без данных
                    self.chart_panel.render(ui);
                }
                ViewMode::Details => {
                    // Просто рисуем панель без данных
                    self.details_panel.render(ui);
                }
                ViewMode::Disks => {
                    self.disks_panel.render(ui);
                }
            }
        });
        
        // Status bar at the bottom
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            let current_path = &self.current_path;
            let scan_result = self.scan_result.clone();
            
            self.main_panel.render_status(ui, current_path, scan_result);
        });
    }
}

/// View modes for the application
#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode {
    Tree,
    Chart,
    Details,
    Disks,
}

/// Columns for sorting
#[derive(Clone, Copy, PartialEq)]
pub enum SortColumn {
    Name,
    Size,
    Modified,
    Type,
    Count,
}