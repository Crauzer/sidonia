use num_derive::{FromPrimitive, ToPrimitive};

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum X3dFormat {
    Unknown = 0x0,
    X8R8G8B8 = 0x1,
    A8R8G8B8 = 0x2,
    R5G6B5 = 0x3,
    DXT1 = 0x4,
    DXT3 = 0x5,
    DXT5 = 0x6,
    D32 = 0x7,
    D24S8 = 0x8,
    D24X8 = 0x9,
    D16 = 0xA,
    INDEX16 = 0xB,
    INDEX32 = 0xC,
    G16R16F = 0xD,
    A16B16G16R16F = 0xE,
    R32F = 0xF,
    G32R32F = 0x10,
    A32B32G32R32F = 0x11,
    A8 = 0x12,
    A8L8 = 0x13,
    L8 = 0x14,
    A2R10G10B10 = 0x15,
    INTZ = 0x16,
}
