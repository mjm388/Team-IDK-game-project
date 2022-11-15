pub mod mdp;
pub mod strategy;

use std::fmt::{Display, Formatter};
use std::{collections::HashMap, fs::File};
use mdp::{Agent, State};
use serde::ser::{SerializeStruct, SerializeMap};
use serde::{Serialize, Serializer, Deserialize};
use strategy::explore::ExplorationStrategy;
use strategy::learn::LearningStrategy;
use strategy::terminate::TerminationStrategy;
use serde_with::serde_as;

#[serde_as]
#[derive(Serialize)]
pub struct AgentTrainer<S>
where
    S: State + Serialize,
{
    #[serde_as(as = "Vec<(_, Vec<(_, _)>)>")]
    pub q: HashMap<S, HashMap<S::Act, f64>>,
}

impl<S> AgentTrainer<S>
where
    S: State + Serialize,
{
    pub fn new() -> AgentTrainer<S> {
        AgentTrainer { q: HashMap::new() }
    }

    pub fn expected_values(&self, state: &S) -> Option<&HashMap<S::Act, f64>> {
        self.q.get(state)
    }

    pub fn expected_value(&self, state: &S, action: &S::Act) -> Option<f64> {
        self.q.get(state).and_then(|m| m.get(action).copied())
    }

    pub fn export_learned_values(&self) -> HashMap<S, HashMap<S::Act, f64>> {
        self.q.clone()
    }

    pub fn best_action(&self, state: &S) -> Option<S::Act> {
        self.expected_values(state)
            .and_then(|m| {
                m.iter()
                    .max_by(|&(_, v1), &(_, v2)| v1.partial_cmp(v2).unwrap())
            })
            .map(|t| t.0.clone())
    }

    pub fn train(
        &mut self,
        agent: &mut dyn Agent<S>,
        learning_strategy: &dyn LearningStrategy<S>,
        termination_strategy: &mut dyn TerminationStrategy<S>,
        exploration_strategy: &dyn ExplorationStrategy<S>,
    ) {
        loop {
            let s_t = agent.current_state().clone();
            let action = exploration_strategy.act(agent);

            // current action value
            let s_t_next = agent.current_state();
            let r_t_next = s_t_next.reward();

            let v = {
                let old_value = self.q.get(&s_t).and_then(|m| m.get(&action));
                learning_strategy.value(&self.q.get(s_t_next), &old_value, r_t_next)
            };

            self.q
                .entry(s_t)
                .or_insert_with(HashMap::new)
                .insert(action, v);

            if termination_strategy.should_stop(s_t_next) {
                break;
            }
        }
    }


}

impl<S: State + Serialize> Default for AgentTrainer<S> {
    fn default() -> Self {
        Self::new()
    }
}


// impl <S: State> Serialize for AgentTrainer<S> {
//     fn serialize<S1>(&self, serializer: S1) -> Result<S1::Ok, S1::Error>
//     where
//         S1: Serializer,
//     {
//         let mut map = serializer.serialize_map(Some(self.q.len()))?;
//         for (k, v) in &self.q {
//             map.serialize_entry(&k.to_string(), &v)?;
//         }
//         map.end()
//     }
// }
