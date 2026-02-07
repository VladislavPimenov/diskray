pub mod main_panel;
pub mod tree_panel;
pub mod chart_panel;
pub mod details_panel;
pub mod disks_panel;  // Новый модуль

// Re-export
pub use main_panel::MainPanel;
pub use tree_panel::TreePanel;
pub use chart_panel::ChartPanel;
pub use details_panel::DetailsPanel;
pub use disks_panel::DisksPanel;  // Новый экспорт