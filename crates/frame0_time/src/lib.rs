use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use time::OffsetDateTime;

pub const DEFAULT_FIXED_STEP_NS: u64 = 16_666_667;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClockSnapshot {
    pub id: String,
    pub kind: String,
    pub pts_ns: u64,
    pub frame_index: u64,
    pub fixed_step_ns: Option<u64>,
    pub drift_ns: i64,
}

pub trait Clock {
    fn id(&self) -> &str;
    fn kind(&self) -> &str;
    fn now_ns(&self) -> u64;
    fn frame_index(&self) -> u64;
    fn snapshot(&self) -> ClockSnapshot;
}

#[derive(Debug, Clone)]
pub struct ManualClock {
    id: String,
    pts_ns: u64,
    frame_index: u64,
}

impl ManualClock {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            pts_ns: 0,
            frame_index: 0,
        }
    }

    pub fn set_ns(&mut self, pts_ns: u64) {
        self.pts_ns = pts_ns;
    }

    pub fn advance_ns(&mut self, delta_ns: u64) {
        self.pts_ns += delta_ns;
        self.frame_index += 1;
    }
}

impl Clock for ManualClock {
    fn id(&self) -> &str {
        &self.id
    }

    fn kind(&self) -> &str {
        "manual"
    }

    fn now_ns(&self) -> u64 {
        self.pts_ns
    }

    fn frame_index(&self) -> u64 {
        self.frame_index
    }

    fn snapshot(&self) -> ClockSnapshot {
        ClockSnapshot {
            id: self.id.clone(),
            kind: self.kind().to_string(),
            pts_ns: self.pts_ns,
            frame_index: self.frame_index,
            fixed_step_ns: None,
            drift_ns: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FixedStepClock {
    id: String,
    step_ns: u64,
    frame_index: u64,
}

impl FixedStepClock {
    pub fn new(id: impl Into<String>, step_ns: u64) -> Self {
        Self {
            id: id.into(),
            step_ns,
            frame_index: 0,
        }
    }

    pub fn advance_frame(&mut self) {
        self.frame_index += 1;
    }

    pub fn pts_for_frame(&self, frame_index: u64) -> u64 {
        self.step_ns.saturating_mul(frame_index)
    }
}

impl Clock for FixedStepClock {
    fn id(&self) -> &str {
        &self.id
    }

    fn kind(&self) -> &str {
        "fixed_step"
    }

    fn now_ns(&self) -> u64 {
        self.pts_for_frame(self.frame_index)
    }

    fn frame_index(&self) -> u64 {
        self.frame_index
    }

    fn snapshot(&self) -> ClockSnapshot {
        ClockSnapshot {
            id: self.id.clone(),
            kind: self.kind().to_string(),
            pts_ns: self.now_ns(),
            frame_index: self.frame_index,
            fixed_step_ns: Some(self.step_ns),
            drift_ns: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonotonicClock {
    id: String,
    started_at: Instant,
}

impl MonotonicClock {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            started_at: Instant::now(),
        }
    }
}

impl Clock for MonotonicClock {
    fn id(&self) -> &str {
        &self.id
    }

    fn kind(&self) -> &str {
        "monotonic"
    }

    fn now_ns(&self) -> u64 {
        duration_to_ns(self.started_at.elapsed())
    }

    fn frame_index(&self) -> u64 {
        self.now_ns() / DEFAULT_FIXED_STEP_NS
    }

    fn snapshot(&self) -> ClockSnapshot {
        ClockSnapshot {
            id: self.id.clone(),
            kind: self.kind().to_string(),
            pts_ns: self.now_ns(),
            frame_index: self.frame_index(),
            fixed_step_ns: None,
            drift_ns: 0,
        }
    }
}

pub fn duration_to_ns(duration: Duration) -> u64 {
    duration
        .as_secs()
        .saturating_mul(1_000_000_000)
        .saturating_add(duration.subsec_nanos() as u64)
}

pub fn timestamp_now_utc() -> String {
    OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
}

pub fn deterministic_frame_times(frames: u64, step_ns: u64) -> Vec<u64> {
    (0..frames).map(|frame| frame * step_ns).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_step_is_deterministic() {
        assert_eq!(
            deterministic_frame_times(4, 10),
            vec![0_u64, 10_u64, 20_u64, 30_u64]
        );
        let mut clock = FixedStepClock::new("clock.test", 10);
        assert_eq!(clock.now_ns(), 0);
        clock.advance_frame();
        assert_eq!(clock.now_ns(), 10);
    }
}
