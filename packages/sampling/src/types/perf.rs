use serde::{Deserialize, Serialize};

use super::output::SamplingResult;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct StepInstructions {
    pub step: String,
    pub instructions: u64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct PerfMetrics {
    pub instructions_used: u64,
    pub heap_memory_bytes: u64,
    pub cycles_balance_before: u64,
    pub cycles_balance_after: u64,
    pub cycles_consumed: u64,
    pub data_rows: u32,
    pub features: u32,
    pub k_range_tested: String,
    pub step_instructions: Vec<StepInstructions>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct SamplingWithMetrics {
    pub result: SamplingResult,
    pub metrics: PerfMetrics,
}
