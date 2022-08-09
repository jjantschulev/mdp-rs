use std::rc::Rc;

use mdp_rs::{
    mdp::MdpBuilder,
    model::{GrounableAction, GroundingActionBuilder},
};

const WORLD_WIDTH: isize = 3;
const WORLD_HEIGHT: isize = 3;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn walk(&mut self, Walk(dir): &Walk) {
        match dir {
            Up => self.y = (self.y + 1).clamp(1, WORLD_HEIGHT),
            Down => self.y = (self.y - 1).clamp(1, WORLD_HEIGHT),
            Left => self.x = (self.x - 1).clamp(1, WORLD_WIDTH),
            Right => self.x = (self.x + 1).clamp(1, WORLD_WIDTH),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

impl Direction {
    fn orthogonal1(&self) -> Direction {
        match self {
            Up => Left,
            Down => Left,
            Left => Up,
            Right => Up,
        }
    }

    fn orthogonal2(&self) -> Direction {
        match self {
            Up => Right,
            Down => Right,
            Left => Down,
            Right => Down,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Walk(Direction);

impl GrounableAction for Walk {
    fn enumerate() -> Vec<Self> {
        vec![Walk(Up), Walk(Down), Walk(Left), Walk(Right)]
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct WorldState {
    pos: Position,
}

fn main() {
    let initial_state = WorldState {
        pos: Position { x: 1, y: 1 },
    };

    let mdp = MdpBuilder::new(initial_state)
        .add_action(Box::new(
            GroundingActionBuilder::<WorldState, Walk>::new()
                .outcome(Rc::new(|action| {
                    Rc::new(move |state, _reward| {
                        state.pos.walk(&action);
                        0.8
                    })
                }))
                .outcome(Rc::new(|action| {
                    Rc::new(move |state, _reward| {
                        state.pos.walk(&Walk(action.0.orthogonal1()));
                        0.1
                    })
                }))
                .outcome(Rc::new(|action| {
                    Rc::new(move |state, _reward| {
                        state.pos.walk(&Walk(action.0.orthogonal2()));
                        0.1
                    })
                })),
        ))
        .build();

    mdp.print();
}
