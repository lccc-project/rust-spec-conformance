//@ run-pass
//@ reference: layout.primitive.size

#[test]
fn test_int_sizes() {
    assert_eq!(core::mem::size_of::<bool>(), 1);
    assert_eq!(core::mem::size_of::<u8>(), 1);
    assert_eq!(core::mem::size_of::<u16>(), 2);
    assert_eq!(core::mem::size_of::<u32>(), 4);
    assert_eq!(core::mem::size_of::<u64>(), 8);
    assert_eq!(core::mem::size_of::<u128>(), 16);
    assert_eq!(core::mem::size_of::<i8>(), 1);
    assert_eq!(core::mem::size_of::<i16>(), 2);
    assert_eq!(core::mem::size_of::<i32>(), 4);
    assert_eq!(core::mem::size_of::<i64>(), 8);
    assert_eq!(core::mem::size_of::<i128>(), 16);
}

#[test]
fn test_float_sizes() {
    assert_eq!(core::mem::size_of::<f32>(), 4);
    assert_eq!(core::mem::size_of::<f64>(), 8);
}

#[test]
fn test_char_size() {
    assert_eq!(core::mem::size_of::<bool>(), 1);
}
