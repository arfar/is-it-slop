use color_eyre::eyre::Context;
use jiff::Timestamp;
use serde::Deserialize;
use ureq::Agent;

#[derive(Deserialize, Debug)]
pub struct GithubRepoDetails {
    pub created_at: Timestamp,
}

static SUSSY_FILES: &[&str] = &["AGENTS.md", "CLAUDE.md"];

pub fn fetch_repo_details(
    github_project: &str,
    agent: &Agent,
) -> color_eyre::Result<GithubRepoDetails> {
    agent
        .get(String::from("https://api.github.com/repos/") + github_project)
        .call()
        .wrap_err("couldn't fetch repo details, are you sure it exists?")?
        .body_mut()
        .read_json()
        .map_err(color_eyre::Report::from)
}

pub fn find_sussy_files(github_project: &str, agent: &Agent) -> Vec<String> {
    SUSSY_FILES
        .iter()
        .filter_map(|sussy_file| {
            agent
                .get(format!(
                    "https://raw.githubusercontent.com/{}/HEAD/{}",
                    github_project, sussy_file
                ))
                .call()
                .is_ok()
                .then_some(sussy_file.to_string())
        })
        .collect()
}

pub fn fetch_gitignore(github_project: &str, agent: &Agent) -> color_eyre::Result<String> {
    Ok(agent
        .get(format!(
            "https://raw.githubusercontent.com/{}/HEAD/.gitignore",
            github_project
        ))
        .call()?
        .body_mut()
        .read_to_string()?)
}

pub fn find_ai_string_in_gitignore(gitignore: &str) -> Vec<String> {
    println!("checking for AI related strings in .gitignore");
    let mut common_ai_strings_found = Vec::new();
    for line in gitignore.lines() {
        if line.to_ascii_lowercase().contains("claude.md") {
            common_ai_strings_found.push(line.to_string());
        }
        if line.to_ascii_lowercase().contains("agents.md") {
            common_ai_strings_found.push(line.to_string());
        }
        if line.to_ascii_lowercase().contains("copilot-instructions") {
            common_ai_strings_found.push(line.to_string());
        }
        if line.to_ascii_lowercase().contains("cursor/rules") {
            common_ai_strings_found.push(line.to_string());
        }
        if line.to_ascii_lowercase().contains("codex/rules") {
            common_ai_strings_found.push(line.to_string());
        }
        if line.to_ascii_lowercase().contains(".hermes/soul") {
            common_ai_strings_found.push(line.to_string());
        }
    }
    common_ai_strings_found
}
