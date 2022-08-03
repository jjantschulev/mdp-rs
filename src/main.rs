use crate::{
    mdp::Mdp,
    model::{ActionBuilder, VariableSet, VariableSetAssignment, FALSE, TRUE},
};

mod mdp;
mod model;

fn main() {
    println!("Hello, world!");

    // let var_set = VariableSet::new()
    //     .add_range("visits", 0..2)
    //     .add_boolean("banned")
    //     .add_boolean("jammed");

    let visit_bakery = ActionBuilder::new("visit_bakery")
        .precondition("banned", Box::new(|a| a == Some(FALSE)))
        .outcome(Box::new(|state| match state.get("visits") {
            Some(0) => {
                state.set("visits", 1);
                (1.0, 1.0)
            }
            Some(1) => {
                state.set("visits", 0);
                (1.0, 4.0)
            }
            _ => unreachable!(),
        }))
        .build();

    let rob = ActionBuilder::new("rob")
        .precondition("banned", Box::new(|a| a == Some(FALSE)))
        .outcome(Box::new(|state| {
            match state.get("visits") {
                Some(0) => {
                    state.set("visits", 1);
                }
                _ => {}
            };
            (0.85, 5.0)
        }))
        .outcome(Box::new(|state| {
            state.unset("visits");
            state.set("banned", TRUE);
            (0.15, 0.0)
        }))
        .build();

    let vending_machine = ActionBuilder::new("vending_machine")
        .precondition("jammed", Box::new(|a| a == Some(FALSE)))
        .outcome(Box::new(|_| (0.5, 1.0)))
        .outcome(Box::new(|state| {
            state.set("jammed", TRUE);
            (0.5, 3.0)
        }))
        .build();

    let wait = ActionBuilder::new("wait")
        .precondition("jammed", Box::new(|a| a == Some(TRUE)))
        .outcome(Box::new(|state| {
            state.set("jammed", FALSE);
            (1.0, 0.0)
        }))
        .build();

    let mut initial_state = VariableSetAssignment::new();
    initial_state.set("visits", 0);
    initial_state.set("jammed", FALSE);
    initial_state.set("banned", FALSE);

    let mdp = Mdp::new(
        initial_state,
        vec![visit_bakery, rob, wait, vending_machine],
    );

    mdp.print();
}
