use std::rc::Rc;

use mdp_rs::{mdp::MdpBuilder, model::SingleActionBuilder, solver::ValueIterationSolver};

// World state definition
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct State {
    visits: usize,
    jammed: bool,
    banned: bool,
}

// Action definitions
#[derive(Debug, Hash)]
pub struct VisitBakery;
#[derive(Debug, Hash)]
pub struct RobBakery;
#[derive(Debug, Hash)]
pub struct VendingMachine;
#[derive(Debug, Hash)]
pub struct Wait;

fn main() {
    // Create initial state
    let initial_state = State {
        banned: false,
        jammed: false,
        visits: 0,
    };

    // Create mdp and add actions + preconditions + effects to the mdp
    let mdp = MdpBuilder::new(initial_state)
        .add_action(Box::new(
            SingleActionBuilder::<State, VisitBakery>::new(VisitBakery)
                .precondition(Rc::new(|s| !s.banned))
                .outcome(Rc::new(|state, reward| {
                    state.visits = (state.visits + 1) % 2;
                    *reward = 1.0;
                    if state.visits == 0 {
                        *reward += 3.0;
                    }
                    1.0
                })),
        ))
        .add_action(Box::new(
            SingleActionBuilder::<State, RobBakery>::new(RobBakery)
                .precondition(Rc::new(|s| !s.banned))
                .outcome(Rc::new(|state, reward| {
                    if state.visits != 1 {
                        state.visits += 1
                    }
                    *reward = 5.0;
                    0.85
                }))
                .outcome(Rc::new(|state, _reward| {
                    state.banned = true;
                    state.visits = 0;
                    0.15
                })),
        ))
        .add_action(Box::new(
            SingleActionBuilder::<State, VendingMachine>::new(VendingMachine)
                .precondition(Rc::new(|s| !s.jammed))
                .outcome(Rc::new(|_s, reward| {
                    *reward = 1.0;
                    0.5
                }))
                .outcome(Rc::new(|state, reward| {
                    *reward = 3.0;
                    state.jammed = true;
                    0.5
                })),
        ))
        .add_action(Box::new(
            SingleActionBuilder::<State, Wait>::new(Wait)
                .precondition(Rc::new(|s| s.jammed))
                .outcome(Rc::new(|state, _r| {
                    state.jammed = false;
                    1.0
                })),
        ))
        .build();

    mdp.print();

    // Solve the MDP with a Value Iteration Solver
    let mut solver = ValueIterationSolver::new(&mdp, 0.94);
    solver.solve();
    let policy = solver.get_policy();
    policy.print(&mdp, solver.values());
}
