use crate::util::ext::CODE_EXTENSIONS;
use std::path::Path;
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};

pub fn collect_code_chunks(path: &Path) -> Vec<String> {
    let mut chunks = Vec::new();

    let total_files = WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .count();

    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-")
    );
    pb.set_message("🔍 Scanning codebase...");

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        pb.inc(1);
        pb.set_message(format!("📄 Processing: {}", entry.file_name().to_string_lossy()));

        let ext = entry.path().extension().and_then(|s| s.to_str());
        if let Some(extension) = ext {
            if CODE_EXTENSIONS.contains(&extension) {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    chunks.push(format!("{}:\n{}", entry.path().display(), content));
                }
            }
        }
    }

    pb.finish_with_message(format!("✅ Processed {} files, found {} code files", total_files, chunks.len()));
    chunks
}
