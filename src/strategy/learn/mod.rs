use std::collections::HashMap;
pub use self::q::QLearning;
use crate::mdp::State;
pub mod q;

pub trait LearningStrategy<S: State> {
    fn value(
        &self,
        new_action_values: &Option<&HashMap<S::Act, f64>>,
        current_value: &Option<&f64>,
        reward: f64,
        init_reward: f64,
    ) -> f64;
}