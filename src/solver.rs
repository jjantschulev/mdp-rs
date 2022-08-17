use crate::{mdp::Mdp, model::State, policy::Policy};

const EPSILON: f64 = 0.00001;

pub struct ValueIterationSolver<'a, S: State> {
    values: Vec<f64>,
    old_values: Vec<f64>,
    mdp: &'a Mdp<S>,
    discount: f64,
}

impl<'a, S: State> ValueIterationSolver<'a, S> {
    pub fn new(mdp: &'a Mdp<S>, discount: f64) -> Self {
        Self {
            values: vec![0.0; mdp.states().len()],
            old_values: vec![0.0; mdp.states().len()],
            mdp,
            discount,
        }
    }

    fn iterate(&mut self) {
        // Save the old values, so we can compare and stop when the change is small enough.
        self.old_values.clone_from(&self.values);

        for i in 0..self.mdp.states().len() {
            let actions = self.mdp.actions(i);
            let max_value = actions
                .iter()
                .map(|(_, transitions)| {
                    let mut expected_reward = 0.0;
                    for t in transitions {
                        expected_reward += t.probability()
                            * (t.reward() + self.discount * self.old_values[t.to()]);
                    }
                    expected_reward
                })
                .reduce(|accum, item| if accum >= item { accum } else { item })
                .unwrap_or_default();

            self.values[i] = max_value;
        }
    }

    pub fn solve(&mut self) {
        loop {
            self.iterate();
            let is_done = self
                .values
                .iter()
                .zip(self.old_values.iter())
                .all(|(a, b)| (*a - *b).abs() < EPSILON);
            if is_done {
                break;
            }
        }
    }

    pub fn values(&self) -> &[f64] {
        self.values.as_ref()
    }

    pub fn get_policy(&self) -> Policy {
        let actions = self
            .mdp
            .states()
            .iter()
            .enumerate()
            .map(|(index, _)| {
                self.mdp
                    .actions(index)
                    .iter()
                    .map(|(action, transitions)| {
                        let mut expected_reward = 0.0;
                        for t in transitions {
                            expected_reward += t.probability()
                                * (t.reward() + self.discount * self.old_values[t.to()]);
                        }
                        (expected_reward, action)
                    })
                    .reduce(|accum, item| if accum.0 >= item.0 { accum } else { item })
                    .map(|(_, name)| name.clone())
            })
            .collect();

        Policy::new(actions)
    }
}
