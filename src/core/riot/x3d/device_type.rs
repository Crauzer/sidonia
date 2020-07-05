use num_derive::{FromPrimitive, ToPrimitive};

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum X3dDeviceType {
    Hal = 0,
    Ref = 1,
}
