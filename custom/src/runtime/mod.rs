mod program;
mod record;

use std::sync::Arc;

pub use program::*;
pub use record::*;

use sp1_core::runtime::ExecutionError;

use crate::fibonacci::FibonacciEvent;

pub const D: usize = 4;

pub struct CustomRuntime {
    /// The program.
    pub program: Arc<CustomProgram>,

    /// The execution record.
    pub record: CustomRecord,
}

impl CustomRuntime {
    pub fn new(program: CustomProgram) -> Self {
        // Create a shared reference to the program.
        let program = Arc::new(program);

        let record = CustomRecord {
            ..Default::default()
        };
        Self { program, record }
    }

    pub fn run(&mut self) -> Result<(), ExecutionError> {
        // TODO: Make use of program
        self.record
            .fibonacci_events
            .push(FibonacciEvent { a: 1, b: 1, n: 64 });
        Ok(())
    }
}
