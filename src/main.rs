use clap::{Arg, Command};
use std::fs;
use std::error::Error;
use std::path::PathBuf;
use miniserde::{json, Deserialize};
#[cfg(unix)]
use std::os::unix::fs as unix_fs;
#[cfg(windows)]
use std::os::windows::fs as windows_fs;

#[derive(Debug, Deserialize)]
struct AvailableReleases {
  available_lts_releases: Vec<u16>,
  available_releases: Vec<u16>,
}

#[derive(Debug, Deserialize)]
struct PackageInfo {
  link: String,
  name: String,
}

#[derive(Debug, Deserialize)]
struct Binary {
  package: PackageInfo,
}

#[derive(Debug, Deserialize)]
struct Asset {
  binaries: Vec<Binary>,
}

fn home_dir() -> PathBuf {
  dirs::home_dir().unwrap_or_else(|| {
    eprintln!("Home dir not found");
    std::process::exit(1)
  })
}

fn base_dir() -> PathBuf {
  home_dir().join(".jman")
}

fn versions_dir() -> PathBuf {
  base_dir().join("versions")
}

fn current_symlink() -> PathBuf {
  base_dir().join("current")
}

fn main() {
  let matches = Command::new("jman")
    .version("0.1.0")
    .about("OpenJDK version manager")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
      Command::new("list")
        .about("List installed versions")
    )
    .subcommand(
      Command::new("list-remote")
        .about("List remote versions")
    )
    .subcommand(
      Command::new("use")
        .about("Select a version to use")
        .arg(
          Arg::new("version")
            .help("Version name")
            .required(true)
            .index(1),
        ),
    )
    .subcommand(
      Command::new("install")
        .about("Select a version to install")
        .arg(
          Arg::new("version")
            .help("Version name")
            .required(true)
            .index(1),
        ),
    )
    .subcommand(
      Command::new("remove")
        .about("Select a version to remove")
        .arg(
          Arg::new("version")
            .help("Version name")
            .required(true)
            .index(1),
        ),
    )
    .get_matches();

  match matches.subcommand() {
    Some(("list", _)) => list_versions(),
    Some(("list-remote", _)) => list_versions_remote(),
    Some(("use", arg)) => {
      let version = arg.get_one::<String>("version").unwrap();
      use_version(version);
      set_env();
    }
    Some(("install", arg)) => {
      let version = arg.get_one::<String>("version").unwrap();
      install_version(version);
    }
    Some(("remove", arg)) => {
      let version = arg.get_one::<String>("version").unwrap();
      remove_version(version);
    }
    _ => {},
  }
}

fn list_versions() {
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

fn use_version(version: &str) {
  let target = versions_dir().join(version);
  let target = match find_bin_path(&target) {
    Some(path) => path,
    None => {
      eprintln!("The version {} does not exist in {:?}", version, target);
      return;
    },
  };

  let current = current_symlink();
  if current.exists() {
    fs::remove_file(&current).unwrap();
  }

  #[cfg(unix)]
  {
    unix_fs::symlink(&target, &current).unwrap();
  }

  #[cfg(windows)]
  {
    if target.is_dir() {
      windows_fs::symlink_dir(&target, &current).unwrap();
    } else {
      windows_fs::symlink_file(&target, &current).unwrap();
    }
  }

  println!("Now using JDK {}", version);
  println!("JAVA_HOME set at {}", current.display());
}

fn list_versions_remote() {

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

fn install_version(version: &str) {
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

fn remove_version(version: &str) {
  let version_dir = versions_dir().join(version);

  if !version_dir.exists() {
    eprintln!("Version {} is not installed.", version);
    return;
  }

  let current = current_symlink();
  let is_current_symlink = is_active(&version_dir, &current);

  if let Err(e) = fs::remove_dir_all(&version_dir) {
    eprintln!("Failed to remove version {}: {}", version, e);
    return;
  }

  println!("Version {} removed.", version);

  if !is_current_symlink {
    return;
  }

  match fs::remove_file(&current) {
    Ok(_) => println!("Active version was removed. Symlink 'current' deleted."),
    Err(e) => eprintln!("Warning: Failed to remove current symlink: {}", e),
  }
}

fn is_active(version_dir: &PathBuf, current: &PathBuf) -> bool {
  if !current.exists() {
    return false;
  }
  
  match fs::read_link(current) {
    Ok(target) => target.starts_with(version_dir),
    Err(_) => false,
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

fn set_env() {
  // #[cfg(unix)]
  {
    let env_file = base_dir().join("env.sh");

    let content = format!(
      "export JAVA_HOME=\"{}/current\"\nexport PATH=\"$JAVA_HOME/bin:$PATH\"\n",
      base_dir().display()
    );
  
    if let Err(e) = fs::write(&env_file, content) {
      eprintln!("Failed to write env file: {}", e);
      return;
    }

    let bashrc = home_dir().join(".bashrc");
    let line = "source \"$HOME/.jman/env.sh\"";
    let bashrc_content = fs::read_to_string(&bashrc).unwrap_or_default();

    if bashrc_content.contains(line) {
      return;
    }

    let updated = format!("{}\n{}", bashrc_content, line);

    if let Err(e) = fs::write(&bashrc, updated) {
      eprintln!("Failed to write in bashrc: {}", e);
      return;
    }

    println!("Environment variables loaded into ~/.bashrc");
    println!("To use this java version on this session run: . ~/.jman/env.sh")
  }
}

fn download_file(url: &str, output_path: &PathBuf) -> Result<(), Box<dyn Error>> {
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

fn extract_file(archive: &PathBuf, dest: &PathBuf) -> Result<(), Box<dyn Error>> {
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

fn find_bin_path(version_dir: &PathBuf) -> Option<PathBuf> {
  let entries = fs::read_dir(version_dir).ok()?;

  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() && path.join("bin").join("java").exists() {
      return Some(path);
    }
  }

  None
}

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
