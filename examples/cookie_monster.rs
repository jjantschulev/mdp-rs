use mdp_rs::{mdp::MdpBuilder, model::ActionBuilder, solver::ValueIterationSolver};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct State {
    visits: usize,
    jammed: bool,
    banned: bool,
}

fn main() {
    let initial_state = State {
        banned: false,
        jammed: false,
        visits: 0,
    };

    let mdp = MdpBuilder::new(initial_state)
        .add_action(
            ActionBuilder::<State>::new("visit_bakery")
                .precondition(Box::new(|s| !s.banned))
                .outcome(Box::new(|state, reward| {
                    state.visits = (state.visits + 1) % 2;
                    *reward = 1.0;
                    if state.visits == 0 {
                        *reward += 3.0;
                    }
                    1.0
                })),
        )
        .add_action(
            ActionBuilder::<State>::new("rob")
                .precondition(Box::new(|s| !s.banned))
                .outcome(Box::new(|state, reward| {
                    if state.visits != 1 {
                        state.visits += 1
                    }
                    *reward = 5.0;
                    0.85
                }))
                .outcome(Box::new(|state, _reward| {
                    state.banned = true;
                    state.visits = 0;
                    0.15
                })),
        )
        .add_action(
            ActionBuilder::<State>::new("vending_machine")
                .precondition(Box::new(|s| !s.jammed))
                .outcome(Box::new(|_s, reward| {
                    *reward = 1.0;
                    0.5
                }))
                .outcome(Box::new(|state, reward| {
                    *reward = 3.0;
                    state.jammed = true;
                    0.5
                })),
        )
        .add_action(
            ActionBuilder::<State>::new("wait")
                .precondition(Box::new(|s| s.jammed))
                .outcome(Box::new(|state, _r| {
                    state.jammed = false;
                    1.0
                })),
        )
        .build();

    mdp.print();

    let mut solver = ValueIterationSolver::new(&mdp, 0.99);
    solver.solve();
    let policy = solver.get_policy();
    policy.print(&mdp, solver.values());
}
