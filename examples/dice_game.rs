use std::rc::Rc;

use mdp_rs::{mdp::MdpBuilder, model::SingleActionBuilder, solver::ValueIterationSolver};

const NUM_TURNS: isize = 100;
const DICE: isize = 20;

#[derive(Debug, Hash)]
struct Roll;
#[derive(Debug, Hash)]
struct Take;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct WorldState {
    dice: isize,
    turns: isize,
}

fn main() {
    let initial_state = WorldState {
        dice: -1,
        turns: NUM_TURNS + 1,
    };

    let mdp = MdpBuilder::new(initial_state)
        .add_action(Box::new({
            let mut a = SingleActionBuilder::<WorldState, Roll>::new(Roll)
                .precondition(Rc::new(|state| state.turns > 0));
            for v in 0..DICE {
                a = a.outcome(Rc::new(move |state, _| {
                    state.dice = v + 1;
                    state.turns -= 1;
                    1.0 / DICE as f64
                }));
            }
            a
        }))
        .add_action(Box::new(
            SingleActionBuilder::<WorldState, Take>::new(Take)
                .precondition(Rc::new(|state| state.turns > 0 && state.dice >= 0))
                .outcome(Rc::new(move |state, reward| {
                    *reward = state.dice as f64;
                    state.turns -= 1;
                    1.0
                })),
        ))
        .build();

    mdp.print();

    // Solve the MDP with a Value Iteration Solver
    let mut solver = ValueIterationSolver::new(&mdp, 1.0);
    solver.solve();
    let policy = solver.get_policy();
    policy.print(&mdp, solver.values());

    // println!("================ Take Threshold ================\n");
}
