use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use walkdir::WalkDir;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Represents a file or directory in the scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub is_directory: bool,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub modified: DateTime<Utc>,
    pub extension: Option<String>,
    pub parent: Option<PathBuf>,
    pub children: Vec<PathBuf>,
}

/// Result of a file system scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub root_path: PathBuf,
    pub total_size: u64,
    pub file_count: u64,
    pub dir_count: u64,
    pub entries: Vec<FileEntry>,
    pub scan_duration: std::time::Duration,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub scan_time: DateTime<Utc>,
}

/// Scan progress information
#[derive(Debug, Clone, Default)]
pub struct ScanProgress {
    pub current_path: PathBuf,
    pub files_scanned: u64,
    pub total_files: Option<u64>,
    pub bytes_scanned: u64,
    pub is_complete: bool,
    pub error_count: u64,
}

/// File system scanner with progress tracking
pub struct FileSystemScanner {
    root_path: PathBuf,
    should_stop: Arc<AtomicBool>,
    progress: Arc<parking_lot::Mutex<ScanProgress>>,
    result: Arc<parking_lot::Mutex<Option<ScanResult>>>,
}

impl FileSystemScanner {
    pub fn new(path: PathBuf) -> Self {
        Self {
            root_path: path,
            should_stop: Arc::new(AtomicBool::new(false)),
            progress: Arc::new(parking_lot::Mutex::new(ScanProgress::default())),
            result: Arc::new(parking_lot::Mutex::new(None)),
        }
    }
    
    /// Start scanning in a separate thread
    pub fn start(&mut self) {
        let root_path = self.root_path.clone();
        let should_stop = self.should_stop.clone();
        let progress = self.progress.clone();
        let result = self.result.clone();
        
        std::thread::spawn(move || {
            if let Ok(scan_result) = Self::scan_directory(&root_path, &should_stop, &progress) {
                *result.lock() = Some(scan_result);
                if let Some(mut prog) = progress.try_lock() {
                    prog.is_complete = true;
                }
            }
        });
    }
    
    /// Stop the scanning process
    pub fn stop(&self) {
        self.should_stop.store(true, Ordering::SeqCst);
    }
    
    /// Check if scanning is finished
    pub fn is_finished(&self) -> bool {
        self.progress.lock().is_complete
    }
    
    /// Get the scan result if available
    pub fn take_result(&mut self) -> Option<ScanResult> {
        self.result.lock().take()
    }
    
    /// Get current progress
    pub fn get_progress(&self) -> ScanProgress {
        self.progress.lock().clone()
    }
    
    /// Actual scanning implementation
    fn scan_directory(
        root: &Path,
        should_stop: &AtomicBool,
        progress: &parking_lot::Mutex<ScanProgress>,
    ) -> Result<ScanResult> {
        let start_time = std::time::Instant::now();
        let mut entries = Vec::new();
        let mut total_size = 0;
        let mut file_count = 0;
        let mut dir_count = 0;
        
        let walker = WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok());
        
        let total_entries = walker.count();
        
        {
            let mut prog = progress.lock();
            prog.total_files = Some(total_entries as u64);
        }
        
        let mut path_to_index: HashMap<PathBuf, usize> = HashMap::new();
        
        for (i, entry) in WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .enumerate()
        {
            if should_stop.load(Ordering::Relaxed) {
                break;
            }
            
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            
            let path = entry.path().to_path_buf();
            
            {
                let mut prog = progress.lock();
                prog.current_path = path.clone();
                prog.files_scanned = i as u64 + 1;
            }
            
            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            
            let is_dir = metadata.is_dir();
            let size = if is_dir { 0 } else { metadata.len() };
            
            let extension = path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|s| s.to_lowercase());
            
            let modified = match metadata.modified() {
                Ok(time) => DateTime::<Utc>::from(time),
                Err(_) => Utc::now(),
            };
            
            let file_entry = FileEntry {
                path: path.clone(),
                name: path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string(),
                size,
                is_directory: is_dir,
                modified,
                extension,
                parent: path.parent().map(|p| p.to_path_buf()),
                children: Vec::new(),
            };
            
            let idx = entries.len();
            path_to_index.insert(path.clone(), idx);
            
            if is_dir {
                dir_count += 1;
            } else {
                file_count += 1;
                total_size += size;
                
                if let Some(mut prog) = progress.try_lock() {
                    prog.bytes_scanned += size;
                }
            }
            
            entries.push(file_entry);
        }
        
        // Build parent-child relationships
        for i in 0..entries.len() {
            if let Some(parent) = &entries[i].parent {
                if let Some(&parent_idx) = path_to_index.get(parent) {
                    // Use cloned path to avoid double borrowing
                    let child_path = entries[i].path.clone();
                    entries[parent_idx].children.push(child_path);
                }
            }
        }
        
        // Calculate directory sizes
        Self::calculate_directory_sizes(&mut entries, &path_to_index);
        
        let scan_duration = start_time.elapsed();
        
        Ok(ScanResult {
            root_path: root.to_path_buf(),
            total_size,
            file_count,
            dir_count,
            entries,
            scan_duration,
            scan_time: Utc::now(),
        })
    }
    
    /// Calculate directory sizes by summing child sizes
    fn calculate_directory_sizes(
        entries: &mut [FileEntry],
        path_to_index: &HashMap<PathBuf, usize>,
    ) {
        let mut dir_indices: Vec<usize> = entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_directory)
            .map(|(idx, _)| idx)
            .collect();
        
        dir_indices.sort_by(|&a, &b| {
            let depth_a = entries[a].path.components().count();
            let depth_b = entries[b].path.components().count();
            depth_b.cmp(&depth_a)
        });
        
        for idx in dir_indices {
            let mut dir_size = 0;
            for child_path in &entries[idx].children {
                if let Some(&child_idx) = path_to_index.get(child_path) {
                    dir_size += entries[child_idx].size;
                }
            }
            entries[idx].size = dir_size;
        }
    }
}