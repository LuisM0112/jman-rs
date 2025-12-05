use std::error::Error;
use miniserde::json;
use crate::models::{Asset, AvailableReleases};

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

pub fn fetch_available_releases() -> Result<AvailableReleases, Box<dyn Error>> {
  let url = "https://api.adoptium.net/v3/info/available_releases";

  let output = std::process::Command::new("curl")
    .arg("-L")
    .arg("-s")
    .arg(&url)
    .output()?;

  let json_str = String::from_utf8_lossy(&output.stdout);

  let info: AvailableReleases = json::from_str(&json_str)?;

  Ok(info)
}

pub fn fetch_version_assets(version: &str) -> Result<Vec<Asset>, Box<dyn Error>> {
  let os = match get_os() {
    OS::Windows => "windows",
    OS::Macos => "mac",
    OS::Linux => "linux",
  };

  let url = format!("https://api.adoptium.net/v3/assets/feature_releases/{}/ga?architecture=x64&image_type=jdk&jvm_impl=hotspot&os={}&project=jdk", version, os);

  let output = match std::process::Command::new("curl")
    .arg("-L")
    .arg("-s")
    .arg(&url)
    .output() {
      Ok(output) => output,
      Err(_) => {
        return Err("Failed to run curl")?;
      }
  };

  if !output.status.success() {
    return Err(format!("API request failed {}", output.status).into());
  }

  let json_str = String::from_utf8_lossy(&output.stdout);

  let assets: Vec<Asset> = match json::from_str(&json_str) {
    Ok(assets) => assets,
    Err(e) => {
      return Err(format!("Failed to parse JSON response: {}", e).into());
    }
  };

  if assets.is_empty() {
    return Err(format!("No releases found for version {}", version).into());
  }
  Ok(assets)
}
