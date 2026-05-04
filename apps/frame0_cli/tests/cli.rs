use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;

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
fn extension_examples_inspect_cleanly() {
    for example in [
        "examples/camera_extension_output/scene.yaml",
        "examples/auv3_audio_unit/scene.yaml",
        "examples/extension_multi_output/scene.yaml",
        "examples/creative_primitives/scene.yaml",
        "examples/cinder_geometry/scene.yaml",
        "examples/operator_network/scene.yaml",
    ] {
        let scene = repo_path(example);
        Command::cargo_bin("frame0")
            .unwrap()
            .args(["inspect", scene.as_str(), "--json"])
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
