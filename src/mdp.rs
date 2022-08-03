use std::collections::HashMap;

use crate::model::{self, VariableSetAssignment};

#[derive(Debug, Clone)]
pub struct Transition {
    from: usize,
    to: usize,
    reward: f64,
    probability: f64,
    name: String,
}

#[derive(Debug, Clone)]
pub struct Mdp {
    states: Vec<VariableSetAssignment>,
    transitions: Vec<Transition>,
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
            transitions,
            actions_from_states,
        }
    }

    pub fn print(&self) {
        println!("\n\n================    States    ================");
        for (i, state) in self.states.iter().enumerate() {
            println!("State {:?}   =>   {:?}", i, state);
        }
        println!("\n\n================  Transitions  ================");
        for (i, transition) in self.transitions.iter().enumerate() {
            println!("Transition {:?}   =>  {:?}", i, transition);
        }

        println!("\n\n================  Transitions From States  ================\n");
        for (i, state) in self.states.iter().enumerate() {
            println!("State {:?} :  {:?}", i, state);
            for (name, transitions) in self.actions_from_states[i].iter() {
                println!("   Action: {:?}", name);
                for t in transitions {
                    println!("      -> {:?}", t);
                }
                println!();
            }
            println!();
            println!();
        }
    }
}