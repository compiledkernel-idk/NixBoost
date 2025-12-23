use anyhow::{Result, anyhow};
use console::style;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;
use serde_json::Value;

pub async fn load_nur_index() -> Result<Value> {
    let home = std::env::var("HOME").map_err(|_| anyhow!("could not find HOME directory"))?;
    let cache_dir = std::path::PathBuf::from(home).join(".cache/nixboost");
    std::fs::create_dir_all(&cache_dir)?;
    let cache_file = cache_dir.join("nur-packages.json");
    
    let mut download_needed = true;
    if cache_file.exists() {
        if let Ok(metadata) = std::fs::metadata(&cache_file) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(elapsed) = modified.elapsed() {
                    if elapsed.as_secs() < 86400 {
                        download_needed = false;
                    }
                }
            }
        }
    }
    
    if download_needed {
        println!("{}", style(":: updating NUR package index...").dim());
        let res = reqwest::get("https://raw.githubusercontent.com/nix-community/nur-search/master/data/packages.json").await?;
        if res.status().is_success() {
            let bytes = res.bytes().await?;
            std::fs::write(&cache_file, bytes)?;
        } else {
            return Err(anyhow!("failed to update NUR index"));
        }
    }
    
    let content = std::fs::read_to_string(&cache_file)?;
    let json: Value = serde_json::from_str(&content)?;
    Ok(json)
}

pub async fn resolve_nur_path(pkg_name: &str) -> Result<Option<String>> {
    let json = load_nur_index().await?;
    if let Some(obj) = json.as_object() {
        let query = pkg_name.to_lowercase();
        for (key, _) in obj {
            if key.to_lowercase().ends_with(&format!(".{}", query)) || key.to_lowercase() == query {
                return Ok(Some(key.clone()));
            }
        }
        for (key, _) in obj {
            if key.to_lowercase().contains(&query) {
                return Ok(Some(key.clone()));
            }
        }
    }
    Ok(None)
}

pub async fn handle_nur_search(targets: Vec<String>) -> Result<()> {
    if targets.is_empty() { return Err(anyhow!("no targets specified for NUR search")); }
    println!("{}", style(":: searching NUR...").bold());
    
    let json = match load_nur_index().await {
        Ok(j) => j,
        Err(e) => {
            println!("{}", style(format!("! failed to load NUR index: {}", e)).yellow());
            return Ok(());
        }
    };
    
    if let Some(obj) = json.as_object() {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_header(vec!["Attribute Path", "Version", "Description"]);
        
        let mut found = false;
        for t in targets {
            let query = t.to_lowercase();
            for (key, val) in obj {
                let description = val["meta"]["description"].as_str().unwrap_or("");
                if key.to_lowercase().contains(&query) || description.to_lowercase().contains(&query) {
                    let version = val["version"].as_str().unwrap_or("");
                    table.add_row(vec![
                        style(key).magenta().to_string(),
                        style(version).green().to_string(),
                        description.to_string()
                    ]);
                    found = true;
                }
            }
        }
        
        if found {
            println!("{}", table);
        } else {
            println!("{}", style("! no matches found in NUR").yellow());
        }
    }
    
    Ok(())
}

pub async fn handle_nur_install(pkg_name: &str) -> Result<()> {
    let mut attr_path = pkg_name.strip_prefix("nur.").unwrap_or(pkg_name).to_string();
    
    if !attr_path.contains("repos.") {
        println!("{}", style(format!("! {} is not a full NUR path, attempting to resolve...", pkg_name)).dim());
        if let Some(resolved) = resolve_nur_path(&attr_path).await? {
            println!("{}", style(format!(":: resolved {} to {}", pkg_name, resolved)).cyan());
            attr_path = resolved.strip_prefix("nur.").unwrap_or(&resolved).to_string();
        }
    }

    println!("{}", style(format!(":: installing {} from NUR...", attr_path)).bold());
    
    let status = std::process::Command::new("nix")
        .args(["profile", "install", &format!("github:nix-community/NUR#{}", attr_path)])
        .status()?;
        
    if !status.success() {
        return Err(anyhow!("NUR installation failed. Ensure the attribute path is correct (e.g., repos.user.pkg)"));
    }
    Ok(())
}
