// src/commands/repository.rs
use crate::cli::RepositoryAction;
use crate::config::{get_repos_file, get_sync_dir};

use std::fs::{File, OpenOptions, read_to_string, write};
use std::io::{BufRead, BufReader, Write};

pub fn run(action: RepositoryAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        RepositoryAction::Add { url } => add(url),
        RepositoryAction::Remove { url } => remove(url),
        RepositoryAction::List => list(),
        RepositoryAction::Sync { url } => sync(url),
        RepositoryAction::Check => check(),
    }
}

// function handler for add subcommand
fn add(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let repos_file = get_repos_file();

    // read pass
    if repos_file.exists() {
        let reader = BufReader::new(File::open(&repos_file)?);
        for line in reader.lines() {
            if line? == url {
                println!("Repository {} already exists.", url);
                return Ok(());
            }
        }
    }

    // append pass
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&repos_file)?;

    writeln!(file, "{}", url)?;
    Ok(())
}

// function handler for remove command
fn remove(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let repos_file = get_repos_file();
    let repos = read_to_string(repos_file)?;

    if !repos.lines().any(|line| line == url) {
        eprintln!("Repository {} not found.", url);
        return Ok(());
    }

    let new_repos_content: String = repos
        .lines()
        .filter(|line| *line != url)
        .collect::<Vec<&str>>()
        .join("\n")
        + "\n";

    let _ = write(get_repos_file(), new_repos_content);
    Ok(())
}

// function handler for list command
fn list() -> Result<(), Box<dyn std::error::Error>> {
    let repos_file = get_repos_file();
    if !repos_file.exists() {
        println!("No repositores added yet.");
        return Ok(());
    }
    println!("{}", read_to_string(repos_file)?);
    Ok(())
}

// function handler for command sync
fn sync(url: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut repos: Vec<String> = Vec::new();
    if !url.is_empty() {
        for single_url in url {
            repos.push(single_url);
        }
    } else {
        let repos_file = get_repos_file();
        if !repos_file.exists() {
            println!("No repositories added yet.");
            return Ok(());
        }
        repos = read_to_string(repos_file)?
            .lines()
            .map(String::from)
            .collect::<Vec<String>>();
    }

    let mut success = 0;
    let mut error = 0;
    let sync_dir = get_sync_dir();

    for url in &repos {
        if url.is_empty() {
            continue;
        }

        println!("Syncing {}...", url);
        let content = match reqwest::blocking::get(url) {
            Ok(response) => match response.error_for_status() {
                Ok(r) => match r.text() {
                    Ok(text) => text,
                    Err(e) => {
                        eprintln!(" Failed to read response from {}: {}.", url, e);
                        error += 1;
                        continue;
                    }
                },
                Err(e) => {
                    eprintln!(" Failed to fetch {}: {}.", url, e);
                    error += 1;
                    continue;
                }
            },
            Err(e) => {
                eprintln!(" Failed to fetch {}: {}.", url, e);
                error += 1;
                continue;
            }
        };

        // turn URL into a safe filename
        let filename = url.replace("https://", "").replace("/", "_");
        let dest = sync_dir.join(filename);

        write(dest, content)?;
        success += 1;
        println!("  Ok.");
    }

    println!("{} Repository synced and {} failed.", success, error);
    Ok(())
}

// function handler for command check
fn check() -> Result<(), Box<dyn std::error::Error>> {
    let repos_file = get_repos_file();
    let repos: Vec<String> = read_to_string(repos_file)?
        .lines()
        // Trim empty line
        .filter(|l| !l.trim().is_empty())
        .map(|s| s.to_string())
        .collect();

    if repos.is_empty() {
        println!("No repository found.");
        return Ok(())
    }
    println!("Found {} repositories.", repos.len());

    let mut alive = 0;
    let mut dead = 0;
    for url in repos {
        println!("Checking: {}", url);
        match reqwest::blocking::Client::new().head(&url).send() {
            Ok(response) => match response.error_for_status() {
                Ok(_) => {
                    println!("  Alive");
                    alive += 1;
                }
                Err(e) => {
                    eprintln!("  Dead: {}", e);
                    dead += 1;
                }
            },
            Err(e) => {
                eprintln!("  Unreachable: {}", e);
                dead += 1;
            }
        }
        alive += 1;
        println!(" Repository okay.");
    }
    println!("Checking done:\n Alive: {}\n Dead: {}", alive, dead);
    Ok(())
}
