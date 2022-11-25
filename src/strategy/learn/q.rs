use std::collections::HashMap;

use crate::mdp::State;
use crate::strategy::learn::LearningStrategy;

pub struct QLearning {
    alpha: f64,
    gamma: f64,
}

impl QLearning {
    pub fn new(alpha: f64, gamma: f64) -> QLearning {
        QLearning {
            alpha,
            gamma,
        }
    }
}

/*impl<S: State> LearningStrategy<S> for QLearning {
    fn value(
        &self,
        new_action_values: &Option<&HashMap<S::Act, f64>>,
        current_value: &Option<&f64>,
        reward: f64,
        init_reward: f64,
        reset: bool,
    ) -> f64 {
        // estimation of max future value
        // the init_reward is used when the state is reset or does not have an existing reward for the next state
        let max_next = 
            if reset {0} 
            else { 
                new_action_values
                .and_then(|m| m.values().max_by(|a, b| a.partial_cmp(b).unwrap()))
                .unwrap_or(&init_reward)
            };
        // Bellman Equation
        current_value.map_or(reward, |x| {
            x + self.alpha * (reward + self.gamma * max_next - x)
        })
    }
}*/