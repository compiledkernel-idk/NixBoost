use anyhow::{Result, anyhow};
use std::process::Command;

/// Detects the current system's Nix architecture (e.g., "x86_64-linux", "aarch64-darwin").
/// It runs `nix eval --raw --impure --expr builtins.currentSystem` to get the truth.
pub fn get_system_arch() -> Result<String> {
    let output = Command::new("nix")
        .args(["eval", "--raw", "--impure", "--expr", "builtins.currentSystem"])
        .output()?;
        
    if !output.status.success() {
        // Fallback: try `uname -m` and mapping it manually if nix eval fails (e.g. strict mode)
        // But for now, let's just return a useful error or default to x86_64-linux if desperate?
        // Let's stick to the error, because if `nix` fails, we are doomed anyway.
        return Err(anyhow!("failed to detect system architecture via nix eval"));
    }
    
    let arch = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(arch)
}
