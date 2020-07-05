use winapi::shared::minwindef::LPVOID;
use crate::core::riot::r3d::screen_buffer::R3dScreenBuffer;
use crate::core::riot::metric_stats_generator::RiotMetricStatsGenerator;
use crate::core::riot::metric_stats_bucker_generator::RiotMetricStatsBucketGenerator;
use crate::core::riot::metrics_details::RiotMetricsDetails;

#[repr(C)]
pub struct RiotRenderPipeline {
    vtbl: LPVOID,
    screen_buffer: *mut R3dScreenBuffer,
    toon_buffer: *mut R3dScreenBuffer,
    shadow_settings_age: u32,
    framerate_metrics: RiotMetricStatsGenerator,
    framerate_buckets: RiotMetricStatsBucketGenerator,
    metrics: RiotMetricsDetails,
    active_metrics: *mut RiotMetricsDetails
}