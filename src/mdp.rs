use std::{collections::HashMap, fmt::Display};

use crate::model::{self, ActionBuilder, VariableSetAssignment};

#[derive(Debug, Clone)]
pub struct Transition {
    from: usize,
    to: usize,
    reward: f64,
    probability: f64,
    name: String,
}

impl Transition {
    pub fn probability(&self) -> f64 {
        self.probability
    }

    pub fn reward(&self) -> f64 {
        self.reward
    }

    pub fn to(&self) -> usize {
        self.to
    }
}

impl Display for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.1}% chance | S({}) => S({}) | Reward: {}",
            self.probability * 100.0,
            self.from,
            self.to,
            self.reward
        )
    }
}

#[derive(Debug, Clone)]
pub struct Mdp {
    states: Vec<VariableSetAssignment>,
    actions_from_states: Vec<HashMap<String, Vec<Transition>>>,
}

impl Mdp {
    pub fn new(initial: VariableSetAssignment, actions: Vec<model::Action>) -> Self {
        let mut states = vec![initial.clone()];
        let mut transitions = vec![];

        let mut stack = vec![initial];

        while let Some(assignment) = stack.pop() {
            let from_index = states.iter().position(|s| s == &assignment).unwrap();
            for action in actions.iter() {
                if action.preconditions_valid(&assignment) {
                    for (new_state, chance, reward) in action.get_successor_states(&assignment) {
                        let mut found = true;
                        let state_index = states
                            .iter()
                            .position(|s| s == &new_state)
                            .unwrap_or_else(|| {
                                states.push(new_state.clone());
                                found = false;
                                states.len() - 1
                            });
                        let transition = Transition {
                            from: from_index,
                            to: state_index,
                            name: action.name().to_string(),
                            probability: chance,
                            reward,
                        };
                        transitions.push(transition);
                        if !found {
                            stack.push(new_state);
                        }
                    }
                }
            }
        }

        let actions_from_states = states
            .iter()
            .enumerate()
            .map(|(i, _)| {
                transitions
                    .iter()
                    .filter_map(|t| if t.from == i { Some(t.clone()) } else { None })
                    .fold(HashMap::new(), |mut map, t| {
                        let exists = map.get(&t.name).is_some();
                        let vec = if exists {
                            map.get_mut(&t.name).unwrap()
                        } else {
                            map.insert(t.name.clone(), vec![]);
                            map.get_mut(&t.name).unwrap()
                        };

                        vec.push(t);
                        map
                    })
            })
            .collect::<Vec<_>>();

        Self {
            states,
            actions_from_states,
        }
    }

    pub fn print(&self) {
        println!("\n\n================  Transitions From States  ================\n");
        for (i, state) in self.states.iter().enumerate() {
            println!("State {} : {}", i, state);
            for (name, transitions) in self.actions_from_states[i].iter() {
                println!("   Action: {:?}", name);
                for t in transitions {
                    println!("      -> {}", t);
                }
                println!();
            }
            println!();
            println!();
        }
    }

    pub fn states(&self) -> &[VariableSetAssignment] {
        self.states.as_ref()
    }

    pub fn actions(&self, state: usize) -> &HashMap<String, Vec<Transition>> {
        &self.actions_from_states[state]
    }
}

pub struct MdpBuilder {
    actions: Vec<ActionBuilder>,
    initial_state: VariableSetAssignment,
}

impl MdpBuilder {
    pub fn new(initial_state: VariableSetAssignment) -> Self {
        Self {
            actions: vec![],
            initial_state,
        }
    }

    pub fn add_action(mut self, action_builder: ActionBuilder) -> Self {
        self.actions.push(action_builder);
        self
    }

    pub fn build(self) -> Mdp {
        let actions = self.actions.into_iter().map(|a| a.build()).collect();
        Mdp::new(self.initial_state, actions)
    }
}
