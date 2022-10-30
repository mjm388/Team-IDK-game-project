use std::collections::HashMap;
pub use self::q::QLearning;
use crate::mdp::State;
pub mod q;

pub trait LearningStrategy<State> {
    fn value(
        &self,
        new_action_values: &Option<&HashMap<State::Act, f64>>,
        current_value: &Option<&f64>,
        reward: f64,
    ) -> f64;
}