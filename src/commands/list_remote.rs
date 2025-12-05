use crate::{models::AvailableReleases, util::api::fetch_available_releases};

pub fn list_versions_remote() {
  let info: AvailableReleases = match fetch_available_releases() {
    Ok(available_releases) => available_releases,
    Err(e) => {
      eprintln!("Failed to fetch available releases: {}", e);
      return;
    }
  };

  println!("Available LTS releases: {:?}", info.available_lts_releases);
  println!("Available releases: {:?}", info.available_releases);
}
