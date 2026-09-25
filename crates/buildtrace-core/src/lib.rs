use serde::Serialize;

pub const REPORT_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExecutableKind {
    BuildScript,
    ProcMacro,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PackageEntry {
    pub name: String,
    pub version: String,
    pub source: String,
    pub checksum: Option<String>,
    pub kind: ExecutableKind,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Report {
    pub packages: Vec<PackageEntry>,
}

pub fn sort_report(report: &mut Report) {
    report
        .packages
        .sort_by(|a, b| a.name.cmp(&b.name).then_with(|| a.version.cmp(&b.version)));
}

pub fn render_human(report: &Report) -> String {
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

pub fn render_json(report: &Report) -> String {
    let payload = JsonReport {
        version: REPORT_VERSION,
        packages: &report.packages,
    };
    serde_json::to_string_pretty(&payload).expect("report serializes")
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
