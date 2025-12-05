use std::fs;
use crate::util::api::fetch_available_releases;
use crate::util::{api::fetch_version_assets, download::download_file, extract::extract_file};
use crate::paths::versions_dir;

pub fn install_version(version: &str) {
  let dir = versions_dir().join(version);

  if dir.exists() {
    eprintln!("The version {} is already installed", version);
    return;
  }

  let version_num: u16 = match version.parse() {
    Ok(version_num) => version_num,
    Err(_) => {
      eprintln!("Version must be a number.");
      return;
    }
  };

  let version_info = match fetch_available_releases() {
    Ok(version_info) => version_info,
    Err(e) => {
      eprintln!("Failed to fetch available releases: {}", e);
      return;
    }
  };

  if !version_info.available_releases.contains(&version_num) {
    eprintln!("Version {} does not exist. Try `jman list-remote`.", version);
    return;
  }

  println!("Fetching JDK {}", version);

  let assets = match fetch_version_assets(version) {
    Ok(assets) => assets,
    Err(e) => {
      eprintln!("Failed to fetch version data: {}", e);
      return;
    }
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
