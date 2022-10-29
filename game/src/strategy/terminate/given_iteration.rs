use crate::mdp::State;
use crate::strategy::terminate::TerminationStrategy;

pub struct FixedIterations {
    i: u32,
    iteration: u32,
}

impl GivenIteration {
    pub fn new(iteration: u32) -> GivenIteration {
        GivenIteration { i: 0, iteration }
    }
}

impl<State> TerminationStrategy<State> for GivenIteration {
    fn should_stop(&mut self, _: &State) -> bool {
        self.i += 1;
        self.i > self.iteration
    }
}
