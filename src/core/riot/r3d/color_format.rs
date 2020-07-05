#[repr(C)]
pub struct R3dColorFormat {
    bits: u32,
    bytes: u32,
    a_bits: u32,
    r_bits: u32,
    g_bits: u32,
    b_bits: u32,
    a_pos: u32,
    r_pos: u32,
    g_pos: u32,
    b_pos: u32,
    color_mask: u32,
    rgb_mask: u32,
    make_color_fn: extern "cdecl" fn(i32, i32, i32, i32) -> u32,
    split_color_fn: extern "cdecl" fn(u32, *mut i32, *mut i32, *mut i32, *mut i32),
}
