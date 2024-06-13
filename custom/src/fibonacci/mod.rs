use core::borrow::Borrow;
use p3_air::PairBuilder;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::PrimeField32;
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;
use sp1_core::air::MachineAir;
use sp1_core::air::MachineAirBuilder;
use sp1_derive::AlignedBorrow;
use tracing::instrument;

use crate::runtime::{CustomProgram, CustomRecord};

pub const NUM_FIBONACCI_COLS: usize = core::mem::size_of::<FibonacciCols<u8>>();

#[derive(Default)]
pub struct FibonacciChip<F> {
    pub _phantom: std::marker::PhantomData<F>,
}

#[derive(Debug, Clone)]
pub struct FibonacciEvent {
    pub a: u64,
    pub b: u64,
    pub n: usize,
}

#[derive(AlignedBorrow, Debug, Clone, Copy)]
#[repr(C)]
pub struct FibonacciPrepCols<T: Copy> {
    pub zero: T,
}

#[derive(AlignedBorrow, Debug, Clone, Copy)]
#[repr(C)]
pub struct FibonacciCols<T: Copy> {
    pub left: T,
    pub right: T,
}

impl<F: Send + Sync> BaseAir<F> for FibonacciChip<F> {
    fn width(&self) -> usize {
        NUM_FIBONACCI_COLS
    }
}

impl<F: PrimeField32> MachineAir<F> for FibonacciChip<F> {
    type Record = CustomRecord;

    type Program = CustomProgram;

    fn name(&self) -> String {
        "Fibonacci".to_string()
    }

    fn preprocessed_width(&self) -> usize {
        1
    }

    fn generate_preprocessed_trace(&self, _program: &Self::Program) -> Option<RowMajorMatrix<F>> {
        // Dummy
        // TODO: Remove?
        let trace = RowMajorMatrix::new(vec![F::zero(); 64], 1);
        Some(trace)
    }

    fn generate_dependencies(&self, _: &Self::Record, _: &mut Self::Record) {
        // This is a no-op.
    }

    #[instrument(name = "generate fibonacci trace", level = "debug", skip_all, fields(rows = input.fibonacci_events[0].n))]
    fn generate_trace(&self, input: &CustomRecord, _: &mut CustomRecord) -> RowMajorMatrix<F> {
        assert!(input.fibonacci_events.len() == 1);
        let event = &input.fibonacci_events[0];
        let a = event.a;
        let b = event.b;
        let n = event.n;

        assert!(n.is_power_of_two());

        let mut trace =
            RowMajorMatrix::new(vec![F::zero(); n * NUM_FIBONACCI_COLS], NUM_FIBONACCI_COLS);

        let (prefix, rows, suffix) = unsafe { trace.values.align_to_mut::<FibonacciCols<F>>() };
        assert!(prefix.is_empty(), "Alignment should match");
        assert!(suffix.is_empty(), "Alignment should match");
        assert_eq!(rows.len(), n);

        rows[0].left = F::from_canonical_u64(a);
        rows[0].right = F::from_canonical_u64(b);

        for i in 1..n {
            rows[i].left = rows[i - 1].right;
            rows[i].right = rows[i - 1].left + rows[i - 1].right;
        }

        #[cfg(debug_assertions)]
        println!(
            "fibonacci trace dims is width: {:?}, height: {:?}",
            trace.width(),
            trace.height()
        );

        trace
    }

    fn included(&self, record: &Self::Record) -> bool {
        !record.fibonacci_events.is_empty()
    }
}

impl<AB> Air<AB> for FibonacciChip<AB::F>
where
    AB: MachineAirBuilder + PairBuilder,
{
    fn eval(&self, builder: &mut AB) {
        let prep = builder.preprocessed();
        let main = builder.main();

        let dummy = prep.row_slice(0);
        let (local, next) = (main.row_slice(0), main.row_slice(1));
        let dummy: &FibonacciPrepCols<AB::Var> = (*dummy).borrow();
        let local: &FibonacciCols<AB::Var> = (*local).borrow();
        let next: &FibonacciCols<AB::Var> = (*next).borrow();

        builder.assert_zero(dummy.zero);

        let mut when_first_row = builder.when_first_row();

        // TODO: Make configurable?

        let mut when_transition = builder.when_transition();

        // a' <- b
        when_transition.assert_eq(local.right, next.left);

        // b' <- a + b
        when_transition.assert_eq(local.left + local.right, next.right);
    }
}
