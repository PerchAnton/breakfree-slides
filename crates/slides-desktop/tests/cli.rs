use std::process::Command;
const BIN: &str = env!("CARGO_BIN_EXE_breakfree-slides");
#[test]
fn capabilities_are_clean_json_without_gui_startup() {
    let out = Command::new(BIN).arg("--capabilities").output().unwrap();
    assert!(out.status.success());
    assert!(out.stderr.is_empty());
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["app_version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(v["app_id"], "breakfree.slides");
    let schema: serde_json::Value = serde_json::from_str(include_str!(
        "../../../contracts/office-v1/app-manifest.schema.json"
    ))
    .unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    validator
        .validate(&v)
        .expect("Actual CLI output must conform to pinned Office v1 schema");
    assert_eq!(v["capabilities"], serde_json::json!(["new", "open"]));
}
#[test]
fn version_exits_without_gui() {
    let out = Command::new(BIN).arg("--version").output().unwrap();
    assert!(out.status.success());
    assert_eq!(
        String::from_utf8(out.stdout).unwrap().trim(),
        env!("CARGO_PKG_VERSION")
    );
}
#[test]
fn invalid_arguments_exit_with_no_gui_and_no_stdout() {
    for args in [
        vec!["--unknown"],
        vec!["--open", "relative.bfslide"],
        vec!["--new", "--capabilities"],
    ] {
        let out = Command::new(BIN).args(args).output().unwrap();
        assert!(!out.status.success());
        assert!(out.stdout.is_empty());
        assert!(!out.stderr.is_empty());
    }
}
#[test]
fn damaged_open_exits_without_modifying_source() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("damaged.bfslide");
    std::fs::write(&path, b"malformed").unwrap();
    let out = Command::new(BIN).arg("--open").arg(&path).output().unwrap();
    assert!(!out.status.success());
    assert!(out.stdout.is_empty());
    assert_eq!(std::fs::read(path).unwrap(), b"malformed");
}
