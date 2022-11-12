pub use self::given_iteration::GivenIteration;
use crate::mdp::State;

pub mod given_iteration;

pub trait TerminationStrategy<S: State> {
    fn should_stop(&mut self, state: &S) -> bool;
}
