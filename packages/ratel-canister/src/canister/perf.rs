use crate::sampling::{PerfMetrics, StepInstructions};

pub fn instruction_counter() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        ic_cdk::api::instruction_counter()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

pub fn cycles_balance() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        ic_cdk::api::canister_cycle_balance() as u64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

pub fn heap_memory_bytes() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        (core::arch::wasm32::memory_size(0) as u64) * 65536
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

pub struct PerfTracker {
    cycles_before: u64,
    steps: Vec<StepInstructions>,
    data_rows: u32,
    features: u32,
    k_range: String,
}

impl PerfTracker {
    pub fn new(data_rows: u32, features: u32, min_k: u32, max_k: u32) -> Self {
        Self {
            cycles_before: cycles_balance(),
            steps: Vec::new(),
            data_rows,
            features,
            k_range: format!("{}-{}", min_k, max_k),
        }
    }

    pub fn start() -> u64 {
        instruction_counter()
    }

    pub fn record(&mut self, step_name: &str, start_counter: u64) {
        let end = instruction_counter();
        self.steps.push(StepInstructions {
            step: step_name.to_string(),
            instructions: end.saturating_sub(start_counter),
        });
    }

    pub fn finish(self) -> PerfMetrics {
        let cycles_after = cycles_balance();
        PerfMetrics {
            instructions_used: instruction_counter(),
            heap_memory_bytes: heap_memory_bytes(),
            cycles_balance_before: self.cycles_before,
            cycles_balance_after: cycles_after,
            cycles_consumed: self.cycles_before.saturating_sub(cycles_after),
            data_rows: self.data_rows,
            features: self.features,
            k_range_tested: self.k_range,
            step_instructions: self.steps,
        }
    }
}
