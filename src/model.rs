use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub trait State: Clone + Eq + Hash {
    fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl<T: Clone + Eq + Hash> State for T {}

pub struct Action<S: State> {
    name: String,
    preconditions: Vec<Box<dyn Fn(&S) -> bool>>,
    outcomes: Vec<Box<dyn Fn(&mut S, &mut f64) -> f64>>,
}

pub struct ActionResult<S: State> {
    pub(crate) state: S,
    pub(crate) probability: f64,
    pub(crate) reward: f64,
}

impl<S: State> Action<S> {
    pub fn preconditions_valid(&self, state: &S) -> bool {
        self.preconditions.iter().all(|check| check(state))
    }

    pub fn get_successor_states(&self, state: &S) -> Vec<ActionResult<S>> {
        self.outcomes
            .iter()
            .map(|transition| {
                let mut next_state = state.clone();
                let mut reward = 0.0;
                let chance = transition(&mut next_state, &mut reward);
                ActionResult {
                    state: next_state,
                    probability: chance,
                    reward,
                }
            })
            .collect()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

pub struct ActionBuilder<S: State> {
    action: Action<S>,
}

impl<S: State> ActionBuilder<S> {
    pub fn new(name: &str) -> Self {
        Self {
            action: Action {
                name: name.to_owned(),
                outcomes: vec![],
                preconditions: vec![],
            },
        }
    }

    pub fn precondition(mut self, valid: Box<dyn Fn(&S) -> bool>) -> Self {
        self.action.preconditions.push(valid);
        self
    }

    pub fn outcome(mut self, effect: Box<dyn Fn(&mut S, &mut f64) -> f64>) -> Self {
        self.action.outcomes.push(effect);
        self
    }

    pub fn build(self) -> Action<S> {
        self.action
    }
}

// Might need this later hehe:

// pub const BOOLEAN: [isize; 2] = [TRUE, FALSE];

// #[derive(Clone)]
// pub struct VariableSet {
//     set: HashMap<String, Vec<isize>>,
// }

// impl VariableSet {
//     pub fn new() -> Self {
//         Self {
//             set: HashMap::new(),
//         }
//     }

//     pub fn add<T: Iterator<Item = isize>>(mut self, name: &str, values: T) -> Self {
//         self.set.insert(name.to_owned(), values.collect());
//         self
//     }

//     pub fn add_slice(mut self, name: &str, values: &[isize]) -> Self {
//         self.set.insert(name.to_owned(), values.to_vec());
//         self
//     }

//     pub fn add_boolean(mut self, name: &str) -> Self {
//         self.set.insert(name.to_owned(), BOOLEAN.to_vec());
//         self
//     }

//     pub fn add_range(mut self, name: &str, range: Range<isize>) -> Self {
//         self.set
//             .insert(name.to_owned(), range.into_iter().collect());
//         self
//     }
// }
