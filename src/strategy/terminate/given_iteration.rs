use crate::mdp::State;
use crate::strategy::terminate::TerminationStrategy;

pub struct GivenIteration {
    i: u32,
    iteration: u32,
}

impl GivenIteration {
    pub fn new(iteration: u32) -> GivenIteration {
        GivenIteration { i: 0, iteration }
    }
}

impl<S: State> TerminationStrategy<S> for GivenIteration {
    fn should_stop(&mut self, _: &S) -> bool {
        self.i += 1;
        self.i > self.iteration
    }
}
