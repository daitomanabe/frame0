pub const ADDON_ID: &str = "org.example.frame0.rust_addon";
pub const ADDON_VERSION: &str = "0.1.0";
pub const FRAME0_API_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddonDescriptor {
    pub id: &'static str,
    pub version: &'static str,
    pub api_version: u32,
    pub capabilities: &'static [&'static str],
}

pub fn descriptor() -> AddonDescriptor {
    AddonDescriptor {
        id: ADDON_ID,
        version: ADDON_VERSION,
        api_version: FRAME0_API_VERSION,
        capabilities: &["visual.node", "parameter.processor"],
    }
}

pub fn process_parameter(input: f32, gain: f32, bias: f32) -> f32 {
    (input * gain + bias).clamp(0.0, 1.0)
}

pub fn event_json(event: &str, value: f32) -> String {
    format!(
        "{{\"event\":\"{}\",\"source\":\"{}\",\"value\":{:.6}}}",
        escape_json(event),
        ADDON_ID,
        value
    )
}

fn escape_json(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_declares_capabilities() {
        let descriptor = descriptor();
        assert_eq!(descriptor.api_version, 1);
        assert!(descriptor.capabilities.contains(&"visual.node"));
    }

    #[test]
    fn process_parameter_clamps_output() {
        assert_eq!(process_parameter(0.5, 2.0, 0.25), 1.0);
        assert_eq!(process_parameter(-1.0, 1.0, 0.0), 0.0);
    }

    #[test]
    fn event_json_escapes_strings() {
        let event = event_json("quote\"test", 0.25);
        assert!(event.contains("quote\\\"test"));
        assert!(event.contains("\"value\":0.250000"));
    }
}
