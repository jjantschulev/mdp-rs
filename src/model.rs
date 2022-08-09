use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{Hash, Hasher},
    marker::PhantomData,
    rc::Rc,
};

pub trait State: Clone + Eq + Hash {
    fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl<T: Clone + Eq + Hash> State for T {}

type PreconditionFn<S> = dyn Fn(&S) -> bool;
type OutcomeFn<S> = dyn Fn(&mut S, &mut f64) -> f64;
type ActionBasedPreconditionFn<S, A> = dyn Fn(Rc<A>) -> Rc<dyn Fn(&S) -> bool>;
type ActionBasedOutcomeFn<S, A> = dyn Fn(Rc<A>) -> Rc<dyn Fn(&mut S, &mut f64) -> f64>;

pub struct Action<S: State> {
    action: Rc<dyn ActionType>,
    preconditions: Vec<Rc<PreconditionFn<S>>>,
    outcomes: Vec<Rc<OutcomeFn<S>>>,
}

pub struct ActionResult<S: State> {
    pub(crate) state: S,
    pub(crate) probability: f64,
    pub(crate) reward: f64,
}

impl<S: State> Action<S> {
    pub fn preconditions_valid(&self, state: &S) -> bool {
        self.preconditions.iter().all(|check| check(state))
    }

    pub fn get_successor_states(&self, state: &S) -> Vec<ActionResult<S>> {
        self.outcomes
            .iter()
            .map(|transition| {
                let mut next_state = state.clone();
                let mut reward = 0.0;
                let chance = transition(&mut next_state, &mut reward);
                ActionResult {
                    state: next_state,
                    probability: chance,
                    reward,
                }
            })
            .collect()
    }

    pub fn name(&self) -> String {
        self.action.to_string()
    }
}

pub struct SingleActionBuilder<S: State, A: ActionType> {
    preconditions: Vec<Rc<PreconditionFn<S>>>,
    outcomes: Vec<Rc<OutcomeFn<S>>>,
    action: Rc<A>,
    action_type: PhantomData<A>,
}

impl<S: State, A: ActionType + 'static> SingleActionBuilder<S, A> {
    pub fn new(action: A) -> Self {
        Self {
            outcomes: vec![],
            preconditions: vec![],
            action_type: PhantomData,
            action: Rc::new(action),
        }
    }

    pub fn precondition(mut self, valid: Rc<PreconditionFn<S>>) -> Self {
        self.preconditions.push(valid);
        self
    }

    pub fn outcome(mut self, effect: Rc<OutcomeFn<S>>) -> Self {
        self.outcomes.push(effect);
        self
    }

    pub fn build(&self) -> Action<S> {
        Action {
            action: self.action.clone(),
            preconditions: self.preconditions.clone(),
            outcomes: self.outcomes.clone(),
        }
    }
}

pub struct GroundingActionBuilder<S: State, A: ActionType> {
    preconditions: Vec<Rc<ActionBasedPreconditionFn<S, A>>>,
    outcomes: Vec<Rc<ActionBasedOutcomeFn<S, A>>>,
    action_type: PhantomData<A>,
}

impl<S: State, A: ActionType + GrounableAction + 'static> GroundingActionBuilder<S, A> {
    pub fn new() -> Self {
        Self {
            outcomes: vec![],
            preconditions: vec![],
            action_type: PhantomData,
        }
    }

    pub fn precondition(mut self, valid: Rc<ActionBasedPreconditionFn<S, A>>) -> Self {
        self.preconditions.push(valid);
        self
    }

    pub fn outcome(mut self, effect: Rc<ActionBasedOutcomeFn<S, A>>) -> Self {
        self.outcomes.push(effect);
        self
    }

    pub fn build(&self) -> Vec<Action<S>> {
        A::enumerate()
            .into_iter()
            .map(|action| {
                let a = Rc::new(action);
                let preconditions = self.preconditions.iter().map(|p| p(a.clone())).collect();
                let outcomes = self.outcomes.iter().map(|p| p(a.clone())).collect();
                return Action {
                    action: a,
                    preconditions,
                    outcomes,
                };
            })
            .collect()
    }
}

pub trait ActionType {
    fn to_string(&self) -> String;
}

pub trait GrounableAction: Sized {
    fn enumerate() -> Vec<Self>;
}

impl<T: Debug> ActionType for T {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub trait IActionBuilder<S: State> {
    fn build(&self) -> Vec<Action<S>>;
}

impl<S: State, A: ActionType + 'static> IActionBuilder<S> for SingleActionBuilder<S, A> {
    fn build(&self) -> Vec<Action<S>> {
        vec![self.build()]
    }
}

impl<S: State, A: ActionType + 'static + GrounableAction> IActionBuilder<S>
    for GroundingActionBuilder<S, A>
{
    fn build(&self) -> Vec<Action<S>> {
        GroundingActionBuilder::build(&self)
    }
}
