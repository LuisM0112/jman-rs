use std::path::PathBuf;

pub fn home_dir() -> PathBuf {
  dirs_next::home_dir().unwrap_or_else(|| {
    eprintln!("Home dir not found");
    std::process::exit(1)
  })
}

pub fn base_dir() -> PathBuf {
  home_dir().join(".jman")
}

pub fn versions_dir() -> PathBuf {
  base_dir().join("versions")
}

pub fn current_symlink() -> PathBuf {
  base_dir().join("current")
}
