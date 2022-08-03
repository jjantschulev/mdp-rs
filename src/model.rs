use std::collections::HashMap;

pub const TRUE: isize = 1;
pub const FALSE: isize = 0;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VariableSetAssignment {
    assignment: HashMap<String, isize>,
}

impl VariableSetAssignment {
    pub fn new() -> Self {
        Self {
            assignment: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: isize) {
        self.assignment.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&isize> {
        self.assignment.get(name)
    }

    pub fn unset(&mut self, name: &str) -> Option<isize> {
        self.assignment.remove(name)
    }
}

pub struct Action {
    name: String,
    preconditions: Vec<(String, Box<dyn Fn(Option<isize>) -> bool>)>,
    outcomes: Vec<Box<dyn Fn(&mut VariableSetAssignment) -> (f64, f64)>>,
}

impl Action {
    pub fn preconditions_valid(&self, assignment: &VariableSetAssignment) -> bool {
        self.preconditions
            .iter()
            .all(|(name, check)| check(assignment.get(name).map(|i| *i)))
    }

    pub fn get_successor_states(
        &self,
        assignment: &VariableSetAssignment,
    ) -> Vec<(VariableSetAssignment, f64, f64)> {
        self.outcomes
            .iter()
            .map(|transition| {
                let mut a = assignment.clone();
                let (chance, reward) = transition(&mut a);
                (a, chance, reward)
            })
            .collect()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

pub struct ActionBuilder {
    action: Action,
}

impl ActionBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            action: Action {
                name: name.to_owned(),
                outcomes: vec![],
                preconditions: vec![],
            },
        }
    }

    pub fn precondition(mut self, var: &str, valid: Box<dyn Fn(Option<isize>) -> bool>) -> Self {
        self.action.preconditions.push((var.to_owned(), valid));
        self
    }

    pub fn outcome(
        mut self,
        effect: Box<dyn Fn(&mut VariableSetAssignment) -> (f64, f64)>,
    ) -> Self {
        self.action.outcomes.push(effect);
        self
    }

    pub fn build(self) -> Action {
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
