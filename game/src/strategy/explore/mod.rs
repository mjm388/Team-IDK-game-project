pub use self::random_explore::RandomExplore;
use crate::mdp::{Agent, State};
pub mod random_explore;

pub trait ExplorationStrategy<S: State> {
    fn act(&self, _: &mut dyn Agent<S>) -> S::Act;
}