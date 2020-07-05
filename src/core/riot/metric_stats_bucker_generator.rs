use crate::core::msvc::vector::StdVector;

#[repr(C)]
pub struct RiotMetricStatsBucketGenerator {
    buckets: StdVector<u32>,       // std::vector<uint>
    bucket_mapper: StdVector<i32>, // std::vector<int>
    bucket_ranges: StdVector<f32>, // std::vector<float>
    bucket_start: f32,
    bucket_end: f32,
}
