// type utils

use super::state;

pub trait MLETrait: state::MessageLevelEncryptionState + Clone {}

// This is a blanket implementation for all types that implement the
// `MessageLevelEncryptionState` trait and the `Clone` trait. Useful for
// testing.
impl<T> MLETrait for T where T: state::MessageLevelEncryptionState + Clone {}
