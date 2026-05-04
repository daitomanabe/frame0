use anyhow::Result;
use clap::{Parser, Subcommand};
use frame0_plugin_host::{
    control_manifest, enumerate_manifest_devices, inspect_manifest, smoke_manifest,
    stream_test_manifest,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use std::process::{Command as ProcessCommand, Stdio};

#[derive(Debug, Parser)]
#[command(
    name = "frame0-plugin-host",
    version,
    about = "Out-of-process FRAME0 native plugin host"
)]
struct Cli {
    #[arg(
        long,
        global = true,
        help = "Emit machine-readable JSON where applicable"
    )]
    json: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Inspect {
        manifest: PathBuf,
    },
    EnumerateDevices {
        manifest: PathBuf,
    },
    Smoke {
        manifest: PathBuf,
    },
    StreamTest {
        manifest: PathBuf,
        #[arg(long)]
        device: Option<String>,
    },
    MlDescribe {
        manifest: PathBuf,
    },
    MlInfer {
        manifest: PathBuf,
        #[arg(long, default_value = "mock_classifier")]
        model: String,
        #[arg(long, default_value = "tensor.mock.input")]
        input_ref: String,
        #[arg(long, default_value_t = 0)]
        pts_ns: u64,
    },
    Supervise {
        manifest: PathBuf,
        #[arg(long, default_value_t = 1)]
        max_restarts: u32,
        #[arg(long)]
        crash_first: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SupervisorAttempt {
    attempt: u32,
    status: String,
    code: Option<i32>,
    restarted: bool,
    stdout: String,
    stderr: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SupervisorReport {
    manifest: PathBuf,
    max_restarts: u32,
    final_status: String,
    attempts: Vec<SupervisorAttempt>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Inspect { manifest } => {
            let report = inspect_manifest(manifest)?;
            print_value(&report, cli.json, "plugin inspected")
        }
        Command::EnumerateDevices { manifest } => {
            let devices = enumerate_manifest_devices(manifest)?;
            print_value(
                &devices,
                cli.json,
                format!("{} plugin devices", devices.len()),
            )
        }
        Command::Smoke { manifest } => {
            let report = smoke_manifest(manifest)?;
            print_value(&report, cli.json, "plugin smoke test passed")
        }
        Command::StreamTest { manifest, device } => {
            let report = stream_test_manifest(manifest, device)?;
            let human = format!(
                "stream test captured {} frame packet(s) and {} audio packet(s)",
                report.frames.len(),
                report.audio.len()
            );
            print_value(&report, cli.json, human)
        }
        Command::MlDescribe { manifest } => {
            let report = control_manifest(manifest, json!({ "method": "ml.describe" }))?;
            print_value(&report, cli.json, "ML models described")
        }
        Command::MlInfer {
            manifest,
            model,
            input_ref,
            pts_ns,
        } => {
            let report = control_manifest(
                manifest,
                json!({
                    "method": "ml.infer",
                    "params": {
                        "model_id": model,
                        "input_ref": input_ref,
                        "pts_ns": pts_ns
                    }
                }),
            )?;
            print_value(&report, cli.json, "ML inference completed")
        }
        Command::Supervise {
            manifest,
            max_restarts,
            crash_first,
        } => {
            let report = supervise_manifest(manifest, max_restarts, crash_first)?;
            let human = format!(
                "plugin supervisor finished with {} after {} attempt(s)",
                report.final_status,
                report.attempts.len()
            );
            print_value(&report, cli.json, human)
        }
    }
}

fn print_value<T: Serialize>(value: &T, json_output: bool, human: impl AsRef<str>) -> Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(value)?);
    } else {
        println!("{}", human.as_ref());
    }
    Ok(())
}

fn supervise_manifest(
    manifest: PathBuf,
    max_restarts: u32,
    crash_first: bool,
) -> Result<SupervisorReport> {
    let exe = std::env::current_exe()?;
    let mut attempts = Vec::new();
    let mut restarts_used = 0_u32;

    loop {
        let attempt = attempts.len() as u32 + 1;
        let mut command = ProcessCommand::new(&exe);
        command
            .arg("smoke")
            .arg(&manifest)
            .arg("--json")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if crash_first && attempt == 1 {
            command.env("FRAME0_MOCK_SDK_CRASH_ON_INIT", "1");
        }

        let output = command.output()?;
        let ok = output.status.success();
        let status = if ok {
            "ok".to_string()
        } else if output.status.code().is_none() {
            "crashed".to_string()
        } else {
            "failed".to_string()
        };
        let can_restart = !ok && restarts_used < max_restarts;
        attempts.push(SupervisorAttempt {
            attempt,
            status,
            code: output.status.code(),
            restarted: can_restart,
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });

        if ok {
            return Ok(SupervisorReport {
                manifest,
                max_restarts,
                final_status: "ok".to_string(),
                attempts,
            });
        }
        if can_restart {
            restarts_used += 1;
            continue;
        }
        return Ok(SupervisorReport {
            manifest,
            max_restarts,
            final_status: "failed".to_string(),
            attempts,
        });
    }
}
