use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;

const REPORT_VERSION: u32 = 1;

#[derive(Debug, Parser)]
#[command(
    name = "cargo-buildtrace",
    version,
    about = "Record what dependency build scripts and procedural macros do during a build"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List build-time executables without executing them
    Inventory {
        /// Path to the manifest to inspect (defaults to ./Cargo.toml)
        #[arg(long, default_value = "Cargo.toml")]
        manifest_path: PathBuf,
        /// Output format
        #[arg(long, value_enum, default_value_t = Format::Human)]
        format: Format,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Format {
    Human,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum ExecutableKind {
    BuildScript,
    ProcMacro,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct PackageEntry {
    name: String,
    version: String,
    source: String,
    checksum: Option<String>,
    kind: ExecutableKind,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Report {
    packages: Vec<PackageEntry>,
}

#[derive(Debug)]
struct InventoryError(String);

impl std::fmt::Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

fn sort_report(report: &mut Report) {
    report
        .packages
        .sort_by(|a, b| a.name.cmp(&b.name).then_with(|| a.version.cmp(&b.version)));
}

/// The single test seam: everything (CLI and tests) goes through here.
///
/// v0.0.1 skeleton: validates the manifest is readable and returns an empty
/// report. Real package identities arrive with metadata support.
fn inventory(manifest_path: &Path) -> Result<Report, InventoryError> {
    if !manifest_path.is_file() {
        return Err(InventoryError(format!(
            "manifest not found: {}",
            manifest_path.display()
        )));
    }
    let mut report = Report::default();
    sort_report(&mut report);
    Ok(report)
}

fn render_human(report: &Report) -> String {
    let mut out = String::from("Build-time executables in this dependency graph\n");
    for (title, kind) in [
        ("Build scripts", ExecutableKind::BuildScript),
        ("Procedural macros", ExecutableKind::ProcMacro),
    ] {
        out.push_str(&format!("\n{title}\n"));
        let mut any = false;
        for entry in report.packages.iter().filter(|e| e.kind == kind) {
            any = true;
            out.push_str(&format!(
                "  {} {}    {}\n",
                entry.name, entry.version, entry.source
            ));
        }
        if !any {
            out.push_str("  (none)\n");
        }
    }
    out
}

#[derive(Serialize)]
struct JsonReport<'a> {
    version: u32,
    packages: &'a [PackageEntry],
}

fn render_json(report: &Report) -> String {
    let payload = JsonReport {
        version: REPORT_VERSION,
        packages: &report.packages,
    };
    serde_json::to_string_pretty(&payload).expect("report serializes")
}

fn run(cli: Cli) -> Result<(), InventoryError> {
    let Command::Inventory {
        manifest_path,
        format,
    } = cli.command;
    let report = inventory(&manifest_path)?;
    match format {
        Format::Human => print!("{}", render_human(&report)),
        Format::Json => println!("{}", render_json(&report)),
    }
    Ok(())
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn empty_report_renders_both_sections() {
        let text = render_human(&Report::default());
        assert!(text.contains("Build scripts"));
        assert!(text.contains("Procedural macros"));
    }

    #[test]
    fn empty_report_json_is_versioned_and_empty() {
        let value: Value =
            serde_json::from_str(&render_json(&Report::default())).expect("valid JSON");
        assert_eq!(value["version"], Value::from(REPORT_VERSION));
        assert_eq!(value["packages"], Value::Array(vec![]));
    }

    #[test]
    fn inventory_accepts_readable_manifest() {
        let report = inventory(Path::new("Cargo.toml")).expect("repo manifest reads");
        assert!(report.packages.is_empty());
    }

    #[test]
    fn inventory_rejects_missing_manifest() {
        let err = inventory(Path::new("does-not-exist-Cargo.toml")).expect_err("must fail");
        assert!(err.to_string().contains("manifest not found"));
    }

    #[test]
    fn report_sorts_by_name_then_version() {
        let mut report = Report {
            packages: vec![
                entry("b", "2.0.0"),
                entry("a", "1.1.0"),
                entry("a", "1.0.0"),
            ],
        };
        sort_report(&mut report);
        let names: Vec<(&str, &str)> = report
            .packages
            .iter()
            .map(|e| (e.name.as_str(), e.version.as_str()))
            .collect();
        assert_eq!(names, vec![("a", "1.0.0"), ("a", "1.1.0"), ("b", "2.0.0")]);
    }

    fn entry(name: &str, version: &str) -> PackageEntry {
        PackageEntry {
            name: name.to_string(),
            version: version.to_string(),
            source: "path+test".to_string(),
            checksum: None,
            kind: ExecutableKind::BuildScript,
        }
    }
}
