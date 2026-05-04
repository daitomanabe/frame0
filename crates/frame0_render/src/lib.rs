use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderCapabilityReport {
    pub backend: String,
    pub available: bool,
    pub supports_window_preview: bool,
    pub supports_headless: bool,
    pub supports_gpu_timing: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TexturePoolStats {
    pub allocated_textures: u64,
    pub reused_textures: u64,
    pub leaked_textures: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeadlessFrameReport {
    pub frame_index: u64,
    pub pts_ns: u64,
    pub width: u32,
    pub height: u32,
    pub backend: String,
    pub gpu_ms: f64,
}

pub fn render_capabilities() -> Vec<RenderCapabilityReport> {
    vec![
        RenderCapabilityReport {
            backend: "metal".to_string(),
            available: cfg!(target_os = "macos"),
            supports_window_preview: cfg!(target_os = "macos"),
            supports_headless: cfg!(target_os = "macos"),
            supports_gpu_timing: false,
            notes: vec![
                "Native Metal binding is intentionally outside frame0_core".to_string(),
                "Use this adapter boundary for future metal-rs or Swift bridge work".to_string(),
            ],
        },
        RenderCapabilityReport {
            backend: "headless_mock".to_string(),
            available: true,
            supports_window_preview: false,
            supports_headless: true,
            supports_gpu_timing: false,
            notes: vec!["Deterministic CI renderer for graph/runtime testing".to_string()],
        },
    ]
}

pub fn simulate_headless_frames(
    frames: u64,
    step_ns: u64,
    width: u32,
    height: u32,
) -> Vec<HeadlessFrameReport> {
    (0..frames)
        .map(|frame_index| HeadlessFrameReport {
            frame_index,
            pts_ns: frame_index * step_ns,
            width,
            height,
            backend: "headless_mock".to_string(),
            gpu_ms: 1.0,
        })
        .collect()
}

pub fn empty_texture_pool_stats() -> TexturePoolStats {
    TexturePoolStats {
        allocated_textures: 0,
        reused_textures: 0,
        leaked_textures: 0,
    }
}
