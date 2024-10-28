//@ run-pass
//@ reference: layout.primitive.size-int

#[test]
fn test_usize_isize_same_size() {
    assert_eq!(core::mem::size_of::<usize>(), core::mem::size_of::<isize>());
}

#[test]
#[cfg_attr(not(target_pointer_width = "16"), should_panic)]
fn test_usize_16_bit() {
    assert_eq!(core::mem::size_of::<usize>(), 2)
}

#[test]
#[cfg_attr(not(target_pointer_width = "32"), should_panic)]
fn test_usize_32_bit() {
    assert_eq!(core::mem::size_of::<usize>(), 4)
}

#[test]
#[cfg_attr(not(target_pointer_width = "64"), should_panic)]
fn test_usize_64_bit() {
    assert_eq!(core::mem::size_of::<usize>(), 8)
}
