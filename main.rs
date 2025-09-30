use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

fn main() -> io::Result<()> {
    println!("🔧 Setting up Powerline terminal customization...");

    // Step 1: Detect home directory
    let home_dir = dirs::home_dir().expect("Unable to find home directory");

    // Step 2: Detect shell
    let shell = env::var("SHELL").unwrap_or_else(|_| "bash".to_string());
    println!("Detected shell: {}", shell);

    let rc_file: PathBuf = if shell.contains("zsh") {
        home_dir.join(".zshrc")
    } else {
        home_dir.join(".bashrc")
    };

    // Step 3: Check if Powerline fonts are installed
    println!("Checking if Powerline fonts are installed...");
    let fc_list = Command::new("fc-list").output();

    match fc_list {
        Ok(output) => {
            let fonts = String::from_utf8_lossy(&output.stdout);
            if fonts.contains("Powerline") {
                println!("✅ Powerline fonts detected.");
            } else {
                println!("⚠️  Powerline fonts not detected.");
                println!("👉 Please install them manually:");
                println!("   Linux:  sudo apt install fonts-powerline");
                println!("   MacOS:  brew install --cask font-meslo-lg-nerd-font");
                println!("   Windows: Install Nerd Fonts from https://www.nerdfonts.com/");
            }
        }
        Err(_) => {
            println!("⚠️ Could not check fonts. Is `fontconfig` installed?");
        }
    }

    // Step 4: Append Powerline prompt configuration
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&rc_file)?;

    writeln!(file, "\n# >>> Powerline customization >>>")?;
    writeln!(file, "if command -v powerline-daemon >/dev/null 2>&1; then")?;
    writeln!(file, "  powerline-daemon -q")?;
    writeln!(file, "  POWERLINE_BASH_CONTINUATION=1")?;
    writeln!(file, "  POWERLINE_BASH_SELECT=1")?;
    writeln!(file, "  . /usr/share/powerline/bindings/bash/powerline.sh")?;
    writeln!(file, "fi")?;
    writeln!(file, "# <<< Powerline customization <<<")?;

    println!("✅ Powerline configuration added to {:?}", rc_file);
    println!("🔄 Restart your terminal or run: source {:?}", rc_file);

    Ok(())
}
