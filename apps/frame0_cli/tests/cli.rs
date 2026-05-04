use assert_cmd::Command;
use predicates::prelude::*;
use std::{fs, path::PathBuf};
use tempfile::tempdir;

fn repo_path(path: &str) -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
        .display()
        .to_string()
}

#[test]
fn version_command_runs() {
    Command::cargo_bin("frame0")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("frame0"));
}

#[test]
fn inspect_outputs_json() {
    let scene = repo_path("examples/hello_shader/scene.yaml");
    Command::cargo_bin("frame0")
        .unwrap()
        .args(["inspect", scene.as_str(), "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"ok\": true"));
}

#[test]
fn graph_outputs_nodes() {
    let scene = repo_path("examples/hello_shader/scene.yaml");
    Command::cargo_bin("frame0")
        .unwrap()
        .args(["graph", scene.as_str(), "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"topological_order\""));
}

#[test]
fn dry_run_outputs_snapshot() {
    let scene = repo_path("examples/hello_shader/scene.yaml");
    Command::cargo_bin("frame0")
        .unwrap()
        .args(["run", scene.as_str(), "--dry-run", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"runtime.dry_run.completed\""));
}

#[test]
fn ndjson_events_are_line_delimited() {
    let scene = repo_path("examples/hello_shader/scene.yaml");
    Command::cargo_bin("frame0")
        .unwrap()
        .args(["run", scene.as_str(), "--events", "ndjson", "--frames", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"event\":\"frame.rendered\""));
}

#[test]
fn devices_list_outputs_mock_device() {
    Command::cargo_bin("frame0")
        .unwrap()
        .args(["devices", "list", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("device.video_input.mock.0"));
}

#[test]
fn docs_index_outputs_public_contracts() {
    Command::cargo_bin("frame0")
        .unwrap()
        .args(["docs", "index", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"api_reference\""))
        .stdout(predicate::str::contains("\"compatibility\""))
        .stdout(predicate::str::contains("docs/api/reference.md"))
        .stdout(predicate::str::contains("docs/api/compatibility.md"))
        .stdout(predicate::str::contains("native_boundaries"))
        .stdout(predicate::str::contains("examples/hello_shader/scene.yaml"));
}

#[test]
fn docs_examples_include_scene_and_readme_paths() {
    Command::cargo_bin("frame0")
        .unwrap()
        .args(["docs", "examples", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"scene\""))
        .stdout(predicate::str::contains(
            "examples/audio_visual_sync/scene.yaml",
        ))
        .stdout(predicate::str::contains(
            "examples/audio_visual_sync/README.md",
        ));
}

#[test]
fn examples_launch_writes_preview_artifacts() {
    let temp = tempdir().unwrap();
    let out_dir = temp.path().join("launch");
    Command::cargo_bin("frame0")
        .unwrap()
        .args([
            "examples",
            "launch",
            "projection_mapping",
            "--frames",
            "4",
            "--out",
            out_dir.to_str().unwrap(),
            "--json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"status\": \"launched\""))
        .stdout(predicate::str::contains("preview.html"));

    assert!(out_dir.join("launch.json").is_file());
    assert!(out_dir.join("events.ndjson").is_file());
    assert!(out_dir.join("frames.json").is_file());
    assert!(out_dir.join("preview.html").is_file());

    let preview = fs::read_to_string(out_dir.join("preview.html")).unwrap();
    assert!(preview.contains("FRAME0 Example Launch"));
    assert!(preview.contains("projection_mapping"));
}

#[test]
fn examples_launch_all_writes_index_and_previews() {
    let temp = tempdir().unwrap();
    let out_dir = temp.path().join("launch_all");
    Command::cargo_bin("frame0")
        .unwrap()
        .args([
            "examples",
            "launch-all",
            "--frames",
            "2",
            "--out",
            out_dir.to_str().unwrap(),
            "--json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"count\""))
        .stdout(predicate::str::contains("index.html"));

    assert!(out_dir.join("index.html").is_file());
    assert!(out_dir.join("projection_mapping/preview.html").is_file());
    assert!(out_dir.join("coreml_style_transfer/launch.json").is_file());

    let index = fs::read_to_string(out_dir.join("index.html")).unwrap();
    assert!(index.contains("FRAME0 Example Launch Index"));
    assert!(index.contains("projection_mapping/preview.html"));
}

#[test]
fn new_addon_rust_copies_template_without_target() {
    let temp = tempdir().unwrap();
    let addon_dir = temp.path().join("my_addon");
    let addon_dir_str = addon_dir.to_str().unwrap();

    Command::cargo_bin("frame0")
        .unwrap()
        .args(["new", addon_dir_str, "--kind", "addon-rust"])
        .assert()
        .success()
        .stdout(predicate::str::contains("created addon-rust"));

    assert!(addon_dir.join("addon.yaml").is_file());
    assert!(addon_dir.join("Cargo.toml").is_file());
    assert!(addon_dir.join("src/lib.rs").is_file());
    assert!(addon_dir.join("examples/basic_scene.yaml").is_file());
    assert!(!addon_dir.join("target").exists());

    let scene = addon_dir.join("examples/basic_scene.yaml");
    Command::cargo_bin("frame0")
        .unwrap()
        .args(["inspect", scene.to_str().unwrap(), "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"ok\": true"));
}

#[test]
fn plugin_verify_outputs_ok() {
    let plugin = repo_path("plugins/mock/plugin.yaml");
    Command::cargo_bin("frame0")
        .unwrap()
        .args(["plugins", "verify", plugin.as_str(), "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"ok\": true"));
}

#[test]
fn all_examples_inspect_cleanly() {
    let examples_dir = PathBuf::from(repo_path("examples"));
    let mut scenes = Vec::new();
    for entry in fs::read_dir(&examples_dir).unwrap() {
        let entry = entry.unwrap();
        let scene = entry.path().join("scene.yaml");
        if scene.is_file() {
            scenes.push(scene);
        }
    }
    scenes.sort();
    assert!(scenes.len() >= 31, "expected rich example coverage");

    for scene in scenes {
        Command::cargo_bin("frame0")
            .unwrap()
            .args(["inspect", scene.to_str().unwrap(), "--json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("\"ok\": true"));
    }
}

#[test]
fn extension_stub_plugins_verify() {
    for plugin in [
        "plugins/camera_extension_stub/plugin.yaml",
        "plugins/audio_unit_stub/plugin.yaml",
        "plugins/syphon_stub/plugin.yaml",
    ] {
        let manifest = repo_path(plugin);
        Command::cargo_bin("frame0")
            .unwrap()
            .args(["plugins", "verify", manifest.as_str(), "--json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("\"ok\": true"));
    }
}
