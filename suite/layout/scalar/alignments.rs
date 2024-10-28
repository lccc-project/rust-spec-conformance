//@ run-pass
//@ reference: layout.primitive.alignment

fn check_valid_align<T>() {
    let align = core::mem::align_of::<T>();
    assert!(align.is_power_of_two());

    assert!(align <= core::mem::size_of::<T>());
}

#[test]
fn test_exact_alignments() {
    assert_eq!(core::mem::align_of::<u8>(), 1);
    assert_eq!(core::mem::align_of::<bool>(), 1);
}

#[test]
fn test_signed_unsigned_same_align() {
    assert_eq!(core::mem::align_of::<u8>(), core::mem::align_of::<i8>());
    assert_eq!(core::mem::align_of::<u16>(), core::mem::align_of::<i16>());
    assert_eq!(core::mem::align_of::<u32>(), core::mem::align_of::<i32>());
    assert_eq!(core::mem::align_of::<u64>(), core::mem::align_of::<i64>());
    assert_eq!(core::mem::align_of::<u128>(), core::mem::align_of::<i128>());
    assert_eq!(
        core::mem::align_of::<usize>(),
        core::mem::align_of::<isize>()
    );
}

#[test]
fn test_aligns_valid() {
    check_valid_align::<u16>();
    check_valid_align::<u32>();
    check_valid_align::<u64>();
    check_valid_align::<u128>();
    check_valid_align::<usize>();
    check_valid_align::<f32>();
    check_valid_align::<f64>();
    check_valid_align::<char>();
}
