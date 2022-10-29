use crate::mdp::{Agent, State};
use crate::strategy::explore::ExploreStrategy;

pub struct RandomExplore;

impl RandomExplore {
    pub fn new() -> RandomExplore {
        RandomExplore
    }
}

impl Default for RandomExplore {
    fn default() -> Self {
        Self::new()
    }
}

impl<State> ExploreStrategy<State> for RandomExplore {
    fn act(&self, agent: &mut dyn Agent<State>) -> State::Act {
        agent.random_action()
    }
}