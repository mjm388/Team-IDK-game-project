pub use self::fixed_iterations::FixedIterations;
use crate::mdp::State;

pub mod given_iteration;

pub trait TerminationStrategy<State> {
    fn should_stop(&mut self, state: &State) -> bool;
}
