use std::fs;

use miniserde::json;

use crate::{models::Asset, paths::versions_dir, util::{download::download_file, extract::extract_file}};

enum OS {
  Windows,
  Linux,
  Macos,
}

fn get_os() -> OS {
  if cfg!(windows) {
    OS::Windows
  } else if cfg!(target_os = "macos") {
    OS::Macos
  } else {
    OS::Linux
  }
}

pub fn install_version(version: &str) {
  let dir = versions_dir().join(version);

  if dir.exists() {
    eprintln!("The version {} is already installed", version);
    return;
  }

  let Some(assets) = fetch_version_assets(version) else {
    eprintln!("Failed to fetch version data");
    return;
  };

  if let Err(e) = fs::create_dir_all(&dir) {
    eprintln!("Failed to create version directory: {}", e);
    return;
  }

  let pkg = &assets[0].binaries[0].package;
  println!("Downloading from: {}", pkg.link);

  let output_path = dir.join(&pkg.name);

  if let Err(e) = download_file(&pkg.link, &output_path) {
    eprintln!("Failed to download: {}", e);
    return;
  }

  println!("Downloaded JDK {} to {}", version, output_path.display());

  if let Err(e) = extract_file(&output_path, &dir) {
    eprintln!("Failed to extract file: {}", e);
    return;
  }

  if let Err(e) = fs::remove_file(&output_path) {
    eprintln!("Failed to delete compressed file: {}", e);
  }
}

fn fetch_version_assets(version: &str) -> Option<Vec<Asset>> {
  let os = match get_os() {
    OS::Windows => "windows",
    OS::Macos => "mac",
    OS::Linux => "linux",
  };

  let url = format!("https://api.adoptium.net/v3/assets/feature_releases/{}/ga?architecture=x64&image_type=jdk&jvm_impl=hotspot&os={}&project=jdk", version, os);

  println!("Fetching JDK {}", version);

  let output = std::process::Command::new("curl")
    .arg("-L")
    .arg("-s")
    .arg(&url)
    .output()
    .expect("Failed to run curl");

  if !output.status.success() {
    println!("API request failed");
    return None;
  }

  let json_str = String::from_utf8_lossy(&output.stdout);

  let assets: Vec<Asset> = json::from_str(&json_str)
    .expect("Failed to parse JSON response");

  if assets.is_empty() {
    println!("No releases found for version {}", version);
    return None;
  }
  Some(assets)
}
