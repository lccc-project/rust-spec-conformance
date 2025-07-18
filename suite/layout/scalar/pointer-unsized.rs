//@ run-pass
//@ reference: layout.pointer.unsized

pub trait Simple {}

impl<T> Simple for T {}

#[test]
fn test_dst_size_at_least_thin_ptr() {
    assert!(core::mem::size_of::<*mut [u8]>() >= core::mem::size_of::<*mut u8>());
    assert!(core::mem::size_of::<*mut dyn Simple>() >= core::mem::size_of::<*mut u8>());
}

#[test]
fn test_dst_align_at_least_thin_ptr() {
    assert!(core::mem::align_of::<*mut [u8]>() >= core::mem::align_of::<*mut u8>());
    assert!(core::mem::align_of::<*mut dyn Simple>() >= core::mem::align_of::<*mut u8>());
}
