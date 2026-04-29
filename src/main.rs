use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use include_dir::{include_dir, Dir, DirEntry};
use std::fs;
use std::path::Path;
use std::process::Command;

static TEMPLATES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");

#[derive(Parser)]
#[command(name = "mini")]
#[command(bin_name = "mini")]
#[command(about = "A mini CLI tool to manage projects", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new mini-project
    Init {
        /// Name of the project
        project_name: String,
        /// Language/Template to use
        #[arg(short, long, value_enum, default_value_t = Lang::C)]
        lang: Lang,
    },
    /// Run 'make' in the current directory
    Make,
    /// Run 'make clean' in the current directory
    Clean,
    /// Remove a project directory
    Remove {
        /// Name of the project directory to remove
        project_name: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Lang {
    C,
    #[value(name = "c-strict")]
    CStrict,
    Python,
    Rust,
}

impl Lang {
    fn as_str(&self) -> &'static str {
        match self {
            Lang::C => "c",
            Lang::CStrict => "c-strict",
            Lang::Python => "python",
            Lang::Rust => "rust",
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { project_name, lang } => {
            init_project(&project_name, lang)?;
        }
        Commands::Make => {
            run_make()?;
        }
        Commands::Clean => {
            run_make_clean()?;
        }
        Commands::Remove { project_name } => {
            remove_project(&project_name)?;
        }
    }

    Ok(())
}

fn extract_entry(entry: &DirEntry, base_path: &Path, strip_prefix: &Path) -> Result<()> {
    let relative_path = entry.path().strip_prefix(strip_prefix)
        .with_context(|| format!("Failed to strip prefix {:?} from {:?}", strip_prefix, entry.path()))?;
    
    match entry {
        DirEntry::Dir(dir) => {
            let path = base_path.join(relative_path);
            if !path.as_os_str().is_empty() {
                fs::create_dir_all(&path).with_context(|| format!("Failed to create dir {:?}", path))?;
            }
            for child in dir.entries() {
                extract_entry(child, base_path, strip_prefix)?;
            }
        }
        DirEntry::File(file) => {
            let path = base_path.join(relative_path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).with_context(|| format!("Failed to create dir {:?}", parent))?;
            }
            fs::write(&path, file.contents()).with_context(|| format!("Failed to write file {:?}", path))?;
        }
    }
    Ok(())
}

fn init_project(name: &str, lang: Lang) -> Result<()> {
    let dest = Path::new(name);
    if dest.exists() {
        anyhow::bail!("Directory '{}' already exists", name);
    }

    println!("Initializing {:?} project '{}'...", lang.as_str(), name);
    
    let template_path = Path::new(lang.as_str());
    let template_dir = TEMPLATES_DIR.get_dir(template_path)
        .with_context(|| format!("Template for {} not found in embedded assets", lang.as_str()))?;

    fs::create_dir_all(dest).context("Failed to create destination directory")?;

    for entry in template_dir.entries() {
        extract_entry(entry, dest, template_path)?;
    }

    // Post-extraction logic
    match lang {
        Lang::Python => {
            if Command::new("uv").arg("--version").output().is_ok() {
                println!("'uv' detected, initializing with 'uv init'...");
                let _ = Command::new("uv")
                    .arg("init")
                    .arg("--no-readme")
                    .current_dir(dest)
                    .status();
            }
        }
        Lang::Rust => {
            if Command::new("cargo").arg("--version").output().is_ok() {
                println!("'cargo' detected, initializing with 'cargo init'...");
                let _ = Command::new("cargo")
                    .arg("init")
                    .current_dir(dest)
                    .status();
            }
        }
        _ => {}
    }

    println!("Project '{}' initialized successfully.", name);
    Ok(())
}

fn run_make() -> Result<()> {
    println!("Running 'make'...");
    let status = Command::new("make")
        .status()
        .context("Failed to execute 'make'")?;

    if !status.success() {
        anyhow::bail!("'make' command failed");
    }
    Ok(())
}

fn run_make_clean() -> Result<()> {
    println!("Running 'make clean'...");
    let status = Command::new("make")
        .arg("clean")
        .status()
        .context("Failed to execute 'make clean'")?;

    if !status.success() {
        anyhow::bail!("'make clean' command failed");
    }
    Ok(())
}

fn remove_project(name: &str) -> Result<()> {
    let path = Path::new(name);
    if !path.exists() {
        anyhow::bail!("Directory '{}' does not exist", name);
    }

    println!("Removing project '{}'...", name);
    fs::remove_dir_all(path)
        .with_context(|| format!("Failed to remove directory '{}'", name))?;

    println!("Project '{}' removed successfully.", name);
    Ok(())
}
