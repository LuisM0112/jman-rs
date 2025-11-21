use miniserde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AvailableReleases {
  pub available_lts_releases: Vec<u16>,
  pub available_releases: Vec<u16>,
}

#[derive(Debug, Deserialize)]
pub struct PackageInfo {
  pub link: String,
  pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Binary {
  pub package: PackageInfo,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
  pub binaries: Vec<Binary>,
}