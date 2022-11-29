pub mod mdp;
pub mod strategy;

use std::fmt::Display;
use std::{collections::HashMap};
use mdp::{Agent, State};
use serde::ser::{SerializeMap};
use serde::{Serialize, Serializer};
use strategy::explore::ExplorationStrategy;
use strategy::learn::LearningStrategy;
use strategy::terminate::TerminationStrategy;

pub struct AgentTrainer<S>
where
    S: State + Serialize + Display,
{
    pub q: HashMap<S, HashMap<S::Act, f64>>,
}

pub struct AgentT<S>
where
    S: State + Serialize + Display,
{
    pub q: HashMap<S, HashMap<S::Act, isize>>,
}

impl<S> AgentT<S>
where
    S: State + Serialize + Display,
{
    pub fn new() -> AgentT<S> {
        AgentT { q: HashMap::new() }
    }
}

impl<S> AgentTrainer<S>
where
    S: State + Serialize + Display,
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
        init_state: &S,
    ) {
        loop {
            let s_t = agent.current_state().clone();
            let action = exploration_strategy.act(agent);
            let r_t_next = agent.act(&action);
            // current action value
            let s_t_next = agent.current_state();

            let reset = s_t_next == init_state;

            let v = {
                let old_value = self.q.get(&s_t).and_then(|m| m.get(&action));
                learning_strategy.value(&self.q.get(s_t_next), &old_value, r_t_next, reset)
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

impl<S: State + Serialize + Display> Default for AgentTrainer<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: State + Serialize + Display> Default for AgentT<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: State + Serialize + Display> Serialize for AgentT<S> {
    fn serialize<S1>(&self, serializer: S1) -> Result<S1::Ok, S1::Error>
    where
        S1: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.q.len()))?;
        for (k, v) in &self.q {
            map.serialize_entry(&k.to_string(), &v)?;
        }
        map.end()
    }
}

// impl<'de, S: State + Serialize + Display> Deserialize<'de> for AgentTrainer<S> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>, 
//     {
//         struct ConnectorTopicsVisitor;

//         impl<'de> Visitor<'de> for ConnectorTopicsVisitor {
//             type Value = AgentTrainer<State + Serialize + Display>;

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("ConnectorTopics")
//             }

//             fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
//             where
//                 V: MapAccess<'de>,
//             {
//                 if let Some(key) = map.next_key()? {
//                     let value: Inner = map.next_value()?;
//                     if let Some(_) = map.next_key::<&str>()? {
//                         Err(Error::duplicate_field("name"))
//                     } else {
//                         Ok(Self::Value {
//                             name: key,
//                             topics: value.topics,
//                         })
//                     }
//                 } else {
//                     Err(Error::missing_field("name"))
//                 }
//             }
//         }

//         deserializer.deserialize_map(ConnectorTopicsVisitor {})
//     }
// }

// #[derive(Debug, Deserialize)]
// struct Inner<S>
// where
//     S: State + Serialize + Display, {
//     q_values: HashMap<S::Act, f64>,
// }