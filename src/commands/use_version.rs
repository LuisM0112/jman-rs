use std::{fs, path::PathBuf};

#[cfg(unix)]
use std::os::unix::fs as unix_fs;

use crate::paths::{current_symlink, versions_dir};

pub fn use_version(version: &str) {
  let target = versions_dir().join(version);
  let target = match find_bin_path(&target) {
    Some(path) => path,
    None => {
      eprintln!("The version {} does not exist in {:?}", version, target);
      return;
    },
  };

  let current = current_symlink();
  if current.exists() {
    #[cfg(unix)]
    if let Err(e) = fs::remove_file(&current) {
      eprintln!("Failed to remove existing symlink: {}", e);
      return;
    }

    #[cfg(windows)]
    if let Err(e) = fs::remove_dir(&current) {
      eprintln!("Failed to remove existing junction: {}", e);
      return;
    }
  }

  #[cfg(unix)]
  {
    if let Err(e) = unix_fs::symlink(&target, &current) {
      eprintln!("Failed to create symlink: {}", e);
      return;
    }
  }

  #[cfg(windows)]
  {
    if let Err(e) = junction::create(&target, &current) {
      eprintln!("Failed to create junction: {}", e);
      return;
    }
  }

  println!("Now using JDK {}", version);
  println!("JAVA_HOME set at {}", current.display());
}

fn find_bin_path(version_dir: &PathBuf) -> Option<PathBuf> {
  let entries = fs::read_dir(version_dir).ok()?;

  #[cfg(unix)]
  let java_bin = "java";

  #[cfg(windows)]
  let java_bin = "java.exe";

  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() && path.join("bin").join(java_bin).exists() {
      return Some(path);
    }
  }

  None
}
