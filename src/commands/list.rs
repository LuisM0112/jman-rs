use std::fs;
use crate::paths::versions_dir;

pub fn list_versions() {
  let dir = versions_dir();
  if !dir.exists() {
    println!("There are not versions installed yet.");
    return;
  }

  println!("Installed versions:");
  for entry in fs::read_dir(dir).unwrap() {
    let entry = entry.unwrap();
    if entry.path().is_dir() {
      println!("- {}", entry.file_name().to_string_lossy());
    }
  }
}
