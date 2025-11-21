use miniserde::json;
use crate::models::AvailableReleases;

pub fn list_versions_remote() {

  let url = "https://api.adoptium.net/v3/info/available_releases";

  let output = std::process::Command::new("curl")
    .arg("-L")
    .arg("-s")
    .arg(&url)
    .output()
    .expect("Failed to run curl");

  let json_str = String::from_utf8_lossy(&output.stdout);

  let info: AvailableReleases = match json::from_str(&json_str) {
    Ok(info) => info,
    Err(e) => {
      eprintln!("Failed to parse JSON response: {}", e);
      return;
    }
  };

  println!("Available LTS releases: {:?}", info.available_lts_releases);
  println!("Available releases: {:?}", info.available_releases);
}
