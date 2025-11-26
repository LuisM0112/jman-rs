#[cfg(unix)]
use std::fs;

#[cfg(windows)]
use winreg::enums::*;
#[cfg(windows)]
use winreg::RegKey;

use crate::paths;

#[cfg(unix)]
pub fn set_env() {

  let env_file = paths::base_dir().join("env.sh");

  let content = format!(
    "export JAVA_HOME=\"{}/current\"\nexport PATH=\"$JAVA_HOME/bin:$PATH\"\n",
    paths::base_dir().display()
  );

  if let Err(e) = fs::write(&env_file, content) {
    eprintln!("Failed to write env file: {}", e);
    return;
  }

  let bashrc = paths::home_dir().join(".bashrc");
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

#[cfg(windows)]
pub fn set_env() {

  let java_home = format!("{}\\current", paths::base_dir().display());
  let bin_path = format!("{}\\bin", java_home);

  let hk_current_user = RegKey::predef(HKEY_CURRENT_USER);
  let env = match hk_current_user
    .open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE) {
      Ok(env) => env,
      Err(e) => {
        eprintln!("Unable to open registry key: {}", e);
        return;
      },
    };

  let current_java_home: String = env.get_value("JAVA_HOME").unwrap_or_default();
  if current_java_home != java_home {
    match env.set_value("JAVA_HOME", &java_home) {
      Ok(_) => println!("JAVA_HOME updated to {}", java_home),
      Err(e) => {
        eprintln!("Unable to write JAVA_HOME: {}", e);
        return;
      }
    }
  }

  let path_value: String = env.get_value("PATH").unwrap_or_default();

  if !path_value.to_lowercase().contains(&bin_path.to_lowercase()) {
    let new_path = format!("{};{}", bin_path, path_value);

    match env.set_value("PATH", &new_path) {
      Ok(_) => println!("PATH updated with {}", bin_path),
      Err(e) => {
        eprintln!("Unable to update PATH: {}", e);
        return;
      }
    }
    println!("Restart your terminal to apply changes.");
    println!("Environment variables updated in Windows Registry.");
  }
}
