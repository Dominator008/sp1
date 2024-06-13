use p3_field::AbstractField;
use sp1_core::{air::PublicValues, stark::MachineRecord};
use std::collections::HashMap;

use crate::fibonacci::FibonacciEvent;

#[derive(Default, Debug, Clone)]
pub struct CustomRecord {
    pub fibonacci_events: Vec<FibonacciEvent>,

    /// The public values.
    pub public_values: PublicValues<u32, u32>,
}

impl MachineRecord for CustomRecord {
    type Config = ();

    fn index(&self) -> u32 {
        0
    }

    fn set_index(&mut self, _: u32) {}

    fn stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert("fibonacci_events".to_string(), self.fibonacci_events.len());
        stats
    }

    // NOTE: This should be unused.
    fn append(&mut self, other: &mut Self) {
        self.fibonacci_events.append(&mut other.fibonacci_events);
    }

    fn shard(mut self, _: &Self::Config) -> Vec<Self> {
        // TODO
        self.public_values.start_pc = 1;
        self.public_values.shard = 1;
        vec![self]
    }

    fn public_values<T: AbstractField>(&self) -> Vec<T> {
        self.public_values.to_vec()
    }
}
