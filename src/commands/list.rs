use std::fs::read_dir;
use crate::paths::versions_dir;

pub fn list_versions() {
  let dir = versions_dir();
  if !dir.exists() {
    println!("There are not versions installed yet.");
    return;
  }

  let entries = match read_dir(dir) {
    Ok(entries) => entries,
    Err(e) => {
      eprintln!("Failed to read versions directory: {}", e);
      return;
    }
  };

  let versions: Vec<_> = entries
    .flatten()
    .filter(|e| e.path().is_dir())
    .map(|e| e.file_name().to_string_lossy().to_string())
    .collect();

  if versions.is_empty() {
    println!("There are not versions installed yet.");
    return;
  }
  
  println!("Installed versions:");
  for version in versions {
    println!("- {}", version);
  }
}
