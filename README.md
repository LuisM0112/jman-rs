# `jman-rs`

## introduction

Lightweight OpenJDK version manager CLI for Linux, Windows, or macOS
Fast, minimal, and cross-platform Java version switching — written in Rust.

> `jman-rs` downloads OpenJDK binaries directly from the [Eclipse Adoptium (Temurin) API](https://api.adoptium.net/).

## Features

- Install multiple OpenJDK versions

- Switch between versions

- Uses symlink/junction for version switching

- List available remote versions

- Remove installed versions

## Installation

**You can review the installation scripts for [Linux](https://github.com/LuisM0112/jman-rs/blob/main/install.sh) or [Windows](https://github.com/LuisM0112/jman-rs/blob/main/install.ps1) before running it**

### Linux and macOS

```bash
curl -fsSL https://raw.githubusercontent.com/LuisM0112/jman-rs/main/install.sh | bash
```

### Windows

```powershell
iwr https://raw.githubusercontent.com/LuisM0112/jman-rs/main/install.ps1 -useb | iex
```

*After installation, restart your terminal or run `source ~/.bashrc`.*

## Directory Structure

jman-rs stores everything under:

```bash
~/.jman/
├─ bin/
│   └─ jman        # jman binary
├─ versions/       # installed JDKs
│   ├─ 17/
│   │   └─ jdk_17_0_10/
│   └─ 21/
│       └─ jdk_21_0_2/
├─ current → versions/<version>/... (symlink/junction)
└─ env             # Linux/macOS only
```

## How It Works

**Linux / macOS**

- `~/.jman/current` → symlink to the selected JDK

- `JAVA_HOME` and `PATH` are exported via `env.sh`

- Automatically sourced in `~/.bashrc`

**Windows**

- `current` is a junction, not a privileged symlink

- `JAVA_HOME` and `PATH` updated in
  `HKEY_CURRENT_USER\Environment`

- May require restarting terminal after switching versions

## Usage

**List installed versions**

```
jman list
```

**List remote available versions**

```
jman list-remote
```

**Install a version**

```
jman install 21
```

**Use a version**

```
jman use 21
```

**Remove a version**

```
jman remove 17
```

## Building From Source

```bash
git clone https://github.com/LuisM0112/jman-rs
cd jman-rs
cargo build --release
```

Binary will be located at:
`target/release/jman-rs`

## Contributing

Pull requests are welcome!
Feel free to open issues for suggestions or bug reports.

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
