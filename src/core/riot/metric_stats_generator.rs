use crate::core::{msvc::vector::StdVector, riot::metrics::RiotMetrics};

#[repr(C)]
pub struct RiotMetricStatsGenerator {
    base_frames: StdVector<f32>,            // std::vector<float>
    average_values: StdVector<RiotMetrics>, // std::vector<Riot::Metrics>
    start_frame_offset: u32,
}
