use crate::util::ext::CODE_EXTENSIONS;
use crate::util::filters::{should_skip_directory, should_skip_file, get_file_priority, IMPORTANT_FILES};
use std::path::Path;
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};

pub fn collect_code_chunks(path: &Path) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut file_data = Vec::new();

    // Create a spinner for the discovery phase
    let discovery_pb = ProgressBar::new_spinner();
    discovery_pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {elapsed_precise} {msg}")
            .unwrap()
    );
    discovery_pb.set_message("🔍 Discovering files...");
    discovery_pb.enable_steady_tick(std::time::Duration::from_millis(100));

    // First pass: collect files with filtering
    let mut discovered_count = 0;
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            // Skip directories that should be filtered out
            if e.file_type().is_dir() {
                let dir_name = e.file_name().to_string_lossy();
                !should_skip_directory(&dir_name)
            } else {
                true
            }
        })
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        discovered_count += 1;
        if discovered_count % 50 == 0 {
            discovery_pb.set_message(format!("🔍 Discovered {} files...", discovered_count));
        }

        let file_path = entry.path();
        
        // Skip files that should be filtered out
        if should_skip_file(file_path) {
            continue;
        }

        // Check if it's a code file or important file
        let ext = file_path.extension().and_then(|s| s.to_str());
        let file_name = file_path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        let is_code_file = ext.is_some_and(|extension| CODE_EXTENSIONS.contains(&extension));
        let is_important_file = IMPORTANT_FILES.iter().any(|&imp| file_name.eq_ignore_ascii_case(imp));

        if is_code_file || is_important_file {
            let priority = get_file_priority(file_path);
            file_data.push((file_path.to_path_buf(), priority));
        }
    }

    discovery_pb.finish_with_message(format!("✅ Found {} relevant files from {} total", file_data.len(), discovered_count));

    // Sort files by priority (high to low)
    file_data.sort_by(|a, b| b.1.cmp(&a.1));

    let total_files = file_data.len();
    
    if total_files == 0 {
        println!("⚠️  No code files found to process");
        return chunks;
    }

    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-")
    );
    pb.set_message("� Reading file contents...");

    // Second pass: read file contents
    for (file_path, _priority) in file_data {
        pb.inc(1);
        let file_name = file_path.file_name().unwrap_or_default().to_string_lossy();
        pb.set_message(format!("📄 {}", file_name));

        if let Ok(content) = std::fs::read_to_string(&file_path) {
            // Skip empty files or files with only whitespace
            if content.trim().is_empty() {
                continue;
            }
            
            chunks.push(format!("{}:\n{}", file_path.display(), content));
        }
    }

    pb.finish_with_message(format!("✅ Processed {} code files successfully", chunks.len()));
    chunks
}
