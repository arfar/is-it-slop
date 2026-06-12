use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug)]
struct Args {
    github_project: String,
}

#[derive(Deserialize, Debug)]
struct CargoToml {
    workspace: Option<Workspace>,
    package: Option<Package>,
}

#[derive(Deserialize, Debug)]
struct Workspace {
    resolver: String,
    package: Option<Package>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Package {
    edition: Option<String>,
    // rust_version: Option<String>, // TODO:
}

// TODO:
#[derive(Deserialize, Debug)]
struct Dependencies {}

fn is_old_edition(edition_str: &str) -> bool {
    edition_str.parse::<u16>().unwrap() < 2024
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let cargo_toml_raw_url = format!(
        "https://raw.githubusercontent.com/{}/HEAD/Cargo.toml",
        args.github_project
    );

    let cargo_toml_str = ureq::get(cargo_toml_raw_url)
        .call()?
        .body_mut()
        .read_to_string()?;

    let cargo_toml: CargoToml = toml::from_str(&cargo_toml_str)?;

    let mut slop_score = 0;
    let mut slop_score_motivations = Vec::new();

    if let Some(package) = cargo_toml.package
        && let Some(edition) = package.edition
        && is_old_edition(&edition)
    {
        slop_score += 1;
        slop_score_motivations.push(format!("is using old Rust edition ({})", edition));
    }

    if let Some(workspace) = cargo_toml.workspace {
        if let Some(package) = workspace.package
            && let Some(edition) = package.edition
            && is_old_edition(&edition)
        {
            slop_score += 1;
            slop_score_motivations.push(format!("is using old Rust edition ({})", edition));
        }
        if workspace.resolver.parse::<u8>().unwrap() < 3 {
            slop_score += 1;
            slop_score_motivations.push(format!(
                "is using old workspace resolver ({})",
                workspace.resolver
            ));
        }
    }

    println!("slop score: {}", slop_score);

    for motivation in slop_score_motivations {
        print!("- ");
        println!("{}", motivation);
    }

    Ok(())
}
