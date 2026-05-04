use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;
use std::process::Command as StdCommand;
use std::sync::Once;

static BUILD_ONCE: Once = Once::new();
static BUILD_ML_ONCE: Once = Once::new();

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn manifest_path() -> String {
    repo_root()
        .join("plugins/mock/plugin.yaml")
        .display()
        .to_string()
}

fn ml_manifest_path() -> String {
    repo_root()
        .join("plugins/mock_ml/plugin.yaml")
        .display()
        .to_string()
}

fn ensure_mock_sdk_built() {
    BUILD_ONCE.call_once(|| {
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        let status = StdCommand::new(cargo)
            .args(["build", "-p", "frame0_mock_sdk"])
            .current_dir(repo_root())
            .status()
            .expect("failed to invoke cargo build for frame0_mock_sdk");
        assert!(status.success(), "frame0_mock_sdk build failed");
    });
    assert!(expected_mock_library_path().is_file());
}

fn ensure_mock_ml_built() {
    BUILD_ML_ONCE.call_once(|| {
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        let status = StdCommand::new(cargo)
            .args(["build", "-p", "frame0_mock_ml"])
            .current_dir(repo_root())
            .status()
            .expect("failed to invoke cargo build for frame0_mock_ml");
        assert!(status.success(), "frame0_mock_ml build failed");
    });
    assert!(expected_mock_ml_library_path().is_file());
}

fn expected_mock_library_path() -> PathBuf {
    let filename = if cfg!(target_os = "macos") {
        "libframe0_mock_sdk.dylib"
    } else if cfg!(target_os = "windows") {
        "frame0_mock_sdk.dll"
    } else {
        "libframe0_mock_sdk.so"
    };
    repo_root().join("target/debug").join(filename)
}

fn expected_mock_ml_library_path() -> PathBuf {
    let filename = if cfg!(target_os = "macos") {
        "libframe0_mock_ml.dylib"
    } else if cfg!(target_os = "windows") {
        "frame0_mock_ml.dll"
    } else {
        "libframe0_mock_ml.so"
    };
    repo_root().join("target/debug").join(filename)
}

#[test]
fn host_inspects_mock_descriptor() {
    ensure_mock_sdk_built();
    Command::cargo_bin("frame0-plugin-host")
        .unwrap()
        .args(["inspect", manifest_path().as_str(), "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("io.frame0.mock.sdk"));
}

#[test]
fn host_enumerates_mock_devices() {
    ensure_mock_sdk_built();
    Command::cargo_bin("frame0-plugin-host")
        .unwrap()
        .args(["enumerate-devices", manifest_path().as_str(), "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("device.video_input.mock.native.0"));
}

#[test]
fn host_smoke_initializes_enumerates_and_shutdowns() {
    ensure_mock_sdk_built();
    Command::cargo_bin("frame0-plugin-host")
        .unwrap()
        .args(["smoke", manifest_path().as_str(), "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"initialized\": true"))
        .stdout(predicate::str::contains("\"shutdown\": true"));
}

#[test]
fn supervisor_restarts_after_mock_plugin_crash() {
    ensure_mock_sdk_built();
    Command::cargo_bin("frame0-plugin-host")
        .unwrap()
        .args([
            "supervise",
            manifest_path().as_str(),
            "--max-restarts",
            "1",
            "--crash-first",
            "--json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"final_status\": \"ok\""))
        .stdout(predicate::str::contains("\"status\": \"crashed\""))
        .stdout(predicate::str::contains("\"restarted\": true"));
}

#[test]
fn stream_test_captures_mock_packets() {
    ensure_mock_sdk_built();
    Command::cargo_bin("frame0-plugin-host")
        .unwrap()
        .args(["stream-test", manifest_path().as_str(), "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\"stream_id\": \"stream.mock.video.0\"",
        ))
        .stdout(predicate::str::contains(
            "\"stream_id\": \"stream.mock.audio.0\"",
        ));
}

#[test]
fn ml_describe_reports_native_models() {
    ensure_mock_ml_built();
    Command::cargo_bin("frame0-plugin-host")
        .unwrap()
        .args(["ml-describe", ml_manifest_path().as_str(), "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("mock_classifier"))
        .stdout(predicate::str::contains("CoreML"));
}

#[test]
fn ml_infer_returns_inference_packet() {
    ensure_mock_ml_built();
    Command::cargo_bin("frame0-plugin-host")
        .unwrap()
        .args([
            "ml-infer",
            ml_manifest_path().as_str(),
            "--model",
            "mock_classifier",
            "--json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"type\": \"ml.inference\""))
        .stdout(predicate::str::contains("frame0.native.ml.mock"));
}
