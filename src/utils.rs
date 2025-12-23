use anyhow::{Result, anyhow};
use console::style;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;

pub async fn fetch_nixos_news() -> Result<()> {
    println!("{}", style(":: fetching nixos news...").bold());
    let client = reqwest::Client::new();
    let res = client.get("https://nixos.org/blog/feed.xml").send().await?.text().await?;
    let channel = rss::Channel::read_from(res.as_bytes()).map_err(|e| anyhow!("failed to parse rss: {}", e))?;
    
    let mut t = Table::new();
    t.load_preset(UTF8_FULL);
    t.set_header(vec!["Date", "Title"]);
    for item in channel.items().iter().take(5) {
        t.add_row(vec![item.pub_date().unwrap_or(""), item.title().unwrap_or("")]);
    }
    println!("{}", t);
    Ok(())
}

pub fn show_nix_history() -> Result<()> {
    println!("{}", style(":: nix-env history (last 20 entries)...").bold());
    let output = std::process::Command::new("nix-env").arg("--list-generations").output()?;
    if !output.status.success() { return Err(anyhow!("failed to list generations")); }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<_> = stdout.lines().rev().take(20).collect();
    
    let mut t = Table::new();
    t.load_preset(UTF8_FULL);
    t.set_header(vec!["Generation"]);
    for line in lines { t.add_row(vec![line]); }
    println!("{}", t);
    Ok(())
}

pub fn run_health_check() -> Result<()> {
    println!("{}", style(":: running nixos health check...").bold());
    
    let output = std::process::Command::new("systemctl").args(["--failed", "--quiet"]).output()?;
    if !output.status.success() { println!("{}", style("! some systemd services have failed").red()); }
    else { println!("{}", style("✓ all systemd services are running fine").green()); }
    
    println!(":: checking nix store integrity...");
    let output = std::process::Command::new("nix-store").arg("--verify").output()?;
    if output.status.success() { println!("{}", style("✓ nix store is healthy").green()); }
    else { println!("{}", style("! nix store issues detected").yellow()); }
    
    Ok(())
}

pub fn clean_nix_store() -> Result<()> {
    println!("{}", style(":: collecting nix garbage...").bold());
    let status = std::process::Command::new("nix-collect-garbage").arg("-d").status()?;
    if status.success() { println!("{}", style("✓ garbage collection finished").green()); }
    else { println!("{}", style("! garbage collection failed").red()); }
    Ok(())
}
