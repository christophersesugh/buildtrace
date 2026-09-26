use std::path::Path;

use buildtrace_core::{ExecutableKind, PackageEntry, Report, sort_report};
use cargo_lock::Lockfile;
use cargo_metadata::camino::Utf8PathBuf;
use cargo_metadata::{Metadata, MetadataCommand, Package, TargetKind};

#[derive(Debug)]
pub struct InventoryError(String);

impl std::fmt::Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Read-only resolution query via the maintained `cargo_metadata` crate.
///
/// `cargo metadata` resolves the dependency graph from manifests, the
/// lockfile, and cached sources. It never compiles or executes dependency
/// code and writes no build output. Build commands (`build`, `check`,
/// `test`, `run`) are never invoked — that is the "never run a `cargo`
/// command" guarantee in ADR-0002.
fn load_metadata(manifest_path: &Path) -> Result<Metadata, InventoryError> {
    MetadataCommand::new()
        .manifest_path(manifest_path)
        .exec()
        .map_err(|err| {
            InventoryError(format!(
                "cannot read manifest {}: {err}",
                manifest_path.display()
            ))
        })
}

struct LockRow {
    name: String,
    version: String,
    source: Option<String>,
    checksum: Option<String>,
}

/// Checksums and revisions come from the lockfile next to the target
/// workspace root, parsed with the maintained `cargo-lock` crate — never a
/// hand-rolled parser, per `plan.md`. A missing lockfile means checksums are
/// unknown, not an error; an unreadable one fails loudly.
fn load_lock_rows(workspace_root: &Utf8PathBuf) -> Result<Vec<LockRow>, InventoryError> {
    let lock_path = workspace_root.join("Cargo.lock");
    if !lock_path.is_file() {
        return Ok(Vec::new());
    }
    let lockfile = Lockfile::load(lock_path.as_std_path())
        .map_err(|err| InventoryError(format!("cannot parse lockfile {lock_path}: {err}")))?;
    Ok(lockfile
        .packages
        .iter()
        .map(|package| LockRow {
            name: package.name.to_string(),
            version: package.version.to_string(),
            source: package.source.as_ref().map(ToString::to_string),
            checksum: package.checksum.as_ref().map(ToString::to_string),
        })
        .collect())
}

/// Best-effort checksum-or-revision for one package: an exact
/// (name, version, source) lockfile hit wins, otherwise the first checksum
/// recorded for that name and version. A Git revision embedded in the source
/// string (after `#`) is the fallback when the lockfile has nothing.
fn checksum_for(rows: &[LockRow], name: &str, version: &str, source: &str) -> Option<String> {
    let mut fallback = None;
    for row in rows
        .iter()
        .filter(|row| row.name == name && row.version == version)
    {
        let Some(checksum) = row.checksum.clone() else {
            continue;
        };
        if row.source.as_deref() == Some(source) {
            return Some(checksum);
        }
        if fallback.is_none() {
            fallback = Some(checksum);
        }
    }
    fallback.or_else(|| git_revision(source))
}

fn git_revision(source: &str) -> Option<String> {
    source.split('#').nth(1).map(str::to_string)
}

/// Exact source string. Workspace members carry no registry/Git/path source
/// in metadata, so their identity falls back to the manifest directory as a
/// `path+file://` URL — still exact, still local.
fn source_of(package: &Package) -> String {
    if let Some(source) = &package.source {
        return source.repr.clone();
    }
    match package.manifest_path.parent() {
        Some(dir) => format!("path+file://{dir}"),
        None => "path+unknown".to_string(),
    }
}

/// Which build-time executable kinds a package contributes, if any.
/// Detection only: custom build targets for build scripts, `proc-macro`
/// target kinds for procedural macros. Nothing is compiled or executed.
fn kinds_of(package: &Package) -> Vec<ExecutableKind> {
    let mut kinds = Vec::new();
    let has = |want: TargetKind| {
        package
            .targets
            .iter()
            .any(|target| target.kind.contains(&want))
    };
    if has(TargetKind::CustomBuild) {
        kinds.push(ExecutableKind::BuildScript);
    }
    if has(TargetKind::ProcMacro) {
        kinds.push(ExecutableKind::ProcMacro);
    }
    kinds
}

/// The single test seam: everything (CLI and tests) goes through here.
pub fn inventory(manifest_path: &Path) -> Result<Report, InventoryError> {
    let metadata = load_metadata(manifest_path)?;
    let lock_rows = load_lock_rows(&metadata.workspace_root)?;
    let mut report = Report::default();
    for package in &metadata.packages {
        let name = package.name.to_string();
        let version = package.version.to_string();
        let source = source_of(package);
        let checksum = checksum_for(&lock_rows, &name, &version, &source);
        for kind in kinds_of(package) {
            report.packages.push(PackageEntry {
                name: name.clone(),
                version: version.clone(),
                source: source.clone(),
                checksum: checksum.clone(),
                kind,
            });
        }
    }
    sort_report(&mut report);
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use buildtrace_core::ExecutableKind;

    fn crate_manifest() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml")
    }

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures")
            .join(name)
            .join("Cargo.toml")
    }

    #[test]
    fn inventory_succeeds_on_readable_manifest() {
        inventory(&crate_manifest()).expect("crate manifest reads");
    }

    #[test]
    fn inventory_rejects_missing_manifest() {
        let err = inventory(Path::new("does-not-exist-Cargo.toml")).expect_err("must fail");
        assert!(err.to_string().contains("cannot read manifest"));
    }

    #[test]
    fn build_script_fixture_lists_build_script_with_identity() {
        let report = inventory(&fixture("has-build-script")).expect("fixture reads");
        assert_eq!(report.packages.len(), 1);
        let entry = &report.packages[0];
        assert_eq!(entry.name, "fixture-build-script");
        assert_eq!(entry.version, "0.1.0");
        assert!(
            entry.source.starts_with("path+"),
            "source: {}",
            entry.source
        );
        assert_eq!(entry.checksum, None);
        assert_eq!(entry.kind, ExecutableKind::BuildScript);
    }

    #[test]
    fn packages_without_build_targets_are_excluded() {
        let report = inventory(&fixture("has-build-script")).expect("fixture reads");
        assert!(
            report.packages.iter().all(|e| e.name != "fixture-plain"),
            "plain lib must be excluded: {:?}",
            report.packages
        );
    }

    #[test]
    fn proc_macro_fixture_lists_proc_macro_with_identity() {
        let report = inventory(&fixture("has-proc-macro")).expect("fixture reads");
        assert_eq!(report.packages.len(), 1);
        let entry = &report.packages[0];
        assert_eq!(entry.name, "fixture-proc-macro");
        assert_eq!(entry.version, "0.1.0");
        assert!(
            entry.source.starts_with("path+"),
            "source: {}",
            entry.source
        );
        assert_eq!(entry.checksum, None);
        assert_eq!(entry.kind, ExecutableKind::ProcMacro);
    }

    #[test]
    fn neither_fixture_reports_empty() {
        let report = inventory(&fixture("has-neither")).expect("fixture reads");
        assert!(report.packages.is_empty());
    }
}
