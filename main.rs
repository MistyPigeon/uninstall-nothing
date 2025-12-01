use std::process::{Command, Stdio};
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    println!("🗑️  Package Uninstaller - Remove Non-Existent Packages");
    println!("======================================================\n");

    // Detect package manager
    let package_manager = detect_package_manager();
    println!("Detected package manager: {}\n", package_manager);

    // Get all installed packages
    let installed_packages = match get_installed_packages(&package_manager) {
        Ok(pkgs) => pkgs,
        Err(e) => {
            eprintln!("Error getting installed packages: {}", e);
            return;
        }
    };

    println!("Found {} installed packages\n", installed_packages.len());

    // Find non-existent packages
    let non_existent = find_non_existent_packages(&installed_packages, &package_manager);

    if non_existent.is_empty() {
        println! ("✓ All installed packages exist in repositories!");
        return;
    }

    println! ("Found {} non-existent packages:", non_existent.len());
    for package in &non_existent {
        println!("  - {}", package);
    }

    println!("\nAttempting to uninstall non-existent packages...\n");

    // Uninstall non-existent packages
    for package in &non_existent {
        match uninstall_package(&package_manager, package) {
            Ok(_) => println!("✓ Uninstalled: {}", package),
            Err(e) => println!("✗ Failed to uninstall {}: {}", package, e),
        }
    }

    println!("\n✓ Operation completed!");
}

fn detect_package_manager() -> String {
    let managers = vec![
        ("apt", "apt-get --version"),
        ("dnf", "dnf --version"),
        ("yum", "yum --version"),
        ("pacman", "pacman --version"),
        ("zypper", "zypper --version"),
    ];

    for (name, check_cmd) in managers {
        if Command::new("sh")
            .arg("-c")
            .arg(check_cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
        {
            return name.to_string();
        }
    }

    "apt".to_string() // default fallback
}

fn get_installed_packages(package_manager: &str) -> io::Result<HashSet<String>> {
    let command = match package_manager {
        "apt" => "dpkg -l | awk '/^ii/ {print $2}'",
        "dnf" | "yum" => "rpm -qa --qf '%{NAME}\\n'",
        "pacman" => "pacman -Q | awk '{print $1}'",
        "zypper" => "rpm -qa --qf '%{NAME}\\n'",
        _ => "dpkg -l | awk '/^ii/ {print $2}'",
    };

    let output = Command::new("sh")
        .arg("-c")
        . arg(command)
        .output()?;

    let packages = String::from_utf8_lossy(&output.stdout);
    let package_set: HashSet<String> = packages
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| ! line.is_empty())
        .collect();

    Ok(package_set)
}

fn find_non_existent_packages(
    installed: &HashSet<String>,
    package_manager: &str,
) -> Vec<String> {
    let mut non_existent = Vec::new();

    for package in installed {
        if ! package_exists_in_repos(package, package_manager) {
            non_existent.push(package.clone());
        }
    }

    non_existent
}

fn package_exists_in_repos(package: &str, package_manager: &str) -> bool {
    let check_command = match package_manager {
        "apt" => format!("apt-cache search '^{}$' | grep -q .", package),
        "dnf" => format!("dnf search '{}' --cve 2>/dev/null | grep -q .", package),
        "yum" => format!("yum search '{}' 2>/dev/null | grep -q .", package),
        "pacman" => format!("pacman -Ss '^{}$' 2>/dev/null | grep -q .", package),
        "zypper" => format!("zypper search '{}' 2>/dev/null | grep -q .", package),
        _ => format!("apt-cache search '^{}$' | grep -q .", package),
    };

    Command::new("sh")
        . arg("-c")
        .arg(check_command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn uninstall_package(package_manager: &str, package: &str) -> io::Result<()> {
    let uninstall_command = match package_manager {
        "apt" => format!("sudo apt-get remove -y {}", package),
        "dnf" => format!("sudo dnf remove -y {}", package),
        "yum" => format!("sudo yum remove -y {}", package),
        "pacman" => format!("sudo pacman -R --noconfirm {}", package),
        "zypper" => format!("sudo zypper remove -y {}", package),
        _ => format!("sudo apt-get remove -y {}", package),
    };

    Command::new("sh")
        .arg("-c")
        .arg(uninstall_command)
        .output()?;

    Ok(())
      }
