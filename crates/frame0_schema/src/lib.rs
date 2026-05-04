use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub const FRAME0_SCHEMA_VERSION: &str = "0.1";

#[derive(Debug, thiserror::Error)]
pub enum SchemaError {
    #[error("failed to read {path}: {source}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse YAML {path}: {source}")]
    ParseYaml {
        path: String,
        #[source]
        source: serde_yaml::Error,
    },
    #[error("failed to parse JSON {path}: {source}")]
    ParseJson {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("unknown schema '{0}'")]
    UnknownSchema(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frame0Diagnostic {
    pub code: String,
    pub severity: DiagnosticSeverity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    pub message: String,
    #[serde(default)]
    pub suggestions: Vec<String>,
}

impl Frame0Diagnostic {
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            severity: DiagnosticSeverity::Error,
            resource: None,
            message: message.into(),
            suggestions: Vec::new(),
        }
    }

    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            severity: DiagnosticSeverity::Warning,
            resource: None,
            message: message.into(),
            suggestions: Vec::new(),
        }
    }

    pub fn with_resource(mut self, resource: impl Into<String>) -> Self {
        self.resource = Some(resource.into());
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    pub fn as_error_envelope(&self) -> ErrorEnvelope {
        ErrorEnvelope {
            error: self.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorEnvelope {
    pub error: Frame0Diagnostic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PermissionSet {
    pub camera: bool,
    pub microphone: bool,
    pub network: bool,
    pub file_read: bool,
    pub file_write: bool,
}

impl PermissionSet {
    pub fn enabled_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        if self.camera {
            names.push("camera".to_string());
        }
        if self.microphone {
            names.push("microphone".to_string());
        }
        if self.network {
            names.push("network".to_string());
        }
        if self.file_read {
            names.push("file_read".to_string());
        }
        if self.file_write {
            names.push("file_write".to_string());
        }
        names
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClockConfig {
    pub primary: String,
    pub fallback: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_step_ns: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Selector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentSpec {
    #[serde(rename = "type")]
    pub component_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<Selector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(default)]
    pub inputs: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pixel_format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shader: Option<String>,
    #[serde(default)]
    pub params: BTreeMap<String, Value>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl ComponentSpec {
    pub fn dependency_refs(&self) -> Vec<String> {
        let mut refs = Vec::new();
        if let Some(input) = &self.input {
            refs.push(input.clone());
        }
        refs.extend(self.inputs.values().cloned());
        refs
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneManifest {
    pub name: String,
    pub version: String,
    pub runtime: String,
    #[serde(default)]
    pub permissions: PermissionSet,
    pub clock: ClockConfig,
    #[serde(default)]
    pub inputs: BTreeMap<String, ComponentSpec>,
    #[serde(default)]
    pub nodes: BTreeMap<String, ComponentSpec>,
    #[serde(default)]
    pub outputs: BTreeMap<String, ComponentSpec>,
}

impl SceneManifest {
    pub fn validate(&self) -> Vec<Frame0Diagnostic> {
        let mut diagnostics = Vec::new();
        if self.name.trim().is_empty() {
            diagnostics.push(Frame0Diagnostic::error(
                "SCENE_NAME_EMPTY",
                "Scene name must not be empty",
            ));
        }
        if self.runtime != "frame0" {
            diagnostics.push(
                Frame0Diagnostic::error(
                    "SCENE_RUNTIME_UNSUPPORTED",
                    format!("Unsupported runtime '{}'", self.runtime),
                )
                .with_suggestion("Use runtime: frame0"),
            );
        }
        if self.clock.primary.trim().is_empty() {
            diagnostics.push(Frame0Diagnostic::error(
                "CLOCK_PRIMARY_EMPTY",
                "clock.primary must not be empty",
            ));
        }
        if self.clock.fallback.trim().is_empty() {
            diagnostics.push(Frame0Diagnostic::error(
                "CLOCK_FALLBACK_EMPTY",
                "clock.fallback must not be empty",
            ));
        }
        if self.nodes.is_empty() {
            diagnostics.push(Frame0Diagnostic::warning(
                "SCENE_HAS_NO_NODES",
                "Scene has no processing nodes",
            ));
        }
        if self.outputs.is_empty() {
            diagnostics.push(Frame0Diagnostic::error(
                "SCENE_HAS_NO_OUTPUTS",
                "Scene must define at least one output",
            ));
        }
        diagnostics.extend(validate_component_map("input", &self.inputs));
        diagnostics.extend(validate_component_map("node", &self.nodes));
        diagnostics.extend(validate_component_map("output", &self.outputs));
        diagnostics.extend(self.validate_references());
        diagnostics
    }

    fn validate_references(&self) -> Vec<Frame0Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut known: Vec<&str> = self.inputs.keys().map(String::as_str).collect();
        known.extend(self.nodes.keys().map(String::as_str));
        for (id, component) in self.nodes.iter().chain(self.outputs.iter()) {
            for dep in component.dependency_refs() {
                let root = dep.split('.').next().unwrap_or(dep.as_str());
                if !known.iter().any(|candidate| candidate == &root) {
                    diagnostics.push(
                        Frame0Diagnostic::error(
                            "GRAPH_REFERENCE_NOT_FOUND",
                            format!("Component '{id}' references unknown resource '{dep}'"),
                        )
                        .with_resource(id)
                        .with_suggestion("Run frame0 graph <scene> --json to inspect dependencies"),
                    );
                }
            }
        }
        diagnostics
    }
}

fn validate_component_map(
    kind: &str,
    components: &BTreeMap<String, ComponentSpec>,
) -> Vec<Frame0Diagnostic> {
    let mut diagnostics = Vec::new();
    for (id, component) in components {
        if id.trim().is_empty() {
            diagnostics.push(Frame0Diagnostic::error(
                "COMPONENT_ID_EMPTY",
                format!("{kind} id must not be empty"),
            ));
        }
        if component.component_type.trim().is_empty() {
            diagnostics.push(
                Frame0Diagnostic::error(
                    "COMPONENT_TYPE_EMPTY",
                    format!("{kind} '{id}' must declare a non-empty type"),
                )
                .with_resource(id),
            );
        }
    }
    diagnostics
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FramePacket {
    #[serde(rename = "type")]
    pub packet_type: String,
    pub stream_id: String,
    pub pts_ns: u64,
    pub duration_ns: u64,
    pub frame_index: u64,
    pub width: u32,
    pub height: u32,
    pub pixel_format: String,
    pub color_space: String,
    pub transfer_function: String,
    pub range: String,
    pub field_order: String,
    pub storage: StorageRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioPacket {
    #[serde(rename = "type")]
    pub packet_type: String,
    pub stream_id: String,
    pub pts_ns: u64,
    pub duration_ns: u64,
    pub sample_rate: u32,
    pub channels: u16,
    pub frames: u32,
    pub format: String,
    pub storage: StorageRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageRef {
    #[serde(rename = "type")]
    pub storage_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventPacket {
    pub event: String,
    pub time: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frame: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default)]
    pub data: BTreeMap<String, Value>,
}

pub fn load_scene(path: impl AsRef<Path>) -> Result<SceneManifest, SchemaError> {
    let path = path.as_ref();
    let text = fs::read_to_string(path).map_err(|source| SchemaError::Read {
        path: path.display().to_string(),
        source,
    })?;
    serde_yaml::from_str(&text).map_err(|source| SchemaError::ParseYaml {
        path: path.display().to_string(),
        source,
    })
}

pub fn load_json_value(path: impl AsRef<Path>) -> Result<Value, SchemaError> {
    let path = path.as_ref();
    let text = fs::read_to_string(path).map_err(|source| SchemaError::Read {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| SchemaError::ParseJson {
        path: path.display().to_string(),
        source,
    })
}

pub fn schema_names() -> &'static [&'static str] {
    &[
        "audio_packet",
        "automation",
        "capability",
        "device",
        "error",
        "event_packet",
        "extension",
        "frame_packet",
        "graph",
        "inference_packet",
        "input_event",
        "media_asset",
        "ml_model",
        "operator_network",
        "parameter",
        "permission",
        "plugin",
        "resource",
        "runtime_snapshot",
        "scene",
        "timeline",
        "visual_node",
    ]
}

pub fn schema_json(name: &str) -> Result<&'static str, SchemaError> {
    match name {
        "audio_packet" | "audio-packet" => {
            Ok(include_str!("../../../schemas/audio_packet.schema.json"))
        }
        "automation" => Ok(include_str!("../../../schemas/automation.schema.json")),
        "capability" => Ok(include_str!("../../../schemas/capability.schema.json")),
        "device" => Ok(include_str!("../../../schemas/device.schema.json")),
        "error" => Ok(include_str!("../../../schemas/error.schema.json")),
        "event_packet" | "event-packet" => {
            Ok(include_str!("../../../schemas/event_packet.schema.json"))
        }
        "extension" => Ok(include_str!("../../../schemas/extension.schema.json")),
        "frame_packet" | "frame-packet" => {
            Ok(include_str!("../../../schemas/frame_packet.schema.json"))
        }
        "graph" => Ok(include_str!("../../../schemas/graph.schema.json")),
        "inference_packet" | "inference-packet" => Ok(include_str!(
            "../../../schemas/inference_packet.schema.json"
        )),
        "input_event" | "input-event" => {
            Ok(include_str!("../../../schemas/input_event.schema.json"))
        }
        "media_asset" | "media-asset" => {
            Ok(include_str!("../../../schemas/media_asset.schema.json"))
        }
        "ml_model" | "ml-model" => Ok(include_str!("../../../schemas/ml_model.schema.json")),
        "operator_network" | "operator-network" => Ok(include_str!(
            "../../../schemas/operator_network.schema.json"
        )),
        "parameter" => Ok(include_str!("../../../schemas/parameter.schema.json")),
        "permission" => Ok(include_str!("../../../schemas/permission.schema.json")),
        "plugin" => Ok(include_str!("../../../schemas/plugin.schema.json")),
        "resource" => Ok(include_str!("../../../schemas/resource.schema.json")),
        "runtime_snapshot" | "runtime-snapshot" => Ok(include_str!(
            "../../../schemas/runtime_snapshot.schema.json"
        )),
        "scene" => Ok(include_str!("../../../schemas/scene.schema.json")),
        "timeline" => Ok(include_str!("../../../schemas/timeline.schema.json")),
        "visual_node" | "visual-node" => {
            Ok(include_str!("../../../schemas/visual_node.schema.json"))
        }
        other => Err(SchemaError::UnknownSchema(other.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_valid_scene() {
        let scene = SceneManifest {
            name: "test".to_string(),
            version: "0.1".to_string(),
            runtime: "frame0".to_string(),
            permissions: PermissionSet::default(),
            clock: ClockConfig {
                primary: "manual".to_string(),
                fallback: "monotonic".to_string(),
                fixed_step_ns: Some(16_666_667),
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
                    shader: Some("shader.msl".to_string()),
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
        };
        assert!(scene.validate().is_empty());
    }

    #[test]
    fn exports_all_schema_json() {
        for name in schema_names() {
            let parsed: Value = serde_json::from_str(schema_json(name).unwrap()).unwrap();
            assert!(parsed.get("$schema").is_some());
        }
    }
}
