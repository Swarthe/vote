//! naive implementation of a democratic decision-making system based on
//! majority rule

pub mod procedure;
pub mod motion;
pub mod person;

pub use person::{Person, PersonList, PersonId};
pub use motion::Motion;
pub use procedure::Procedure;
