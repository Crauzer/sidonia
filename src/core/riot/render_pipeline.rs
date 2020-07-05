use crate::core::riot::{
    metric_stats_bucker_generator::RiotMetricStatsBucketGenerator, metric_stats_generator::RiotMetricStatsGenerator,
    metrics_details::RiotMetricsDetails, r3d::screen_buffer::R3dScreenBuffer,
};
use winapi::shared::minwindef::LPVOID;

#[repr(C)]
pub struct RiotRenderPipeline {
    vtbl: LPVOID,
    screen_buffer: *mut R3dScreenBuffer,
    toon_buffer: *mut R3dScreenBuffer,
    shadow_settings_age: u32,
    framerate_metrics: RiotMetricStatsGenerator,
    framerate_buckets: RiotMetricStatsBucketGenerator,
    metrics: RiotMetricsDetails,
    active_metrics: *mut RiotMetricsDetails,
}
