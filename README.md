# uninstall-package

** CREDITS TO https://github.com/buyukakyuz/install-nothing FOR IDEA**

A Rust terminal application that identifies and uninstalls Linux packages that no longer exist in your system's package repositories.

## Features

- 🔍 Automatically detects your Linux distribution's package manager
- 📦 Scans all installed packages
- 🔎 Checks if each package exists in available repositories
- 🗑️ Safely uninstalls non-existent packages
- ✓ Supports multiple package managers:
  - APT (Debian/Ubuntu)
  - DNF (Fedora)
  - YUM (CentOS/RHEL)
  - Pacman (Arch Linux)
  - Zypper (openSUSE)

## Installation

### Prerequisites

- Rust 1.70 or later
- Linux system with one of the supported package managers
- `sudo` privileges (required for uninstalling packages)

### Build from source

```bash
git clone <repository-url>
cd package-uninstaller
cargo build --release
```

The compiled binary will be at `target/release/package-uninstaller`

## Usage

```bash
# Run the application
cargo run --release

# Or run the compiled binary directly
./target/release/package-uninstaller
```

## How it works

1. **Package Manager Detection**: Automatically detects which package manager is available on your system
2. **Package Enumeration**: Lists all installed packages using the system's package manager
3. **Repository Verification**: Checks each package against available repositories
4. **Safe Uninstallation**: Removes only packages that no longer exist in repositories
5. **User Feedback**: Provides clear output showing which packages were removed

## Supported Distributions

- Ubuntu/Debian
- Fedora
- CentOS/RHEL
- Arch Linux
- openSUSE

## Security Notes

- This application requires `sudo` privileges to uninstall packages
- Always review the list of packages before proceeding
- Consider backing up your system before running this tool
- The application confirms which packages will be uninstalled before removal

## Example Output

```
🗑️  Package Uninstaller - Remove Non-Existent Packages
======================================================

Detected package manager: apt

Found 450 installed packages

Found 3 non-existent packages:
  - old-package-1
  - obsolete-lib
  - deprecated-tool

Attempting to uninstall non-existent packages...

✓ Uninstalled: old-package-1
✓ Uninstalled: obsolete-lib
✓ Uninstalled: deprecated-tool

✓ Operation completed!
```

## Development

### Building

```bash
cargo build
```

### Running tests

```bash
cargo test
```

### Building release version

```bash
cargo build --release
```

## Troubleshooting

### Permission Denied

The application requires `sudo` privileges.  Make sure you have the necessary permissions:

```bash
sudo cargo run --release
```

### Package Manager Not Detected

If your package manager isn't detected, the application defaults to APT. You may need to modify the detection logic. 

### Uninstallation Fails

Some packages may be protected or have dependencies.  The application will report failures without forcing removal.

## License

Misty Foundation License v8.3

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
```

## Quick Start

1. **Clone or copy the files** to a directory
2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run the application**:
   ```bash
   sudo cargo run --release
   ```

## What This App Does

✨ **Features:**
- Detects your Linux distribution's package manager (apt, dnf, yum, pacman, zypper)
- Lists all installed packages
- Checks each package against available repositories
- Safely uninstalls packages that don't exist in repositories
- Shows clear progress and results

## Important Notes

⚠️ **Security & Safety:**
- Requires `sudo` privileges for uninstallation
- Always review the list of packages before proceeding
- Works with all major Linux distributions
- The app will inform you of any uninstallation failures

This is a fully functional Rust application that you can compile and run on your Linux system! 
