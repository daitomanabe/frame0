use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceMode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceResource {
    pub id: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub vendor: String,
    pub capabilities: Vec<String>,
    pub status: String,
    pub process: String,
    pub permissions: Vec<String>,
    pub modes: Vec<DeviceMode>,
}

impl DeviceResource {
    pub fn supports(&self, capability: &str) -> bool {
        self.capabilities.iter().any(|item| item == capability)
    }
}

pub fn mock_devices() -> Vec<DeviceResource> {
    vec![
        DeviceResource {
            id: "device.video_input.mock.0".to_string(),
            device_type: "device.video_input".to_string(),
            vendor: "frame0.mock".to_string(),
            capabilities: vec![
                "video.input".to_string(),
                "timecode.input".to_string(),
                "gpu.texture.export".to_string(),
            ],
            status: "available".to_string(),
            process: "plugin_host.mock_video".to_string(),
            permissions: vec!["camera".to_string()],
            modes: vec![
                DeviceMode {
                    id: "1080p60".to_string(),
                    width: Some(1920),
                    height: Some(1080),
                    fps: Some(60.0),
                    sample_rate: None,
                    channels: None,
                },
                DeviceMode {
                    id: "720p60".to_string(),
                    width: Some(1280),
                    height: Some(720),
                    fps: Some(60.0),
                    sample_rate: None,
                    channels: None,
                },
            ],
        },
        DeviceResource {
            id: "device.audio_input.mock.0".to_string(),
            device_type: "device.audio_input".to_string(),
            vendor: "frame0.mock".to_string(),
            capabilities: vec!["audio.input".to_string()],
            status: "available".to_string(),
            process: "plugin_host.mock_audio".to_string(),
            permissions: vec!["microphone".to_string()],
            modes: vec![
                DeviceMode {
                    id: "stereo_48k".to_string(),
                    width: None,
                    height: None,
                    fps: None,
                    sample_rate: Some(48_000),
                    channels: Some(2),
                },
                DeviceMode {
                    id: "mono_48k".to_string(),
                    width: None,
                    height: None,
                    fps: None,
                    sample_rate: Some(48_000),
                    channels: Some(1),
                },
            ],
        },
        DeviceResource {
            id: "device.osc.mock.0".to_string(),
            device_type: "device.control_input".to_string(),
            vendor: "frame0.mock".to_string(),
            capabilities: vec!["osc.input".to_string()],
            status: "available".to_string(),
            process: "plugin_host.mock_control".to_string(),
            permissions: vec!["network".to_string()],
            modes: vec![DeviceMode {
                id: "udp_9000".to_string(),
                width: None,
                height: None,
                fps: None,
                sample_rate: None,
                channels: None,
            }],
        },
        DeviceResource {
            id: "extension.audio_unit.mock.0".to_string(),
            device_type: "extension.audio_unit".to_string(),
            vendor: "frame0.mock".to_string(),
            capabilities: vec![
                "os.audio_unit".to_string(),
                "extension.input".to_string(),
                "extension.output".to_string(),
                "auv3.parameter".to_string(),
                "auv3.midi".to_string(),
            ],
            status: "available".to_string(),
            process: "plugin_host.extension_auv3".to_string(),
            permissions: vec!["microphone".to_string(), "file_read".to_string()],
            modes: vec![DeviceMode {
                id: "auv3_parameter_bus".to_string(),
                width: None,
                height: None,
                fps: None,
                sample_rate: Some(48_000),
                channels: Some(2),
            }],
        },
        DeviceResource {
            id: "extension.camera_output.mock.0".to_string(),
            device_type: "extension.camera_output".to_string(),
            vendor: "frame0.mock".to_string(),
            capabilities: vec![
                "os.camera_extension".to_string(),
                "video.output".to_string(),
                "extension.output".to_string(),
                "iosurface.import".to_string(),
                "coremedia.sample_buffer".to_string(),
            ],
            status: "available".to_string(),
            process: "plugin_host.extension_camera".to_string(),
            permissions: vec!["camera".to_string(), "file_read".to_string()],
            modes: vec![DeviceMode {
                id: "1080p60_bgra_iosurface".to_string(),
                width: Some(1920),
                height: Some(1080),
                fps: Some(60.0),
                sample_rate: None,
                channels: None,
            }],
        },
    ]
}

pub fn find_device(id: &str) -> Option<DeviceResource> {
    mock_devices().into_iter().find(|device| device.id == id)
}

pub fn devices_with_capability(capability: &str) -> Vec<DeviceResource> {
    mock_devices()
        .into_iter()
        .filter(|device| device.supports(capability))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_devices_cover_video_and_audio() {
        assert!(!devices_with_capability("video.input").is_empty());
        assert!(!devices_with_capability("audio.input").is_empty());
    }
}
