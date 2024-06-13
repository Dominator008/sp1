use crate::fibonacci::FibonacciChip;
use core::iter::once;
use p3_field::PrimeField32;
use sp1_core::stark::{Chip, StarkGenericConfig, StarkMachine, PROOF_MAX_NUM_PVS};
use sp1_derive::MachineAir;
use std::marker::PhantomData;

#[derive(MachineAir)]
#[sp1_core_path = "sp1_core"]
#[execution_record_path = "crate::runtime::CustomRecord"]
#[program_path = "crate::runtime::CustomProgram"]
#[builder_path = "sp1_core::air::MachineAirBuilder<F = F>"]
pub enum CustomAir<F: PrimeField32> {
    Fibonacci(FibonacciChip<F>),
}

impl<F: PrimeField32> CustomAir<F> {
    /// A custom machine
    pub fn machine<SC: StarkGenericConfig<Val = F>>(config: SC) -> StarkMachine<SC, Self> {
        let chips = Self::get_all()
            .into_iter()
            .map(Chip::new)
            .collect::<Vec<_>>();
        StarkMachine::new(config, chips, PROOF_MAX_NUM_PVS)
    }

    pub fn get_all() -> Vec<Self> {
        once(CustomAir::Fibonacci(FibonacciChip {
            _phantom: PhantomData,
        }))
        .collect()
    }
}
