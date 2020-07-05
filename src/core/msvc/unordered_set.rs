use crate::core::msvc::{list::StdList, vector::StdVector};

#[repr(C)]
#[derive(Debug)]
pub struct StdUnorderedSet<T> {
    max_load_factor: f32,
    items: StdList<T>,
    buckets: StdVector<T>,
    unk1: u32,
    unk2: u32,
}
