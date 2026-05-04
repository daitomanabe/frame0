use frame0_schema::{Frame0Diagnostic, SceneManifest};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GraphNodeKind {
    Input,
    Node,
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub kind: GraphNodeKind,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphSnapshot {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub topological_order: Vec<String>,
    pub diagnostics: Vec<Frame0Diagnostic>,
}

pub fn build_graph(scene: &SceneManifest) -> GraphSnapshot {
    let mut diagnostics = scene.validate();
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for (id, input) in &scene.inputs {
        nodes.push(GraphNode {
            id: id.clone(),
            kind: GraphNodeKind::Input,
            node_type: input.component_type.clone(),
            dependencies: Vec::new(),
        });
    }

    for (id, node) in &scene.nodes {
        let dependencies = normalize_dependencies(node.dependency_refs());
        for dep in &dependencies {
            edges.push(GraphEdge {
                from: dep.clone(),
                to: id.clone(),
                label: None,
            });
        }
        nodes.push(GraphNode {
            id: id.clone(),
            kind: GraphNodeKind::Node,
            node_type: node.component_type.clone(),
            dependencies,
        });
    }

    for (id, output) in &scene.outputs {
        let dependencies = normalize_dependencies(output.dependency_refs());
        for dep in &dependencies {
            edges.push(GraphEdge {
                from: dep.clone(),
                to: id.clone(),
                label: None,
            });
        }
        nodes.push(GraphNode {
            id: id.clone(),
            kind: GraphNodeKind::Output,
            node_type: output.component_type.clone(),
            dependencies,
        });
    }

    let topological_order = match topological_sort(&nodes, &edges) {
        Ok(order) => order,
        Err(cycle) => {
            diagnostics.push(
                Frame0Diagnostic::error(
                    "GRAPH_CYCLE_DETECTED",
                    format!("Graph contains a dependency cycle involving '{}'", cycle),
                )
                .with_resource(cycle),
            );
            Vec::new()
        }
    };

    GraphSnapshot {
        nodes,
        edges,
        topological_order,
        diagnostics,
    }
}

fn normalize_dependencies(refs: Vec<String>) -> Vec<String> {
    refs.into_iter()
        .filter_map(|reference| reference.split('.').next().map(str::to_string))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn topological_sort(nodes: &[GraphNode], edges: &[GraphEdge]) -> Result<Vec<String>, String> {
    let node_ids: BTreeSet<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let mut in_degree: BTreeMap<String, usize> =
        node_ids.iter().map(|id| (id.clone(), 0_usize)).collect();
    let mut adjacency: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for edge in edges {
        if !node_ids.contains(&edge.from) || !node_ids.contains(&edge.to) {
            continue;
        }
        adjacency
            .entry(edge.from.clone())
            .or_default()
            .push(edge.to.clone());
        *in_degree.entry(edge.to.clone()).or_default() += 1;
    }

    let mut queue: VecDeque<String> = in_degree
        .iter()
        .filter_map(|(id, count)| (*count == 0).then(|| id.clone()))
        .collect();
    let mut order = Vec::new();

    while let Some(id) = queue.pop_front() {
        order.push(id.clone());
        if let Some(targets) = adjacency.get(&id) {
            for target in targets {
                if let Some(count) = in_degree.get_mut(target) {
                    *count -= 1;
                    if *count == 0 {
                        queue.push_back(target.clone());
                    }
                }
            }
        }
    }

    if order.len() == node_ids.len() {
        Ok(order)
    } else {
        let cycle = in_degree
            .into_iter()
            .find_map(|(id, count)| (count > 0).then_some(id))
            .unwrap_or_else(|| "unknown".to_string());
        Err(cycle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame0_schema::{ClockConfig, ComponentSpec, PermissionSet};
    use std::collections::BTreeMap;

    fn component(kind: &str, input: Option<&str>) -> ComponentSpec {
        ComponentSpec {
            component_type: kind.to_string(),
            selector: None,
            input: input.map(str::to_string),
            inputs: BTreeMap::new(),
            mode: None,
            pixel_format: None,
            shader: None,
            params: BTreeMap::new(),
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn graph_orders_dependencies() {
        let scene = SceneManifest {
            name: "graph".to_string(),
            version: "0.1".to_string(),
            runtime: "frame0".to_string(),
            permissions: PermissionSet::default(),
            clock: ClockConfig {
                primary: "manual".to_string(),
                fallback: "monotonic".to_string(),
                fixed_step_ns: None,
            },
            inputs: BTreeMap::from([("camera".to_string(), component("device.video_input", None))]),
            nodes: BTreeMap::from([(
                "shader".to_string(),
                component("render.shader", Some("camera.video")),
            )]),
            outputs: BTreeMap::from([(
                "screen".to_string(),
                component("screen", Some("shader.output")),
            )]),
        };
        let graph = build_graph(&scene);
        assert!(graph.diagnostics.is_empty());
        assert_eq!(graph.topological_order, vec!["camera", "shader", "screen"]);
    }
}
