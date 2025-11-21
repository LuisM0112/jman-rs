use std::path::PathBuf;
use std::error::Error;

pub fn extract_file(archive: &PathBuf, dest: &PathBuf) -> Result<(), Box<dyn Error>> {
  let status = std::process::Command::new("tar")
    .arg("-xf")
    .arg(archive)
    .arg("-C")
    .arg(dest)
    .status()?;

  if !status.success() {
    return Err("Extraction failed")?;
  }

  Ok(())
}