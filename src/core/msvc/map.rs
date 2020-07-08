use std::ptr::NonNull;

#[repr(u8)]
enum Color {
    Red = 0,
    Black = 1,
}

#[repr(C)]
struct Node<K, V> {
    left: Option<NonNull<Node<K, V>>>,
    parent: Option<NonNull<Node<K, V>>>,
    right: Option<NonNull<Node<K, V>>>,
    color: Color,
    nil: bool,
    key: K,
    value: V,
}

impl<K, V> Node<K, V> {}

#[repr(C)]
pub struct StdMap<K, V> {
    head: Option<NonNull<Node<K, V>>>,
    size: u32,
}