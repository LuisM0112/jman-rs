use std::path::PathBuf;
use std::error::Error;

pub fn download_file(url: &str, output_path: &PathBuf) -> Result<(), Box<dyn Error>> {
  let status = std::process::Command::new("curl")
    .arg("-L")
    .arg("-o")
    .arg(output_path)
    .arg(url)
    .status()?;

  if !status.success() {
    return Err(format!("curl failed with status {}", status).into());
  }

  Ok(())
}
