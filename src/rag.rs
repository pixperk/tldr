use crate::util::ext::CODE_EXTENSIONS;
use std::path::Path;
use walkdir::WalkDir;

pub fn collect_code_chunks(path: &Path) -> Vec<String> {
    let mut chunks = Vec::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let ext = entry.path().extension().and_then(|s| s.to_str());
        if let Some(extension) = ext {
            if CODE_EXTENSIONS.contains(&extension) {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    chunks.push(format!("{}:\n{}", entry.path().display(), content));
                }
            }
        }
    }

    chunks
}
