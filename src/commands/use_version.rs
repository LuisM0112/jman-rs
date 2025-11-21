use std::{fs, path::PathBuf};

#[cfg(unix)]
use std::os::unix::fs as unix_fs;
#[cfg(windows)]
use std::os::windows::fs as windows_fs;

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
    fs::remove_file(&current).unwrap();
  }

  #[cfg(unix)]
  {
    unix_fs::symlink(&target, &current).unwrap();
  }

  #[cfg(windows)]
  {
    if target.is_dir() {
      windows_fs::symlink_dir(&target, &current).unwrap();
    } else {
      windows_fs::symlink_file(&target, &current).unwrap();
    }
  }

  println!("Now using JDK {}", version);
  println!("JAVA_HOME set at {}", current.display());
}

fn find_bin_path(version_dir: &PathBuf) -> Option<PathBuf> {
  let entries = fs::read_dir(version_dir).ok()?;

  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() && path.join("bin").join("java").exists() {
      return Some(path);
    }
  }

  None
}
