use anyhow::{Result, anyhow};
use std::process::Command;
use serde_json::Value;

pub struct NixManager;

impl NixManager {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn search(&self, query: &str) -> Result<Vec<NixPackage>> {
        let output = Command::new("nix")
            .args(["search", "--json", "nixpkgs", query])
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("nix search failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        let json: Value = serde_json::from_slice(&output.stdout)?;
        let mut results = Vec::new();

        if let Some(obj) = json.as_object() {
            for (key, val) in obj {
                let name = key.strip_prefix("legacyPackages.x86_64-linux.").unwrap_or(key).to_string();
                let version = val["version"].as_str().unwrap_or("unknown").to_string();
                let description = val["description"].as_str().unwrap_or("").to_string();
                results.push(NixPackage { name, version, description });
            }
        }
        Ok(results)
    }

    pub fn install(&self, pkg: &str) -> Result<()> {
        let status = Command::new("nix")
            .args(["profile", "install", &format!("nixpkgs#{}", pkg)])
            .status()?;

        if !status.success() {
            return Err(anyhow!("nix profile install failed"));
        }
        Ok(())
    }

    pub fn remove(&self, pkg: &str) -> Result<()> {
        let status = Command::new("nix")
            .args(["profile", "remove", pkg])
            .status()?;

        if !status.success() {
            return Err(anyhow!("nix profile remove failed"));
        }
        Ok(())
    }

    pub fn list_installed(&self) -> Result<Vec<String>> {
        let output = Command::new("nix")
            .args(["profile", "list", "--json"])
            .output()?;

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
