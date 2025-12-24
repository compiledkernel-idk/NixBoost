use anyhow::{Result, anyhow};
use tokio::process::Command;
use serde_json::Value;
use crate::arch;

pub struct NixManager;

impl NixManager {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub async fn search(&self, query: &str) -> Result<Vec<NixPackage>> {
        let system_arch = arch::get_system_arch().unwrap_or_else(|_| "x86_64-linux".to_string());
        let legacy_prefix = format!("legacyPackages.{}.", system_arch);
        
        let output = Command::new("nix")
            .args(["search", "--json", "nixpkgs", query])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!("nix search failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        let json: Value = serde_json::from_slice(&output.stdout)?;
        let mut results = Vec::new();

        if let Some(obj) = json.as_object() {
            for (key, val) in obj {
                // Remove prefix like "legacyPackages.x86_64-linux." or just "legacyPackages.<arch>."
                let name = key.strip_prefix(&legacy_prefix)
                     .or_else(|| key.strip_prefix("legacyPackages.x86_64-linux.")) // Fallback just in case
                     .unwrap_or(key).to_string();
                     
                let version = val["version"].as_str().unwrap_or("unknown").to_string();
                let description = val["description"].as_str().unwrap_or("").to_string();
                results.push(NixPackage { name, version, description });
            }
        }
        Ok(results)
    }

    pub async fn install(&self, pkgs: &[String]) -> Result<()> {
        if pkgs.is_empty() { return Ok(()); }
        
        // Prepare arguments: "nixpkgs#pkg1", "nixpkgs#pkg2", ...
        let install_args: Vec<String> = pkgs.iter()
            .map(|p| format!("nixpkgs#{}", p))
            .collect();
            
        let mut args = vec!["profile", "install"];
        let mut args_str = install_args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        args.append(&mut args_str);

        let status = Command::new("nix")
            .args(&args)
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("nix profile install failed for batch"));
        }
        Ok(())
    }

    pub async fn remove(&self, pkgs: &[String]) -> Result<()> {
        if pkgs.is_empty() { return Ok(()); }

        let mut args = vec!["profile", "remove"];
        let mut args_str = pkgs.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        args.append(&mut args_str);

        let status = Command::new("nix")
            .args(&args)
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("nix profile remove failed for batch"));
        }
        Ok(())
    }

    pub async fn list_installed(&self) -> Result<Vec<String>> {
        let output = Command::new("nix")
            .args(["profile", "list", "--json"])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!("nix profile list failed"));
        }

        let json: Value = serde_json::from_slice(&output.stdout)?;
        let mut installed = Vec::new();

        if let Some(elements) = json["elements"].as_object() {
            for (name, _) in elements {
                installed.push(name.clone());
            }
        }

        installed.sort();
        Ok(installed)
    }
}

pub struct NixPackage {
    pub name: String,
    pub version: String,
    pub description: String,
}
