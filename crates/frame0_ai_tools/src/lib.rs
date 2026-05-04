use frame0_graph::GraphSnapshot;
use frame0_schema::{Frame0Diagnostic, SceneManifest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorExplanation {
    pub code: String,
    pub summary: String,
    pub likely_causes: Vec<String>,
    pub suggested_commands: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphDiff {
    pub added_nodes: Vec<String>,
    pub removed_nodes: Vec<String>,
    pub added_edges: Vec<String>,
    pub removed_edges: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenePatchReport {
    pub patched_scene: Value,
    pub diagnostics: Vec<Frame0Diagnostic>,
}

pub fn explain_error(diagnostic: &Frame0Diagnostic) -> ErrorExplanation {
    match diagnostic.code.as_str() {
        "GRAPH_REFERENCE_NOT_FOUND" => ErrorExplanation {
            code: diagnostic.code.clone(),
            summary: "A node or output references a missing input/node id.".to_string(),
            likely_causes: vec![
                "The root before the first dot does not exist in inputs or nodes.".to_string(),
                "A node was renamed without updating downstream inputs.".to_string(),
            ],
            suggested_commands: vec![
                "frame0 graph <scene.yaml> --json".to_string(),
                "frame0 inspect <scene.yaml> --json".to_string(),
            ],
        },
        "DEVICE_CAPABILITY_NOT_FOUND" => ErrorExplanation {
            code: diagnostic.code.clone(),
            summary: "The scene asks for a capability that no available device advertises."
                .to_string(),
            likely_causes: vec![
                "A physical device is disconnected or lacks permission.".to_string(),
                "The scene should use the mock adapter for CI.".to_string(),
            ],
            suggested_commands: vec![
                "frame0 devices list --json".to_string(),
                "frame0 devices modes <device-id> --json".to_string(),
            ],
        },
        "SCENE_RUNTIME_UNSUPPORTED" => ErrorExplanation {
            code: diagnostic.code.clone(),
            summary: "The manifest is not targeting the FRAME0 runtime.".to_string(),
            likely_causes: vec!["runtime is missing or not set to frame0.".to_string()],
            suggested_commands: vec!["Set runtime: frame0".to_string()],
        },
        _ => ErrorExplanation {
            code: diagnostic.code.clone(),
            summary: diagnostic.message.clone(),
            likely_causes: vec![
                "No specialized explanation is registered for this code yet.".to_string(),
            ],
            suggested_commands: diagnostic.suggestions.clone(),
        },
    }
}

pub fn suggest_fix(diagnostics: &[Frame0Diagnostic]) -> Vec<String> {
    let mut suggestions = BTreeSet::new();
    for diagnostic in diagnostics {
        for suggestion in &diagnostic.suggestions {
            suggestions.insert(suggestion.clone());
        }
        match diagnostic.code.as_str() {
            "SCENE_HAS_NO_OUTPUTS" => {
                suggestions.insert(
                    "Add an outputs section with at least one screen or headless output"
                        .to_string(),
                );
            }
            "GRAPH_REFERENCE_NOT_FOUND" => {
                suggestions
                    .insert("Rename the reference root or add the missing input/node".to_string());
            }
            "SCENE_RUNTIME_UNSUPPORTED" => {
                suggestions.insert("Set runtime: frame0".to_string());
            }
            _ => {}
        }
    }
    suggestions.into_iter().collect()
}

pub fn diff_graphs(before: &GraphSnapshot, after: &GraphSnapshot) -> GraphDiff {
    let before_nodes: BTreeSet<_> = before.nodes.iter().map(|node| node.id.clone()).collect();
    let after_nodes: BTreeSet<_> = after.nodes.iter().map(|node| node.id.clone()).collect();
    let before_edges: BTreeSet<_> = before
        .edges
        .iter()
        .map(|edge| format!("{}->{}", edge.from, edge.to))
        .collect();
    let after_edges: BTreeSet<_> = after
        .edges
        .iter()
        .map(|edge| format!("{}->{}", edge.from, edge.to))
        .collect();

    GraphDiff {
        added_nodes: after_nodes.difference(&before_nodes).cloned().collect(),
        removed_nodes: before_nodes.difference(&after_nodes).cloned().collect(),
        added_edges: after_edges.difference(&before_edges).cloned().collect(),
        removed_edges: before_edges.difference(&after_edges).cloned().collect(),
    }
}

pub fn merge_patch_scene(scene: &SceneManifest, patch: &Value) -> ScenePatchReport {
    let mut value = serde_json::to_value(scene).expect("scene manifest serializes");
    merge_patch(&mut value, patch);
    let diagnostics = match serde_json::from_value::<SceneManifest>(value.clone()) {
        Ok(scene) => scene.validate(),
        Err(error) => vec![Frame0Diagnostic::error(
            "SCENE_PATCH_INVALID",
            format!("Patch produced an invalid scene: {error}"),
        )],
    };
    ScenePatchReport {
        patched_scene: value,
        diagnostics,
    }
}

fn merge_patch(target: &mut Value, patch: &Value) {
    match (target, patch) {
        (Value::Object(target), Value::Object(patch)) => {
            for (key, value) in patch {
                if value.is_null() {
                    target.remove(key);
                } else {
                    merge_patch(target.entry(key).or_insert(Value::Null), value);
                }
            }
        }
        (target, patch) => {
            *target = patch.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame0_schema::{ClockConfig, PermissionSet};
    use std::collections::BTreeMap;

    #[test]
    fn merge_patch_can_change_scene_name() {
        let scene = SceneManifest {
            name: "old".to_string(),
            version: "0.1".to_string(),
            runtime: "frame0".to_string(),
            permissions: PermissionSet::default(),
            clock: ClockConfig {
                primary: "manual".to_string(),
                fallback: "monotonic".to_string(),
                fixed_step_ns: None,
            },
            inputs: BTreeMap::new(),
            nodes: BTreeMap::new(),
            outputs: BTreeMap::new(),
        };
        let report = merge_patch_scene(&scene, &serde_json::json!({ "name": "new" }));
        assert_eq!(report.patched_scene["name"], "new");
    }
}
