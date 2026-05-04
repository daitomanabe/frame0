use frame0_schema::PermissionSet;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub const FRAME0_PLUGIN_API_VERSION: u32 = 1;

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("failed to read {path}: {source}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse plugin manifest {path}: {source}")]
    Parse {
        path: String,
        #[source]
        source: serde_yaml::Error,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginManifestFile {
    pub plugin: PluginManifest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub version: String,
    #[serde(rename = "type")]
    pub plugin_type: String,
    pub entry: BTreeMap<String, String>,
    pub api_version: u32,
    pub capabilities: Vec<String>,
    pub isolation: PluginIsolation,
    pub permissions: PermissionSet,
    #[serde(default)]
    pub vendor: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginIsolation {
    pub process: String,
    pub restart_policy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginVerification {
    pub id: String,
    pub ok: bool,
    pub diagnostics: Vec<String>,
}

pub fn load_plugin_manifest(path: impl AsRef<Path>) -> Result<PluginManifestFile, PluginError> {
    let path = path.as_ref();
    let text = fs::read_to_string(path).map_err(|source| PluginError::Read {
        path: path.display().to_string(),
        source,
    })?;
    serde_yaml::from_str(&text).map_err(|source| PluginError::Parse {
        path: path.display().to_string(),
        source,
    })
}

pub fn verify_plugin(manifest: &PluginManifest) -> PluginVerification {
    let mut diagnostics = Vec::new();
    if manifest.api_version != FRAME0_PLUGIN_API_VERSION {
        diagnostics.push(format!(
            "api_version {} does not match runtime API version {}",
            manifest.api_version, FRAME0_PLUGIN_API_VERSION
        ));
    }
    if manifest.isolation.process != "separate" {
        diagnostics
            .push("v0.1 native plugins should default to separate process isolation".to_string());
    }
    if manifest.capabilities.is_empty() {
        diagnostics.push("plugin must declare at least one capability".to_string());
    }
    PluginVerification {
        id: manifest.id.clone(),
        ok: diagnostics.is_empty(),
        diagnostics,
    }
}

pub fn discover_plugin_manifests(root: impl AsRef<Path>) -> Vec<PathBuf> {
    let root = root.as_ref();
    let Ok(entries) = fs::read_dir(root) else {
        return Vec::new();
    };
    entries
        .filter_map(Result::ok)
        .flat_map(|entry| {
            let path = entry.path();
            if path.is_dir() {
                vec![path.join("plugin.yaml"), path.join("plugin.yml")]
            } else {
                vec![path]
            }
        })
        .filter(|path| path.is_file())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name == "plugin.yaml" || name == "plugin.yml")
        })
        .collect()
}

pub fn list_plugins(root: impl AsRef<Path>) -> Vec<PluginManifest> {
    discover_plugin_manifests(root)
        .into_iter()
        .filter_map(|path| load_plugin_manifest(path).ok().map(|file| file.plugin))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_api_version() {
        let manifest = PluginManifest {
            id: "io.frame0.test".to_string(),
            version: "0.1.0".to_string(),
            plugin_type: "native".to_string(),
            entry: BTreeMap::from([("macos_arm64".to_string(), "libtest.dylib".to_string())]),
            api_version: FRAME0_PLUGIN_API_VERSION,
            capabilities: vec!["video.input".to_string()],
            isolation: PluginIsolation {
                process: "separate".to_string(),
                restart_policy: "on_crash".to_string(),
            },
            permissions: PermissionSet::default(),
            vendor: BTreeMap::new(),
        };
        assert!(verify_plugin(&manifest).ok);
    }
}
