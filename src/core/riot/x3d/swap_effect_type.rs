use num_derive::{FromPrimitive, ToPrimitive};

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum X3dSwapEffectType {
    Discard = 0x0,
    Flip = 0x1,
    Copy = 0x2,
}
