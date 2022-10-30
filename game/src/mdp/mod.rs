use std::hash::Hash;

pub trait State: Eq + Hash + Clone {
    // Action type
    type Act: Eq + Hash + Clone;
    // The reward when agent reaches this state
    fn reward(&self) -> f64;
    // available actions to reach another state from here
    fn action_set(&self) -> Vec<Self::Act>;
    // select random action
    fn random_action(&self) -> Self::Action {
        let actions = self.action_set();
        let action = rand::random::<usize>() % action_set.len();
        action_set[action].clone()
    }
}



pub trait Agent<S: State> {
    fn current_state(&self) -> &S;
    fn act(&mut self, action: &S::Act);
    fn random_act(&mut self) -> S::Act {
        let action = self.current_state().random_action();
        self.act(&action);
        action
    }
}