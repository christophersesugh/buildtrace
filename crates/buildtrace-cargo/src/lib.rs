use std::path::Path;

use buildtrace_core::{Report, sort_report};

#[derive(Debug)]
pub struct InventoryError(String);

impl std::fmt::Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// The single test seam: everything (CLI and tests) goes through here.
///
/// v0.0.1 skeleton: validates the manifest is readable and returns an empty
/// report. Real package identities arrive with metadata support.
pub fn inventory(manifest_path: &Path) -> Result<Report, InventoryError> {
    std::fs::read(manifest_path).map_err(|err| {
        InventoryError(format!(
            "cannot read manifest {}: {err}",
            manifest_path.display()
        ))
    })?;
    let mut report = Report::default();
    sort_report(&mut report);
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn crate_manifest() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml")
    }

    #[test]
    fn inventory_accepts_readable_manifest() {
        let report = inventory(&crate_manifest()).expect("crate manifest reads");
        assert!(report.packages.is_empty());
    }

    #[test]
    fn inventory_rejects_missing_manifest() {
        let err = inventory(Path::new("does-not-exist-Cargo.toml")).expect_err("must fail");
        assert!(err.to_string().contains("cannot read manifest"));
    }
}
