use frame0_device::{devices_with_capability, mock_devices};
use frame0_graph::{build_graph, GraphSnapshot};
use frame0_schema::{EventPacket, Frame0Diagnostic, PermissionSet, SceneManifest};
use frame0_time::{timestamp_now_utc, Clock, ClockSnapshot, FixedStepClock, DEFAULT_FIXED_STEP_NS};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceState {
    Created,
    Resolved,
    Opening,
    Active,
    Paused,
    Failed,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub capabilities: Vec<String>,
    pub status: ResourceState,
    pub process: String,
    pub permissions: Vec<String>,
    pub owner: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub vendor_properties: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default)]
pub struct ResourceRegistry {
    resources: BTreeMap<String, Resource>,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, resource: Resource) {
        self.resources.insert(resource.id.clone(), resource);
    }

    pub fn get(&self, id: &str) -> Option<&Resource> {
        self.resources.get(id)
    }

    pub fn update_state(&mut self, id: &str, status: ResourceState) -> Option<Resource> {
        self.resources.get_mut(id).map(|resource| {
            resource.status = status;
            resource.clone()
        })
    }

    pub fn list(&self) -> Vec<Resource> {
        self.resources.values().cloned().collect()
    }

    pub fn from_scene(scene: &SceneManifest, graph: &GraphSnapshot) -> Self {
        let mut registry = Self::new();

        for device in mock_devices() {
            registry.register(Resource {
                id: device.id,
                resource_type: device.device_type,
                capabilities: device.capabilities,
                status: ResourceState::Resolved,
                process: device.process,
                permissions: device.permissions,
                owner: "device_registry".to_string(),
                dependencies: Vec::new(),
                vendor_properties: BTreeMap::from([("vendor".to_string(), json!(device.vendor))]),
            });
        }

        for node in &graph.nodes {
            let component = scene
                .inputs
                .get(&node.id)
                .or_else(|| scene.nodes.get(&node.id))
                .or_else(|| scene.outputs.get(&node.id));
            let permissions = scene.permissions.enabled_names();
            let capabilities = component
                .and_then(|item| item.selector.as_ref())
                .and_then(|selector| selector.capability.clone())
                .map(|capability| vec![capability])
                .unwrap_or_default();
            registry.register(Resource {
                id: format!("{}.{}", node.kind_as_prefix(), node.id),
                resource_type: node.node_type.clone(),
                capabilities,
                status: ResourceState::Created,
                process: "frame0_runtime".to_string(),
                permissions,
                owner: scene.name.clone(),
                dependencies: node.dependencies.clone(),
                vendor_properties: BTreeMap::new(),
            });
        }

        registry
    }
}

trait GraphNodeExt {
    fn kind_as_prefix(&self) -> &'static str;
}

impl GraphNodeExt for frame0_graph::GraphNode {
    fn kind_as_prefix(&self) -> &'static str {
        match &self.kind {
            frame0_graph::GraphNodeKind::Input => "input",
            frame0_graph::GraphNodeKind::Node => "node",
            frame0_graph::GraphNodeKind::Output => "output",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeInfo {
    pub id: String,
    pub version: String,
    pub state: String,
    pub process: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneSummary {
    pub name: String,
    pub version: String,
    pub runtime: String,
    pub permissions: PermissionSet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeStats {
    pub frame_count: u64,
    pub dropped_frames: u64,
    pub late_frames: u64,
    pub plugin_crash_count: u64,
    pub audio_xruns: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeSnapshot {
    pub runtime: RuntimeInfo,
    pub scene: SceneSummary,
    pub resources: Vec<Resource>,
    pub graph: GraphSnapshot,
    pub clock: ClockSnapshot,
    pub stats: RuntimeStats,
    pub diagnostics: Vec<Frame0Diagnostic>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DryRunReport {
    pub ok: bool,
    pub snapshot: RuntimeSnapshot,
    pub events: Vec<EventPacket>,
}

pub fn create_snapshot(scene: &SceneManifest) -> RuntimeSnapshot {
    let graph = build_graph(scene);
    let registry = ResourceRegistry::from_scene(scene, &graph);
    let clock = FixedStepClock::new(
        scene.clock.primary.clone(),
        scene.clock.fixed_step_ns.unwrap_or(DEFAULT_FIXED_STEP_NS),
    )
    .snapshot();
    RuntimeSnapshot {
        runtime: RuntimeInfo {
            id: format!("runtime.{}", scene.name),
            version: env!("CARGO_PKG_VERSION").to_string(),
            state: "loaded".to_string(),
            process: "frame0_runtime".to_string(),
        },
        scene: SceneSummary {
            name: scene.name.clone(),
            version: scene.version.clone(),
            runtime: scene.runtime.clone(),
            permissions: scene.permissions.clone(),
        },
        resources: registry.list(),
        diagnostics: graph.diagnostics.clone(),
        graph,
        clock,
        stats: RuntimeStats {
            frame_count: 0,
            dropped_frames: 0,
            late_frames: 0,
            plugin_crash_count: 0,
            audio_xruns: 0,
        },
    }
}

pub fn dry_run(scene: &SceneManifest) -> DryRunReport {
    let snapshot = create_snapshot(scene);
    let ok = !snapshot.diagnostics.iter().any(|item| {
        matches!(
            item.severity,
            frame0_schema::DiagnosticSeverity::Error | frame0_schema::DiagnosticSeverity::Fatal
        )
    });
    let mut events = vec![event("runtime.loaded", Some(&snapshot.runtime.id), None)];
    for resource in &snapshot.resources {
        if resource.owner == snapshot.scene.name {
            events.push(event("resource.created", Some(&resource.id), None));
        }
    }
    for diagnostic in &snapshot.diagnostics {
        events.push(event(
            "diagnostic",
            diagnostic.resource.as_deref(),
            Some(BTreeMap::from([
                ("code".to_string(), json!(diagnostic.code)),
                ("severity".to_string(), json!(diagnostic.severity)),
                ("message".to_string(), json!(diagnostic.message)),
            ])),
        ));
    }
    events.push(event(
        "runtime.dry_run.completed",
        Some(&snapshot.runtime.id),
        None,
    ));
    DryRunReport {
        ok,
        snapshot,
        events,
    }
}

pub fn simulated_run_events(scene: &SceneManifest, frames: u64) -> Vec<EventPacket> {
    let snapshot = create_snapshot(scene);
    let step_ns = scene.clock.fixed_step_ns.unwrap_or(DEFAULT_FIXED_STEP_NS);
    let mut events = vec![event("runtime.started", Some(&snapshot.runtime.id), None)];
    events.push(event(
        "graph.resolved",
        Some(&snapshot.runtime.id),
        Some(BTreeMap::from([(
            "nodes".to_string(),
            json!(snapshot.graph.nodes.len()),
        )])),
    ));
    for frame in 0..frames {
        let mut data = BTreeMap::new();
        data.insert("pts_ns".to_string(), json!(frame * step_ns));
        data.insert("gpu_ms".to_string(), json!(1.0_f64));
        data.insert("backend".to_string(), json!("headless_mock"));
        let mut packet = event("frame.rendered", None, Some(data));
        packet.frame = Some(frame);
        events.push(packet);
    }
    events.push(event("runtime.stopped", Some(&snapshot.runtime.id), None));
    events
}

pub fn resolve_required_devices(scene: &SceneManifest) -> Vec<Frame0Diagnostic> {
    let mut diagnostics = Vec::new();
    for (id, input) in &scene.inputs {
        if let Some(selector) = &input.selector {
            if let Some(capability) = &selector.capability {
                if devices_with_capability(capability).is_empty() {
                    diagnostics.push(
                        Frame0Diagnostic::error(
                            "DEVICE_CAPABILITY_NOT_FOUND",
                            format!("No device found with capability '{capability}'"),
                        )
                        .with_resource(id)
                        .with_suggestion("Run frame0 devices list --json"),
                    );
                }
            }
        }
    }
    diagnostics
}

fn event(
    name: impl Into<String>,
    id: Option<&str>,
    data: Option<BTreeMap<String, Value>>,
) -> EventPacket {
    EventPacket {
        event: name.into(),
        time: timestamp_now_utc(),
        id: id.map(str::to_string),
        frame: None,
        code: None,
        resource: id.map(str::to_string),
        data: data.unwrap_or_default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame0_schema::{ClockConfig, ComponentSpec};

    fn scene() -> SceneManifest {
        SceneManifest {
            name: "runtime_test".to_string(),
            version: "0.1".to_string(),
            runtime: "frame0".to_string(),
            permissions: PermissionSet::default(),
            clock: ClockConfig {
                primary: "manual".to_string(),
                fallback: "monotonic".to_string(),
                fixed_step_ns: Some(10),
            },
            inputs: BTreeMap::new(),
            nodes: BTreeMap::from([(
                "shader".to_string(),
                ComponentSpec {
                    component_type: "render.shader".to_string(),
                    selector: None,
                    input: None,
                    inputs: BTreeMap::new(),
                    mode: None,
                    pixel_format: None,
                    shader: None,
                    params: BTreeMap::new(),
                    extra: BTreeMap::new(),
                },
            )]),
            outputs: BTreeMap::from([(
                "screen".to_string(),
                ComponentSpec {
                    component_type: "screen".to_string(),
                    selector: None,
                    input: Some("shader.output".to_string()),
                    inputs: BTreeMap::new(),
                    mode: None,
                    pixel_format: None,
                    shader: None,
                    params: BTreeMap::new(),
                    extra: BTreeMap::new(),
                },
            )]),
        }
    }

    #[test]
    fn dry_run_builds_snapshot() {
        let report = dry_run(&scene());
        assert!(report.ok);
        assert_eq!(report.snapshot.scene.name, "runtime_test");
        assert!(report
            .events
            .iter()
            .any(|event| event.event == "runtime.dry_run.completed"));
    }
}
