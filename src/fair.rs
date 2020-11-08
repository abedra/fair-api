use crate::types::{ScenarioResult, Scenario, Error};
use rand_distr::{Pert, Distribution};
use std::borrow::BorrowMut;
use either::Either;
use either::Either::{Right, Left};

pub fn model_scenario(scenario: &Scenario) -> Either<Error, ScenarioResult> {
    if scenario.sample_size < 100 {
        return Left(Error{ error: String::from("Sample size too small")});
    }

    let vulnerability = calculate_vulnerability(scenario);
    let loss_event_frequency = loss_event_frequency(vulnerability, scenario.threat_event_frequency);
    let probable_loss = loss_expectancy(scenario.probable_loss_magnitude, loss_event_frequency);
    let worst_case_loss = loss_expectancy(scenario.worst_case_loss_magnitude, loss_event_frequency);

    return Right(ScenarioResult { scenario: scenario.clone(), probable_loss, worst_case_loss });
}

fn calculate_vulnerability(scenario: &Scenario) -> f32 {
    let cs_pert = Pert::new(
        scenario.control_strength.min,
        scenario.control_strength.max,
        scenario.control_strength.most_likely
    ).unwrap();

    let tcap_pert = Pert::new(
        scenario.threat_capability.min,
        scenario.threat_capability.max,
        scenario.threat_capability.most_likely
    ).unwrap();

    let mut rand = rand::thread_rng();
    let mut control_strength_failures = 0;

    for _ in 1..scenario.sample_size {
        let cs = cs_pert.sample(rand.borrow_mut());
        let tcap = tcap_pert.sample(rand.borrow_mut());

        if tcap > cs {
            control_strength_failures += 1;
        }
    }

    return control_strength_failures as f32 / scenario.sample_size as f32;
}

fn loss_expectancy(loss_magnitude: f32, loss_event_frequency: f32) -> f32 {
    return loss_magnitude * loss_event_frequency;
}

fn loss_event_frequency(vulnerability: f32, threat_event_frequency: f32) -> f32 {
    return vulnerability * threat_event_frequency;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ControlStrength, ThreatCapability, Error};

    #[test]
    fn invalid_scenario_size() {
        let scenario = Scenario {
            name: String::from("Test Scenario"),
            sample_size: 0,
            threat_event_frequency: 0.25,
            probable_loss_magnitude: 1000.,
            worst_case_loss_magnitude: 10000.,
            control_strength: ControlStrength { min: 0., max: 100., most_likely: 50. },
            threat_capability: ThreatCapability { min: 0., max: 100., most_likely: 50. }
        };

        let expected = Error { error: String::from("Sample size too small") };

        assert_eq!(expected, model_scenario(&scenario).unwrap_left());
    }
}
