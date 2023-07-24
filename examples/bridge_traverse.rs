use std::rc::Rc;

use mdp_rs::{mdp::MdpBuilder, model::SingleActionBuilder, solver::ValueIterationSolver};

#[derive(Debug, Hash)]
struct GoForward(Vec<Person>);
#[derive(Debug, Hash)]
struct GoBackward(Vec<Person>);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Person {
    traverse_time: usize,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct WorldState {
    at_start: Vec<Person>,
    at_end: Vec<Person>,
    flashlight_at_start: bool,
}

impl WorldState {
    fn is_finished(&self) -> bool {
        self.at_start.is_empty() && self.at_end.len() == PEOPLE.len()
    }
}

static PEOPLE: [Person; 4] = [
    Person { traverse_time: 1 },
    Person { traverse_time: 2 },
    Person { traverse_time: 5 },
    Person { traverse_time: 10 },
];

fn main() {
    let initial_state = WorldState {
        at_start: PEOPLE.clone().into(),
        at_end: vec![],
        flashlight_at_start: true,
    };

    let mut mdp = MdpBuilder::new(initial_state.clone());

    for (i, p1) in PEOPLE.iter().enumerate() {
        mdp = mdp.add_action(Box::new(
            SingleActionBuilder::<WorldState, GoForward>::new(GoForward(vec![p1.clone()]))
                .precondition(Rc::new(move |s| !s.is_finished()))
                .precondition(Rc::new(|s| s.flashlight_at_start))
                .precondition(Rc::new(move |s| s.at_start.contains(&p1)))
                .outcome(Rc::new(move |s, r| {
                    s.at_start.retain(|p| p != p1);
                    s.at_end.push(p1.clone());
                    s.flashlight_at_start = false;
                    *r = -(p1.traverse_time as f64);
                    1.0
                })),
        ));
        mdp = mdp.add_action(Box::new(
            SingleActionBuilder::<WorldState, GoBackward>::new(GoBackward(vec![p1.clone()]))
                .precondition(Rc::new(move |s| !s.is_finished()))
                .precondition(Rc::new(|s| !s.flashlight_at_start))
                .precondition(Rc::new(move |s| s.at_end.contains(&p1)))
                .outcome(Rc::new(move |s, r| {
                    s.at_end.retain(|p| p != p1);
                    s.at_start.push(p1.clone());
                    s.flashlight_at_start = true;
                    *r = -(p1.traverse_time as f64);
                    1.0
                })),
        ));
        for (j, p2) in PEOPLE.iter().enumerate() {
            if j <= i {
                continue;
            }
            mdp = mdp.add_action(Box::new(
                SingleActionBuilder::<WorldState, GoForward>::new(GoForward(vec![
                    p1.clone(),
                    p2.clone(),
                ]))
                .precondition(Rc::new(move |s| !s.is_finished()))
                .precondition(Rc::new(|s| s.flashlight_at_start))
                .precondition(Rc::new(move |s| {
                    s.at_start.contains(&p1) && s.at_start.contains(&p2)
                }))
                .outcome(Rc::new(move |s, r| {
                    s.at_start.retain(|p| p != p1 && p != p2);
                    s.at_end.push(p1.clone());
                    s.at_end.push(p2.clone());
                    s.flashlight_at_start = false;
                    *r = -(usize::max(p1.traverse_time, p2.traverse_time) as f64);
                    1.0
                })),
            ));
            mdp = mdp.add_action(Box::new(
                SingleActionBuilder::<WorldState, GoBackward>::new(GoBackward(vec![
                    p1.clone(),
                    p2.clone(),
                ]))
                .precondition(Rc::new(move |s| !s.is_finished()))
                .precondition(Rc::new(|s| !s.flashlight_at_start))
                .precondition(Rc::new(move |s| {
                    s.at_end.contains(&p1) && s.at_end.contains(&p2)
                }))
                .outcome(Rc::new(move |s, r| {
                    s.at_end.retain(|p| p != p1 && p != p2);
                    s.at_start.push(p1.clone());
                    s.at_start.push(p2.clone());
                    s.flashlight_at_start = true;
                    *r = -(usize::max(p1.traverse_time, p2.traverse_time) as f64);
                    1.0
                })),
            ));
        }
    }

    let mdp = mdp.build();

    // mdp.print();

    // Solve the MDP with a Value Iteration Solver
    let mut solver = ValueIterationSolver::new(&mdp, 1.0);
    solver.solve();
    let policy = solver.get_policy();
    // policy.print(&mdp, solver.values());

    let mut current_state = initial_state.clone();
    let mut total_time = 0;
    while !current_state.is_finished() {
        println!("{:?}", current_state);
        let state_index = mdp.index_of_state(&current_state).unwrap();
        let transitions = mdp
            .actions(state_index)
            .get(policy.get_action(state_index).unwrap())
            .unwrap();

        let next_state_index = transitions.first().unwrap().to();
        let next_state = mdp.states()[next_state_index].clone();

        let changes = current_state
            .at_start
            .iter()
            .filter(|p| !next_state.at_start.contains(p))
            .chain(
                current_state
                    .at_end
                    .iter()
                    .filter(|p| !next_state.at_end.contains(p)),
            )
            .collect::<Vec<_>>();
        println!(
            "\n{} people crossed the bridge: {:?}\n",
            changes.len(),
            changes
        );

        total_time += changes.iter().map(|p| p.traverse_time).max().unwrap_or(0);

        current_state = next_state;
    }
    println!("{:?}", current_state);

    println!("\nTotal time: {}", total_time);
}
