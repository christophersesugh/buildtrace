use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn binary() -> PathBuf {
    Path::new(env!("CARGO_BIN_EXE_cargo-buildtrace")).to_path_buf()
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(name)
        .join("Cargo.toml")
}

fn inventory(args: &[&str]) -> Output {
    Command::new(binary())
        .arg("inventory")
        .args(args)
        .output()
        .expect("inventory runs")
}

fn stdout_text(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout is utf8")
}

#[test]
fn build_script_fixture_lists_package_and_exits_zero() {
    let manifest = fixture("has-build-script");
    let output = inventory(&["--manifest-path", &manifest.to_string_lossy()]);
    assert!(output.status.success());
    let text = stdout_text(&output);
    assert!(text.contains("Build scripts"));
    assert!(text.contains("fixture-build-script 0.1.0"));
    assert!(!text.contains("fixture-plain"));
}

#[test]
fn proc_macro_fixture_json_is_versioned_and_exits_zero() {
    let manifest = fixture("has-proc-macro");
    let output = inventory(&[
        "--format",
        "json",
        "--manifest-path",
        &manifest.to_string_lossy(),
    ]);
    assert!(output.status.success());
    let text = stdout_text(&output);
    assert!(text.contains("\"version\": 1"));
    assert!(text.contains("fixture-proc-macro"));
    assert!(text.contains("proc-macro"));
}

#[test]
fn neither_fixture_reports_empty_and_exits_zero() {
    let manifest = fixture("has-neither");
    let output = inventory(&["--manifest-path", &manifest.to_string_lossy()]);
    assert!(output.status.success());
    let text = stdout_text(&output);
    assert!(text.contains("Procedural macros"));
    assert!(text.contains("(none)"));
}

#[test]
fn missing_manifest_exits_two_with_diagnostic() {
    let output = inventory(&["--manifest-path", "does-not-exist-Cargo.toml"]);
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).expect("stderr is utf8");
    assert!(stderr.contains("cannot read manifest"));
}
