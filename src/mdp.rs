use crate::model::{self, ActionType, IActionBuilder, State};

use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct Transition {
    from: usize,
    to: usize,
    reward: f64,
    probability: f64,
    action: Rc<dyn ActionType>,
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
pub struct Mdp<S: State> {
    states: Vec<S>,
    actions_from_states: Vec<HashMap<Rc<dyn ActionType>, Vec<Transition>>>,
}

impl<S: State> Mdp<S> {
    pub fn new(initial: S, actions: Vec<model::Action<S>>) -> Self {
        let mut stack = vec![initial.get_hash()];
        let mut hashmap: HashMap<u64, usize> = HashMap::new();
        hashmap.insert(initial.get_hash(), 0);
        let mut states = vec![initial];

        let mut transitions = vec![];

        while let Some(state_hash) = stack.pop() {
            let from_index = *hashmap.get(&state_hash).unwrap();
            for action in actions.iter() {
                let state = &states[from_index];
                if action.preconditions_valid(state) {
                    for action_result in action.get_successor_states(&state) {
                        let resulting_state_hash = action_result.state.get_hash();
                        let (index, exists) = match hashmap.get(&resulting_state_hash) {
                            Some(index) => (*index, true),
                            None => (states.len(), false),
                        };

                        if !exists {
                            states.push(action_result.state);
                            hashmap.insert(resulting_state_hash, index);
                            stack.push(resulting_state_hash);
                        }
                        let transition = Transition {
                            from: from_index,
                            to: index,
                            action: action.action(),
                            probability: action_result.probability,
                            reward: action_result.reward,
                        };
                        transitions.push(transition);
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
                        let exists = map.get(&t.action).is_some();
                        let vec = if exists {
                            map.get_mut(&t.action).unwrap()
                        } else {
                            map.insert(t.action.clone(), vec![]);
                            map.get_mut(&t.action).unwrap()
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

    pub fn states(&self) -> &[S] {
        self.states.as_ref()
    }

    pub fn actions(&self, state: usize) -> &HashMap<Rc<dyn ActionType>, Vec<Transition>> {
        &self.actions_from_states[state]
    }
}

impl<S: State + Debug> Mdp<S> {
    pub fn print(&self) {
        println!("\n\n================  Transitions From States  ================\n");
        for (i, state) in self.states.iter().enumerate() {
            println!("State {} : {:?}", i, state);
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
}

pub struct MdpBuilder<S: State> {
    actions: Vec<Box<dyn IActionBuilder<S>>>,
    initial_state: S,
}

impl<S: State> MdpBuilder<S> {
    pub fn new(initial_state: S) -> Self {
        Self {
            actions: vec![],
            initial_state,
        }
    }

    pub fn add_action(mut self, action_builder: Box<dyn IActionBuilder<S>>) -> Self {
        self.actions.push(action_builder);
        self
    }

    pub fn build(self) -> Mdp<S> {
        let actions = self.actions.iter().map(|a| a.build()).flatten().collect();
        Mdp::new(self.initial_state, actions)
    }
}
