use crate::types::{ScenarioResult, Scenario};
use rand_distr::{Pert, Distribution};
use std::borrow::BorrowMut;

pub fn model_scenario(scenario: &Scenario) -> ScenarioResult {
    let vulnerability = calculate_vulnerability(scenario);
    let loss_event_frequency = loss_event_frequency(vulnerability, scenario.threat_event_frequency);
    let probable_loss = loss_expectancy(scenario.probable_loss_magnitude, loss_event_frequency);
    let worst_case_loss = loss_expectancy(scenario.worst_case_loss_magnitude, loss_event_frequency);

    return ScenarioResult { scenario: scenario.clone(), probable_loss, worst_case_loss };
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
