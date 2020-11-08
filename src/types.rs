use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Heartbeat {
    pub status: String,
    pub version: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ControlStrength {
    pub min: f32,
    pub max: f32,
    pub most_likely: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ThreatCapability {
    pub min: f32,
    pub max: f32,
    pub most_likely: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Scenario {
    pub name: String,
    pub sample_size: i32,
    pub threat_event_frequency: f32,
    pub probable_loss_magnitude: f32,
    pub worst_case_loss_magnitude: f32,
    pub control_strength: ControlStrength,
    pub threat_capability: ThreatCapability,
}

#[derive(Serialize, Deserialize)]
pub struct ScenarioResult {
    pub scenario: Scenario,
    pub probable_loss: f32,
    pub worst_case_loss: f32
}
