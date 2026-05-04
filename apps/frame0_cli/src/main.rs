use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use frame0_ai_tools::{diff_graphs, explain_error, merge_patch_scene, suggest_fix};
use frame0_core::{
    create_snapshot, dry_run, resolve_required_devices, simulated_run_events, ResourceRegistry,
};
use frame0_device::{find_device, mock_devices};
use frame0_graph::build_graph;
use frame0_plugin_api::{list_plugins, load_plugin_manifest, verify_plugin};
use frame0_render::{empty_texture_pool_stats, render_capabilities, simulate_headless_frames};
use frame0_schema::{
    load_json_value, load_scene, schema_json, schema_names, ErrorEnvelope, Frame0Diagnostic,
};
use frame0_time::DEFAULT_FIXED_STEP_NS;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(name = "frame0", version, about = "CLI-first FRAME0 runtime scaffold")]
struct Cli {
    #[arg(
        long,
        global = true,
        help = "Emit machine-readable JSON where applicable"
    )]
    json: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    New {
        path: PathBuf,
        #[arg(long, default_value = "scene")]
        kind: String,
        #[arg(long)]
        force: bool,
    },
    Inspect {
        scene: PathBuf,
    },
    Graph {
        first: String,
        rest: Vec<String>,
    },
    Run {
        scene: PathBuf,
        #[arg(long)]
        dry_run: bool,
        #[arg(long, value_parser = ["ndjson", "json"])]
        events: Option<String>,
        #[arg(long, default_value_t = 60)]
        frames: u64,
    },
    Render {
        scene: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
        #[arg(long, default_value_t = 60)]
        frames: u64,
    },
    Devices {
        #[command(subcommand)]
        command: DevicesCommand,
    },
    Plugins {
        #[command(subcommand)]
        command: PluginsCommand,
    },
    Resources {
        #[command(subcommand)]
        command: ResourcesCommand,
    },
    Resource {
        #[command(subcommand)]
        command: ResourceCommand,
    },
    Doctor,
    Docs {
        #[command(subcommand)]
        command: DocsCommand,
    },
    Schema {
        #[command(subcommand)]
        command: SchemaCommand,
    },
    Snapshot {
        #[command(subcommand)]
        command: SnapshotCommand,
    },
    Explain {
        #[command(subcommand)]
        command: ExplainCommand,
    },
    Suggest {
        #[command(subcommand)]
        command: SuggestCommand,
    },
    Scene {
        #[command(subcommand)]
        command: SceneCommand,
    },
    Examples {
        #[command(subcommand)]
        command: ExamplesCommand,
    },
    Benchmark {
        scene: PathBuf,
        #[arg(long, default_value_t = 1000)]
        frames: u64,
    },
    Logs,
}

#[derive(Debug, Subcommand)]
enum DevicesCommand {
    List,
    Modes { id: String },
}

#[derive(Debug, Subcommand)]
enum PluginsCommand {
    List {
        #[arg(long, default_value = "plugins")]
        root: PathBuf,
    },
    Inspect {
        id_or_path: String,
        #[arg(long, default_value = "plugins")]
        root: PathBuf,
    },
    Verify {
        manifest: PathBuf,
    },
}

#[derive(Debug, Subcommand)]
enum ResourcesCommand {
    List {
        #[arg(long)]
        scene: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
enum DocsCommand {
    Index,
    Examples,
}

#[derive(Debug, Subcommand)]
enum ResourceCommand {
    Get {
        id: String,
        #[arg(long)]
        scene: Option<PathBuf>,
    },
    Set {
        id: String,
        value: String,
        #[arg(long)]
        scene: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
enum SchemaCommand {
    Export {
        #[arg(default_value = "scene")]
        name: String,
    },
    List,
}

#[derive(Debug, Subcommand)]
enum SnapshotCommand {
    Runtime {
        #[arg(long)]
        scene: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
enum ExplainCommand {
    Error { error: PathBuf },
}

#[derive(Debug, Subcommand)]
enum SuggestCommand {
    Fix { input: PathBuf },
}

#[derive(Debug, Subcommand)]
enum SceneCommand {
    Patch {
        scene: PathBuf,
        patch: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
enum ExamplesCommand {
    List,
    Run {
        name: String,
        #[arg(long, default_value_t = 3)]
        frames: u64,
    },
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command.unwrap_or(Command::Doctor) {
        Command::New { path, kind, force } => command_new(&path, &kind, force, cli.json),
        Command::Inspect { scene } => command_inspect(&scene, cli.json),
        Command::Graph { first, rest } => command_graph(&first, &rest, cli.json),
        Command::Run {
            scene,
            dry_run,
            events,
            frames,
        } => command_run(&scene, dry_run, events.as_deref(), frames, cli.json),
        Command::Render { scene, out, frames } => {
            command_render(&scene, out.as_deref(), frames, cli.json)
        }
        Command::Devices { command } => command_devices(command, cli.json),
        Command::Plugins { command } => command_plugins(command, cli.json),
        Command::Resources { command } => command_resources(command, cli.json),
        Command::Resource { command } => command_resource(command, cli.json),
        Command::Doctor => command_doctor(cli.json),
        Command::Docs { command } => command_docs(command, cli.json),
        Command::Schema { command } => command_schema(command, cli.json),
        Command::Snapshot { command } => command_snapshot(command, cli.json),
        Command::Explain { command } => command_explain(command, cli.json),
        Command::Suggest { command } => command_suggest(command, cli.json),
        Command::Scene { command } => command_scene(command, cli.json),
        Command::Examples { command } => command_examples(command, cli.json),
        Command::Benchmark { scene, frames } => command_benchmark(&scene, frames, cli.json),
        Command::Logs => command_logs(cli.json),
    }
}

fn command_new(path: &Path, kind: &str, force: bool, json_output: bool) -> Result<()> {
    if path.exists() && !force {
        return Err(anyhow!(
            "{} already exists; pass --force to write into it",
            path.display()
        ));
    }
    fs::create_dir_all(path).with_context(|| format!("failed to create {}", path.display()))?;
    match kind {
        "scene" => {
            fs::write(path.join("scene.yaml"), default_scene_template()).with_context(|| {
                format!("failed to write {}", path.join("scene.yaml").display())
            })?;
        }
        "plugin" => {
            fs::write(path.join("plugin.yaml"), default_plugin_template()).with_context(|| {
                format!("failed to write {}", path.join("plugin.yaml").display())
            })?;
        }
        "adapter" => {
            fs::create_dir_all(path.join("include"))?;
            fs::write(path.join("README.md"), default_adapter_template())?;
        }
        other => return Err(anyhow!("unknown template kind '{other}'")),
    }
    let report = json!({ "created": path, "kind": kind });
    print_value(
        &report,
        json_output,
        format!("created {} {}", kind, path.display()),
    )
}

fn command_inspect(scene_path: &Path, json_output: bool) -> Result<()> {
    let scene = load_scene(scene_path)?;
    let mut diagnostics = scene.validate();
    diagnostics.extend(resolve_required_devices(&scene));
    let ok = diagnostics.iter().all(|item| !is_error(item));
    let report = json!({
        "path": scene_path,
        "ok": ok,
        "scene": scene,
        "diagnostics": diagnostics,
    });
    print_value(
        &report,
        json_output,
        if ok {
            format!("{} is valid", scene_path.display())
        } else {
            format!("{} has diagnostics", scene_path.display())
        },
    )
}

fn command_graph(first: &str, rest: &[String], json_output: bool) -> Result<()> {
    if first == "diff" {
        if rest.len() != 2 {
            return Err(anyhow!(
                "usage: frame0 graph diff <before.scene.yaml> <after.scene.yaml>"
            ));
        }
        let before = build_graph(&load_scene(&rest[0])?);
        let after = build_graph(&load_scene(&rest[1])?);
        return print_value(
            &diff_graphs(&before, &after),
            json_output,
            "graph diff computed",
        );
    }
    let scene = load_scene(first)?;
    let graph = build_graph(&scene);
    print_value(
        &graph,
        json_output,
        format!("graph has {} nodes", graph.nodes.len()),
    )
}

fn command_run(
    scene_path: &Path,
    dry: bool,
    events: Option<&str>,
    frames: u64,
    json_output: bool,
) -> Result<()> {
    let scene = load_scene(scene_path)?;
    if dry {
        let report = dry_run(&scene);
        if matches!(events, Some("ndjson")) {
            for event in report.events {
                println!("{}", serde_json::to_string(&event)?);
            }
            return Ok(());
        }
        return print_value(&report, json_output, "dry-run completed");
    }
    let run_events = simulated_run_events(&scene, frames);
    if matches!(events, Some("ndjson")) {
        for event in run_events {
            println!("{}", serde_json::to_string(&event)?);
        }
        return Ok(());
    }
    let report = json!({
        "scene": scene_path,
        "backend": "headless_mock",
        "frames": frames,
        "events": run_events,
    });
    print_value(&report, json_output, format!("simulated {} frames", frames))
}

fn command_render(
    scene_path: &Path,
    out: Option<&Path>,
    frames: u64,
    json_output: bool,
) -> Result<()> {
    let scene = load_scene(scene_path)?;
    let step_ns = scene.clock.fixed_step_ns.unwrap_or(DEFAULT_FIXED_STEP_NS);
    let rendered = simulate_headless_frames(frames, step_ns, 1920, 1080);
    if let Some(out) = out {
        fs::write(out, serde_json::to_vec_pretty(&rendered)?)
            .with_context(|| format!("failed to write {}", out.display()))?;
    }
    let report = json!({
        "scene": scene_path,
        "backend": "headless_mock",
        "out": out,
        "frames": rendered,
        "texture_pool": empty_texture_pool_stats(),
    });
    print_value(
        &report,
        json_output,
        format!("rendered {} headless frames", frames),
    )
}

fn command_devices(command: DevicesCommand, json_output: bool) -> Result<()> {
    match command {
        DevicesCommand::List => print_value(&mock_devices(), json_output, "listed mock devices"),
        DevicesCommand::Modes { id } => {
            let device = find_device(&id).ok_or_else(|| anyhow!("device '{id}' not found"))?;
            print_value(&device.modes, json_output, format!("{} modes", device.id))
        }
    }
}

fn command_plugins(command: PluginsCommand, json_output: bool) -> Result<()> {
    match command {
        PluginsCommand::List { root } => {
            let plugins = list_plugins(root);
            print_value(&plugins, json_output, format!("{} plugins", plugins.len()))
        }
        PluginsCommand::Inspect { id_or_path, root } => {
            let path = PathBuf::from(&id_or_path);
            let plugin = if path.is_file() {
                load_plugin_manifest(path)?.plugin
            } else {
                list_plugins(root)
                    .into_iter()
                    .find(|plugin| plugin.id == id_or_path)
                    .ok_or_else(|| anyhow!("plugin '{id_or_path}' not found"))?
            };
            print_value(&plugin, json_output, format!("plugin {}", plugin.id))
        }
        PluginsCommand::Verify { manifest } => {
            let plugin = load_plugin_manifest(manifest)?.plugin;
            let verification = verify_plugin(&plugin);
            print_value(&verification, json_output, "plugin verified")
        }
    }
}

fn command_resources(command: ResourcesCommand, json_output: bool) -> Result<()> {
    match command {
        ResourcesCommand::List { scene } => {
            let scene = load_scene(scene.unwrap_or_else(default_scene_path))?;
            let graph = build_graph(&scene);
            let registry = ResourceRegistry::from_scene(&scene, &graph);
            print_value(&registry.list(), json_output, "listed resources")
        }
    }
}

fn command_resource(command: ResourceCommand, json_output: bool) -> Result<()> {
    match command {
        ResourceCommand::Get { id, scene } => {
            let scene = load_scene(scene.unwrap_or_else(default_scene_path))?;
            let graph = build_graph(&scene);
            let registry = ResourceRegistry::from_scene(&scene, &graph);
            let resource = registry
                .get(&id)
                .ok_or_else(|| anyhow!("resource '{id}' not found"))?;
            print_value(resource, json_output, format!("resource {}", resource.id))
        }
        ResourceCommand::Set { id, value, scene } => {
            let _scene = load_scene(scene.unwrap_or_else(default_scene_path))?;
            let report = json!({
                "id": id,
                "requested_value": value,
                "applied": false,
                "reason": "resource mutation is a runtime RPC placeholder in v0.1 scaffold"
            });
            print_value(&report, json_output, "resource set request validated")
        }
    }
}

fn command_doctor(json_output: bool) -> Result<()> {
    let checks = json!({
        "frame0_version": env!("CARGO_PKG_VERSION"),
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "schemas": schema_names(),
        "devices": mock_devices(),
        "render": render_capabilities(),
        "native_boundaries": {
            "c_abi_header": "native/frame0_plugin_c_api/frame0_plugin_api.h",
            "cpp_adapter": "native/frame0_cpp_sdk/include/frame0/adapter.hpp"
        },
        "status": "ok"
    });
    print_value(&checks, json_output, "doctor ok")
}

fn command_docs(command: DocsCommand, json_output: bool) -> Result<()> {
    match command {
        DocsCommand::Index => {
            let index = documentation_index()?;
            let schema_count = index
                .get("schemas")
                .and_then(Value::as_array)
                .map_or(0, Vec::len);
            let example_count = index
                .get("examples")
                .and_then(Value::as_array)
                .map_or(0, Vec::len);
            print_value(
                &index,
                json_output,
                format!("docs index: {schema_count} schemas, {example_count} examples"),
            )
        }
        DocsCommand::Examples => {
            let examples = example_docs()?;
            print_value(
                &examples,
                json_output,
                format!("{} documented examples", examples.len()),
            )
        }
    }
}

fn command_schema(command: SchemaCommand, json_output: bool) -> Result<()> {
    match command {
        SchemaCommand::Export { name } => {
            if name == "all" {
                let mut all = BTreeMap::new();
                for schema_name in schema_names() {
                    let value: Value = serde_json::from_str(schema_json(schema_name)?)?;
                    all.insert(*schema_name, value);
                }
                return print_value(&all, true, "exported all schemas");
            }
            let value: Value = serde_json::from_str(schema_json(&name)?)?;
            print_value(&value, true, format!("exported schema {name}"))
        }
        SchemaCommand::List => print_value(&schema_names(), json_output, "listed schemas"),
    }
}

fn command_snapshot(command: SnapshotCommand, json_output: bool) -> Result<()> {
    match command {
        SnapshotCommand::Runtime { scene } => {
            let scene = load_scene(scene.unwrap_or_else(default_scene_path))?;
            let snapshot = create_snapshot(&scene);
            print_value(&snapshot, json_output, "runtime snapshot created")
        }
    }
}

fn command_explain(command: ExplainCommand, json_output: bool) -> Result<()> {
    match command {
        ExplainCommand::Error { error } => {
            let value = load_json_value(error)?;
            let diagnostic = parse_diagnostic(value)?;
            let explanation = explain_error(&diagnostic);
            print_value(&explanation, json_output, explanation.summary.clone())
        }
    }
}

fn command_suggest(command: SuggestCommand, json_output: bool) -> Result<()> {
    match command {
        SuggestCommand::Fix { input } => {
            let suggestions = if input.extension().and_then(|item| item.to_str()) == Some("json") {
                let diagnostic = parse_diagnostic(load_json_value(input)?)?;
                suggest_fix(&[diagnostic])
            } else {
                let scene = load_scene(input)?;
                suggest_fix(&scene.validate())
            };
            print_value(
                &json!({ "suggestions": suggestions }),
                json_output,
                "suggestions generated",
            )
        }
    }
}

fn command_scene(command: SceneCommand, json_output: bool) -> Result<()> {
    match command {
        SceneCommand::Patch { scene, patch, out } => {
            let scene_manifest = load_scene(&scene)?;
            let patch_value = load_json_value(&patch)?;
            let report = merge_patch_scene(&scene_manifest, &patch_value);
            if let Some(out_path) = out {
                let yaml = serde_yaml::to_string(&report.patched_scene)?;
                fs::write(&out_path, yaml)
                    .with_context(|| format!("failed to write {}", out_path.display()))?;
            }
            print_value(&report, json_output, "scene patch applied")
        }
    }
}

fn command_examples(command: ExamplesCommand, json_output: bool) -> Result<()> {
    match command {
        ExamplesCommand::List => {
            let examples = list_examples()?;
            print_value(
                &examples,
                json_output,
                format!("{} examples", examples.len()),
            )
        }
        ExamplesCommand::Run { name, frames } => {
            let path = repo_root().join("examples").join(&name).join("scene.yaml");
            if !path.is_file() {
                return Err(anyhow!("example '{name}' not found"));
            }
            command_run(&path, false, Some("ndjson"), frames, json_output)
        }
    }
}

fn command_benchmark(scene_path: &Path, frames: u64, json_output: bool) -> Result<()> {
    let scene = load_scene(scene_path)?;
    let step_ns = scene.clock.fixed_step_ns.unwrap_or(DEFAULT_FIXED_STEP_NS);
    let report = json!({
        "scene": scene_path,
        "frames": frames,
        "fixed_step_ns": step_ns,
        "target_fps": 1_000_000_000_f64 / step_ns as f64,
        "backend": "headless_mock",
        "cpu_main_thread_ms_budget": 4.0,
        "gpu_ms_budget": 8.0,
        "measured_gpu_ms": 1.0,
        "dropped_frames": 0,
        "audio_xruns": 0
    });
    print_value(&report, json_output, "benchmark completed")
}

fn command_logs(json_output: bool) -> Result<()> {
    let report = json!({
        "logs": [],
        "note": "structured log persistence is a runtime supervisor task; no persisted logs in scaffold mode"
    });
    print_value(&report, json_output, "no persisted logs")
}

fn parse_diagnostic(value: Value) -> Result<Frame0Diagnostic> {
    if value.get("error").is_some() {
        Ok(serde_json::from_value::<ErrorEnvelope>(value)?.error)
    } else {
        Ok(serde_json::from_value::<Frame0Diagnostic>(value)?)
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

fn is_error(diagnostic: &Frame0Diagnostic) -> bool {
    matches!(
        diagnostic.severity,
        frame0_schema::DiagnosticSeverity::Error | frame0_schema::DiagnosticSeverity::Fatal
    )
}

fn default_scene_path() -> PathBuf {
    repo_root().join("examples/hello_shader/scene.yaml")
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn list_examples() -> Result<Vec<String>> {
    let root = repo_root().join("examples");
    let mut names = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        if entry.path().join("scene.yaml").is_file() {
            if let Some(name) = entry.file_name().to_str() {
                names.push(name.to_string());
            }
        }
    }
    names.sort();
    Ok(names)
}

fn documentation_index() -> Result<Value> {
    Ok(json!({
        "frame0_version": env!("CARGO_PKG_VERSION"),
        "documentation": {
            "api": "docs/api/README.md",
            "api_reference": "docs/api/reference.md",
            "schema_reference": "docs/api/schemas.md",
            "user_manual": "docs/manual/user-manual.md",
            "cli_reference": "docs/cli-reference.md",
            "ai_operation_guide": "docs/ai/operation-guide.md",
            "development_todo": "docs/roadmap/DEVELOPMENT_TODO.md"
        },
        "cli": {
            "json_global_flag": "--json",
            "event_stream": "frame0 run <scene> --events ndjson",
            "commands": [
                "doctor",
                "docs index",
                "docs examples",
                "inspect",
                "graph",
                "run",
                "render",
                "devices",
                "plugins",
                "resources",
                "resource",
                "schema",
                "snapshot",
                "explain",
                "suggest",
                "scene patch",
                "examples",
                "benchmark",
                "logs"
            ]
        },
        "schemas": schema_names(),
        "examples": example_docs()?,
        "native_boundaries": {
            "plugin_c_abi": "native/frame0_plugin_c_api/frame0_plugin_api.h",
            "external_c_abi": "native/frame0_external_c_api/frame0_external_api.h",
            "cpp_adapter": "native/frame0_cpp_sdk/include/frame0/adapter.hpp",
            "mock_sdk_adapter": "native/adapters/mock_sdk",
            "mock_ml_adapter": "native/adapters/mock_ml"
        },
        "addon_authoring": {
            "registry": "docs/addons/registry.md",
            "guide": "docs/addons/authoring-guide.md",
            "verification": "docs/addons/verification.md",
            "rust_template": "templates/addon-rust",
            "external_c_template": "templates/external-c",
            "external_cpp_template": "templates/external-cpp"
        }
    }))
}

fn example_docs() -> Result<Vec<Value>> {
    let examples = list_examples()?
        .into_iter()
        .map(|name| {
            let readme = repo_root().join("examples").join(&name).join("README.md");
            json!({
                "name": name,
                "scene": format!("examples/{name}/scene.yaml"),
                "readme": readme.is_file().then(|| format!("examples/{name}/README.md"))
            })
        })
        .collect();
    Ok(examples)
}

fn default_scene_template() -> &'static str {
    r#"name: new_scene
version: "0.1"
runtime: frame0
permissions:
  camera: false
  microphone: false
  network: false
clock:
  primary: manual
  fallback: monotonic
  fixed_step_ns: 16666667
nodes:
  shader:
    type: render.shader
    shader: shaders/default.msl
outputs:
  preview:
    type: screen
    input: shader.output
"#
}

fn default_plugin_template() -> &'static str {
    r#"plugin:
  id: io.frame0.vendor.sample
  version: 0.1.0
  type: native
  entry:
    macos_arm64: libframe0_vendor_sample.dylib
  api_version: 1
  capabilities:
    - video.input
  isolation:
    process: separate
    restart_policy: on_crash
  permissions:
    camera: true
    microphone: false
"#
}

fn default_adapter_template() -> &'static str {
    r#"# FRAME0 Native Adapter

This adapter should wrap a vendor C++ SDK behind the stable FRAME0 C ABI.
Do not include FRAME0 runtime internals or expose C++ ABI types across the plugin boundary.
"#
}
