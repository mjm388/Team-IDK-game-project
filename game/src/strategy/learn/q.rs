use std::collections::HashMap;

use crate::mdp::State;
use crate::strategy::learn::LearningStrategy;

pub struct QLearning {
    alpha: f64,
    gamma: f64,
    init: f64,
}

impl QLearning {
    pub fn new(alpha: f64, gamma: f64, init: f64) -> QLearning {
        QLearning {
            alpha,
            gamma,
            init,
        }
    }
}

impl<S: State> LearningStrategy<S> for QLearning {
    fn value(
        &self,
        new_action_values: &Option<&HashMap<S::Act, f64>>,
        current_value: &Option<&f64>,
        reward: f64,
    ) -> f64 {
        let max_next = new_action_values
            .and_then(|m| m.values().max_by(|a, b| a.partial_cmp(b).unwrap()))
            .unwrap_or(&self.init);
        current_value.map_or(self.init, |x| {
            x + self.alpha * (reward + self.gamma * max_next - x)
        })
    }
}