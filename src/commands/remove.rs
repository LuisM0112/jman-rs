use std::{fs, path::PathBuf};

use crate::paths::{current_symlink, versions_dir};


pub fn remove_version(version: &str) {
  let version_dir = versions_dir().join(version);

  if !version_dir.exists() {
    eprintln!("Version {} is not installed.", version);
    return;
  }

  let current = current_symlink();
  let is_current_symlink = is_active(&version_dir, &current);

  if let Err(e) = fs::remove_dir_all(&version_dir) {
    eprintln!("Failed to remove version {}: {}", version, e);
    return;
  }

  println!("Version {} removed.", version);

  if !is_current_symlink {
    return;
  }

  match fs::remove_file(&current) {
    Ok(_) => println!("Active version was removed. Symlink 'current' deleted."),
    Err(e) => eprintln!("Warning: Failed to remove current symlink: {}", e),
  }
}

fn is_active(version_dir: &PathBuf, current: &PathBuf) -> bool {
  if !current.exists() {
    return false;
  }
  
  match fs::read_link(current) {
    Ok(target) => target.starts_with(version_dir),
    Err(_) => false,
  }
}
