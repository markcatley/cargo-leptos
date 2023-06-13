use crate::compile::{front_cargo_process, server_cargo_process};
use crate::config::{Config, Project};
use crate::ext::anyhow::{anyhow, Context, Result};
use crate::logger::GRAY;

pub async fn test_all(conf: &Config) -> Result<()> {
    for proj in &conf.projects {
        if !test_proj(proj).await? {
            return Err(anyhow!("Tests failed for {}", proj.name));
        }
    }

    Ok(())
}

pub async fn test_proj(proj: &Project) -> Result<bool> {
    let (envs, line, mut proc) = server_cargo_process("test", proj).dot()?;

    let server_exit_status = proc.wait().await.dot()?;
    log::debug!("Cargo envs: {}", GRAY.paint(envs));
    log::info!("Cargo server tests finished {}", GRAY.paint(line));

    let (envs, line, mut proc) = front_cargo_process("test", false, proj).dot()?;

    let front_exit_status = proc.wait().await.dot()?;
    log::debug!("Cargo envs: {}", GRAY.paint(envs));
    log::info!("Cargo front tests finished {}", GRAY.paint(line));

    Ok(server_exit_status.success() && front_exit_status.success())
}
