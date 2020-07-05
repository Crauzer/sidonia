use crate::core::msvc::{list::StdList, vector::StdVector};

#[repr(C)]
#[derive(Debug)]
pub struct StdUnorderedMap<K, V> {
    max_load_factor: f32,
    items: StdList<K>,     // TODO: fix type hack
    buckets: StdVector<V>, // TODO: fix type hack
    unk1: u32,
    unk2: u32,
}
