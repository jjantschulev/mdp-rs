use mdp_rs::{
    mdp::MdpBuilder,
    model::{ActionBuilder, VariableSetAssignment},
};

const WIDTH: isize = 20;
const HEIGHT: isize = 20;
const COST: f64 = -1.0;

fn main() {
    let mut initial_state = VariableSetAssignment::new();
    initial_state.set("x", 0);
    initial_state.set("y", 0);

    let mdp = MdpBuilder::new(initial_state)
        .add_action(
            ActionBuilder::new("up")
                .outcome(Box::new(|state| {
                    let y = *state.get("y").unwrap();
                    if y < HEIGHT - 1 {
                        state.set("y", y + 1);
                    }
                    (0.8, COST)
                }))
                .outcome(Box::new(|state| {
                    let x = *state.get("x").unwrap();
                    if x < WIDTH - 1 {
                        state.set("x", x + 1);
                    }
                    (0.1, COST)
                }))
                .outcome(Box::new(|state| {
                    let x = *state.get("x").unwrap();
                    if x > 0 {
                        state.set("x", x - 1);
                    }
                    (0.1, COST)
                })),
        )
        .add_action(
            ActionBuilder::new("down")
                .outcome(Box::new(|state| {
                    let y = *state.get("y").unwrap();
                    if y > 0 {
                        state.set("y", y - 1);
                    }
                    (0.8, COST)
                }))
                .outcome(Box::new(|state| {
                    let x = *state.get("x").unwrap();
                    if x < WIDTH - 1 {
                        state.set("x", x + 1);
                    }
                    (0.1, COST)
                }))
                .outcome(Box::new(|state| {
                    let x = *state.get("x").unwrap();
                    if x > 0 {
                        state.set("x", x - 1);
                    }
                    (0.1, COST)
                })),
        )
        .add_action(
            ActionBuilder::new("left")
                .outcome(Box::new(|state| {
                    let x = *state.get("x").unwrap();
                    if x > 0 {
                        state.set("x", x - 1);
                    }
                    (0.8, COST)
                }))
                .outcome(Box::new(|state| {
                    let y = *state.get("y").unwrap();
                    if y < HEIGHT - 1 {
                        state.set("y", y + 1);
                    }
                    (0.1, COST)
                }))
                .outcome(Box::new(|state| {
                    let y = *state.get("y").unwrap();
                    if y > 0 {
                        state.set("y", y - 1);
                    }
                    (0.1, COST)
                })),
        )
        .add_action(
            ActionBuilder::new("right")
                .outcome(Box::new(|state| {
                    let x = *state.get("x").unwrap();
                    if x < WIDTH - 1 {
                        state.set("x", x + 1);
                    }
                    (0.8, COST)
                }))
                .outcome(Box::new(|state| {
                    let y = *state.get("y").unwrap();
                    if y < HEIGHT - 1 {
                        state.set("y", y + 1);
                    }
                    (0.1, COST)
                }))
                .outcome(Box::new(|state| {
                    let y = *state.get("y").unwrap();
                    if y > 0 {
                        state.set("y", y - 1);
                    }
                    (0.1, COST)
                })),
        )
        .build();

    mdp.print();
}
