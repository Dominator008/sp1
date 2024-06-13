use p3_field::Field;
use serde::{Deserialize, Serialize};
use sp1_core::air::MachineProgram;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomProgram {
    // Placeholder
}

impl<F: Field> MachineProgram<F> for CustomProgram {
    fn pc_start(&self) -> F {
        // TODO
        F::one()
    }
}
