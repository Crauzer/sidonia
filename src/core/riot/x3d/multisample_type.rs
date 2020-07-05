use num_derive::{FromPrimitive, ToPrimitive};

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum X3dMultiSampleType {
    None = 0x0,
    MultiSample2Samples = 0x1,
    MultiSample4Samples = 0x2,
}
