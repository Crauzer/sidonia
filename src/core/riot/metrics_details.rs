use crate::core::riot::metric_stats_generator::RiotMetricStatsGenerator;

#[repr(C)]
pub struct RiotMetricsDetails {
    shadow_metric_stats_generator: RiotMetricStatsGenerator,
    terrain_metric_stats_generator: RiotMetricStatsGenerator,
    lake_metric_stats_generator: RiotMetricStatsGenerator,
    skinned_objects1_metric_stats_generator: RiotMetricStatsGenerator,
    particles1_metric_stats_generator: RiotMetricStatsGenerator,
    particles2_metric_stats_generator: RiotMetricStatsGenerator,
    particles3_metric_stats_generator: RiotMetricStatsGenerator,
    skinned_objects2_metric_stats_generator: RiotMetricStatsGenerator,
    inking_metric_stats_generator: RiotMetricStatsGenerator,
    selection_effects_metric_stats_generator: RiotMetricStatsGenerator,
}
