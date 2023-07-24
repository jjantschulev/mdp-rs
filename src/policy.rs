use std::fmt::Debug;

use crate::{
    mdp::Mdp,
    model::{ActionBox, State},
};

#[derive(Debug)]
pub struct Policy {
    actions: Vec<Option<ActionBox>>,
}

impl Policy {
    pub fn new(actions: Vec<Option<ActionBox>>) -> Self {
        Self { actions }
    }

    pub fn actions(&self) -> &[Option<ActionBox>] {
        self.actions.as_ref()
    }

    pub fn get_action(&self, state: usize) -> Option<&ActionBox> {
        self.actions[state].as_ref()
    }

    pub fn print<S: State + Debug>(&self, mdp: &Mdp<S>, values: &[f64]) {
        println!("================ Computed Policy ================\n");
        for (state, (action, state_value)) in self.actions().iter().zip(values.iter()).enumerate() {
            println!("State {} {:?}", state, mdp.states()[state]);
            println!("  - Action:      {:?}", action.as_ref(),);
            println!("  - State Value: {:.1}", state_value,);
            println!()
        }
    }
}
