/*
 * nixboost - High-performance NixOS package manager frontend.
 */

use anyhow::{Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use console::style;
use std::time::Duration;
use dialoguer::{Confirm, theme::ColorfulTheme};

mod nix_manager;
mod cli;
mod nur;
mod utils;
mod updater;
mod arch;

use cli::{Cli, VERSION};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {msg}")?
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "));
    pb.set_message("checking for updates...");
    pb.enable_steady_tick(Duration::from_millis(80));

    if let Some(info) = updater::check_for_updates(VERSION) {
        pb.finish_and_clear();
        println!("{} a new version is available: {} -> {}", style("::").bold().cyan(), VERSION, info.version);
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Update now?")
            .default(true)
            .interact()?
        {
            if let Err(e) = updater::perform_update(info) {
                eprintln!("{} update failed: {}", style("error:").red().bold(), e);
            } else { 
                println!("   please restart nixboost."); 
                return Ok(()); 
            }
        }
    }
    pb.finish_and_clear();

    let manager = nix_manager::NixManager::new()?;

    // Handle utility commands
    if cli.news { return utils::fetch_nixos_news().await; }
    if cli.history { return utils::show_nix_history(); }
    if cli.health { return utils::run_health_check(); }
    if cli.clean { return utils::clean_nix_store(); }
    
    if cli.list {
        let installed = manager.list_installed().await?;
        println!("{}", style(":: installed packages:").bold());
        for pkg in installed { println!("   {}", pkg); }
        return Ok(());
    }

    if cli.nur { return nur::handle_nur_search(cli.targets).await; }

    // Search nixpkgs
    if cli.sync && cli.search {
        let results = manager.search(&cli.targets.join(" ")).await?;
        if results.is_empty() { 
            println!("no matches found."); 
        } else {
            for pkg in results {
                println!("{}/{} {}
    {}", style("nixpkgs").cyan().bold(), style(pkg.name).bold(), style(pkg.version).green(), pkg.description);
            }
        }
        return Ok(());
    }

    if cli.targets.is_empty() { return Ok(()); }

    // Install packages
    // Install packages (Batch)
    if cli.sync {
        let targets = &cli.targets;
        println!("{}", style(format!(":: installing {} package(s)...", targets.len())).bold());
        
        // Try batch install first
        if let Err(_) = manager.install(targets).await {
             println!("{}", style("! batch install failed (some packages might be missing from nixpkgs).").yellow());
             println!("{}", style(":: falling back to individual install & NUR check...").dim());
             
             // Fallback to individual + NUR
             for t in targets {
                 println!("{}", style(format!(":: installing {}...", t)).bold());
                 if let Err(_) = manager.install(&[t.clone()]).await {
                    println!("{}", style(format!("! {} not found in nixpkgs, checking NUR...", t)).yellow());
                    if let Err(ae) = nur::handle_nur_install(t).await {
                        eprintln!("{} failed to install {}: {}", style("error:").red().bold(), t, ae);
                    }
                 }
             }
        }
    } 
    // Remove packages (Batch)
    else if cli.remove {
        if cli.targets.is_empty() {
            let installed = manager.list_installed().await?;
            println!("{}", style(":: installed packages:").bold());
            for pkg in installed { println!("   {}", pkg); }
            println!("\nuse 'nixboost -R <package>' to remove one.");
            return Ok(());
        }

        println!("{} the following packages will be removed:", style("::").bold().yellow());
        for t in &cli.targets { println!("   {}", t); }
        
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Proceed with removal?")
            .default(true)
            .interact()?
        {
             println!("{}", style(format!(":: removing {} package(s)...", cli.targets.len())).bold());
             if let Err(e) = manager.remove(&cli.targets).await {
                  eprintln!("{} failed to remove packages: {}", style("error:").red().bold(), e);
             }
        } else {
            println!(":: removal cancelled.");
        }
    }

    println!("{}", style(":: operation finished.").green().bold());
    Ok(())
}
