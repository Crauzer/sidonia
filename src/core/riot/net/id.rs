pub type NetIdType = u32;

#[repr(C)]
#[derive(Debug)]
pub struct NetId {
    id_type: NetIdType,
}
