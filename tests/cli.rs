use std::process::Command;
fn cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rectangle-union-sweep"))
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn cli_json_and_failure_contracts() {
    let output = cli(&["area", "examples/overlapping.json"]);
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["union_area"], "48");
    assert!(
        cli(&["verify", "examples/overlapping.json"])
            .status
            .success()
    );
    assert!(!cli(&["area", "does-not-exist.json"]).status.success());
    let generated = cli(&["generate", "--pattern", "adjacent", "--count", "5"]);
    let ds: rectangle_union_sweep::Dataset = serde_json::from_slice(&generated.stdout).unwrap();
    assert_eq!(ds.rectangles.len(), 5);
}
