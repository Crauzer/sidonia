use crate::core::riot::replication::RiotReplicate;

#[repr(C)]
pub struct RiotEvolutionState {
    evolve_points: RiotReplicate<u32>,
    flags: RiotReplicate<u32>
}