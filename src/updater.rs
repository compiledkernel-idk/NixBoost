use serde::Deserialize;
use console::style;
use std::fs;
use std::io::{self, Write};
use anyhow::Result;

#[derive(Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
    assets: Vec<GithubAsset>,
}

pub struct UpdateInfo {
    pub version: String,
    pub nixboost_url: Option<String>,
}

pub fn check_for_updates(current_version: &str) -> Option<UpdateInfo> {
    let url = "https://api.github.com/repos/NacreousDawn596/nixboost/releases/latest";
    
    let response = ureq::get(url)
        .set("User-Agent", "nixboost-updater")
        .timeout(std::time::Duration::from_secs(2))
        .call();

    if let Ok(res) = response {
        if let Ok(release) = res.into_json::<GithubRelease>() {
            let latest = release.tag_name.trim_start_matches('v');
            if latest != current_version {
                let mut info = UpdateInfo {
                    version: latest.to_string(),
                    nixboost_url: None,
                };
                for asset in release.assets {
                    if asset.name == "nixboost" {
                        info.nixboost_url = Some(asset.browser_download_url);
                    }
                }
                return Some(info);
            }
        }
    }
    None
}

pub fn perform_update(info: UpdateInfo) -> Result<()> {
    println!("{}", style(":: starting automatic update...").bold().cyan());

    let current_nixboost = std::env::current_exe()?;
    if let Some(url) = info.nixboost_url {
        update_binary("nixboost", &url, &current_nixboost)?;
    }

    println!("{}", style(":: update completed successfully.").green().bold());
    Ok(())
}

fn update_binary(name: &str, url: &str, target: &std::path::Path) -> Result<()> {
    print!("   fetching {}... ", name);
    io::stdout().flush()?;

    let response = ureq::get(url).call().map_err(|e| anyhow::anyhow!("failed to download {}: {}", name, e))?;
    let mut bytes = Vec::new();
    response.into_reader().read_to_end(&mut bytes)?;

    let temp_path = target.with_extension("tmp");
    fs::write(&temp_path, bytes)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&temp_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&temp_path, perms)?;
    }

    fs::rename(&temp_path, target).map_err(|e| {
        if e.kind() == io::ErrorKind::PermissionDenied {
            anyhow::anyhow!("permission denied: please run with sudo to update")
        } else {
            anyhow::anyhow!("failed to replace {}: {}", name, e)
        }
    })?;

    println!("{}", style("done").green());
    Ok(())
}