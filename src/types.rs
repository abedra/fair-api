use serde::{Deserialize, Serialize};
use either::Either::{Left, Right};
use either::Either;

#[derive(Serialize, Deserialize)]
pub struct Heartbeat {
    pub status: String,
    pub version: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ControlStrength {
    pub min: f32,
    pub max: f32,
    pub most_likely: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThreatCapability {
    pub min: f32,
    pub max: f32,
    pub most_likely: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Scenario {
    pub name: String,
    pub sample_size: i32,
    pub threat_event_frequency: f32,
    pub probable_loss_magnitude: f32,
    pub worst_case_loss_magnitude: f32,
    pub control_strength: ControlStrength,
    pub threat_capability: ThreatCapability,
}

impl Scenario {
    pub fn is_valid(&self) -> Either<Error, ()> {
        if self.sample_size < 100 {
            return Left(Error{ error: String::from("Sample size must be 100 or greater")});
        }

        if self.control_strength.max <= 0. {
            return Left(Error{ error: String::from("Control strength max must be greater than 0")});
        }

        if self.threat_capability.max <= 0. {
            return Left(Error{ error: String::from("Threat capability max must be greater than 0")});
        }

        if self.threat_event_frequency <= 0. {
            return Left(Error{ error: String::from("Threat event frequency must be greater than 0")})
        }

        Right(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScenarioResult {
    pub scenario: Scenario,
    pub probable_loss: f32,
    pub worst_case_loss: f32
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Error {
    pub error: String
}
