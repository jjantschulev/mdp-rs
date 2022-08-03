use mdp::MdpBuilder;

use crate::model::{ActionBuilder, VariableSetAssignment, FALSE, TRUE};

mod mdp;
mod model;

fn main() {
    let mut initial_state = VariableSetAssignment::new();
    initial_state.set("visits", 0);
    initial_state.set("jammed", FALSE);
    initial_state.set("banned", FALSE);

    let mdp = MdpBuilder::new(initial_state)
        .add_action(
            ActionBuilder::new("visit_bakery")
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
                })),
        )
        .add_action(
            ActionBuilder::new("rob")
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
                })),
        )
        .add_action(
            ActionBuilder::new("vending_machine")
                .precondition("jammed", Box::new(|a| a == Some(FALSE)))
                .outcome(Box::new(|_| (0.5, 1.0)))
                .outcome(Box::new(|state| {
                    state.set("jammed", TRUE);
                    (0.5, 3.0)
                })),
        )
        .add_action(
            ActionBuilder::new("wait")
                .precondition("jammed", Box::new(|a| a == Some(TRUE)))
                .outcome(Box::new(|state| {
                    state.set("jammed", FALSE);
                    (1.0, 0.0)
                })),
        )
        .build();

    mdp.print();
}
