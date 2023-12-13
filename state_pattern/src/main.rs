/*
    The state pattern is a object-oriented design pattern that allows an object to alter its behavior
    when its internal state changes.
    When using the state pattern, an object will be in one of several pre-defined states.
    The object will behave differently depending on its internal state.
    The object will change its own state based on some event.
    From the outside, a developer will only interact with the same API regardless of the object's internal state.

    One advantage of the state pattern is that it extracts business logic.
    When business logic changes, the API on the outside of the object does not change.
    Only the specific implementations of states will change.

    It is also easy to expand on the state pattern and add new states.

    One could use enums over the state pattern.
    However, enums are non-exhaustive.
    This means that everywhere where the enum is used, a match statement is required, handling all possible states.

    Another advantage of the state pattern is that every state can hold its own data.
    This allows, for example, to store a number and implement a counting mechanism in just one state.
    This would make it easy to implement an approval mechanism that requires multiple approvals.

    One disadvantage of the state pattern is that the different states are coupled tightly to each other.
    For example, when adding a new state B between A and C,
    it is necessary to change the implementation of A to not transition to C anymore, but to B.

    Another disadvantage is that the state pattern introduces a certain level of redundancy.
    Some states may implement methods that are not available in other states.
    The developer has to find a way to handle this elegantly.
    On the other hand, multiple states may implement a certain behaviour in the same way.
    This can be handled by implementing a default behaviour in the trait.

    In the following example, we will apply the state pattern to implement a
    blog post that can be in one of three states: Draft, PendingReview, or Published.

    We will first implement a more traditional, object-oriented approach.
    Then, we'll also do a more natural approach.
*/

mod state_traditional;

fn main() {
    println!("=== Traditional Approach ===");
    state_traditional::run();

    println!("=== Rust Approach ===");
}
