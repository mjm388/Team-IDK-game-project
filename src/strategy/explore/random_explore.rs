use crate::mdp::{Agent, State};
use crate::strategy::explore::ExplorationStrategy;

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

impl<S: State> ExplorationStrategy<S> for RandomExplore {
    fn act(&self, agent: &mut dyn Agent<S>) -> S::Act {
        agent.random_act()
    }
}