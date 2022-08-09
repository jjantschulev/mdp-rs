use std::fmt::Debug;

use crate::{mdp::Mdp, model::State};

#[derive(Debug)]
pub struct Policy {
    actions: Vec<Option<String>>,
}

impl Policy {
    pub fn new(actions: Vec<Option<String>>) -> Self {
        Self { actions }
    }

    pub fn actions(&self) -> &[Option<String>] {
        self.actions.as_ref()
    }

    pub fn print<S: State + Debug>(&self, mdp: &Mdp<S>, values: &[f64]) {
        println!("================ Computed Policy ================\n");
        for (state, (action, state_value)) in self.actions().iter().zip(values.iter()).enumerate() {
            println!("State {} {:?}", state, mdp.states()[state]);
            println!(
                "  - Action:      {:?}",
                action.as_ref().unwrap_or(&"None".to_string()),
            );
            println!("  - State Value: {:.1}", state_value,);
            println!()
        }
    }
}
