# MDP Playground

I'm making this repo as an environment for experiementing with MDP's and algorithms used to solve and construct them. This is both to improve my knowledge of rust and MDPs for COMP4620 @ ANU.

## Features

Listing features here to not forget what i've built.

### `model.rs`

A simple way to build an `Mdp`. The `MdpBuilder` uses a state / action based way to build the Mdp.  
Essentially you can pass an initial "world_state" to the builder. This state may have many variables (eg, `num_visits` and `is_banned`). The values of these variables must be integers.  
Then you can add actions using the `ActionBuilder`. Each action has a

- string name.
- list of preconditions. Each precondition must resolve to true for the action to be avaliable.  
  The preconditions can use variables from the world state.
- list of outcomes. Each outcome is essentially a function that has mutable access to the world state. The outcome can mutate the world state to induce a state transition.
  The function must return the probability of this particular outcome occurring and the associated reward with this particular outcome.

### `mdp.rs`

This is a struct to store the information about a MDP. Namely: State, Transitions and Actions.

You can construct a `Mdp` using the `MdpBuilder::build()` function. This will start at the initial state provided.  
It will then apply allowed actions (by their preconditions) and build new "states" that encode the changes made by the action's outcomes. This continues until no more states can be reached by applying actions to states.

Finally, it organizes the avaliable actions and transitions from actions in a hashmap for each state.

### `solver.rs`

This is a very simple value iteration solver for an `Mdp`. It can also generate a policy once solved.
