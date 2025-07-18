//@ run-pass
//@ reference: layout.slice

#[repr(align(4096))]
struct HugeType([u16; 4096]);

fn test_slice_size_align<T, const N: usize>(x: &[T; N]) {
    let y = x as &[T];

    assert_eq!(core::mem::size_of::<[T; N]>(), core::mem::size_of_val(y));
    assert_eq!(core::mem::align_of::<[T; N]>(), core::mem::align_of_val(y));
    assert_eq!(core::mem::align_of::<T>(), core::mem::align_of_val(y));
}

#[cfg(not(target_pointer_width = "16"))]
static HUGE: [HugeType; 47] = [const { unsafe { core::mem::zeroed() } }; 47];

#[test]
fn test_slice_layouts() {
    test_slice_size_align(&[0u8; 47]);
    test_slice_size_align(&[0u16; 47]);
    #[cfg(not(target_pointer_width = "16"))]
    test_slice_size_align(&HUGE);
    test_slice_size_align(&[(); 47]);
}
