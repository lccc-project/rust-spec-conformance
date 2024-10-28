//@ run-pass
//@ reference: layout.array

#[repr(align(4096))]
struct HugeType([u16; 4096]);

#[test]
fn test_array_sizes() {
    assert_eq!(core::mem::align_of::<[u8; 47]>(), 1);
    assert_eq!(
        core::mem::align_of::<[u16; 47]>(),
        core::mem::align_of::<u16>()
    );
    assert_eq!(
        core::mem::align_of::<[HugeType; 47]>(),
        core::mem::align_of::<HugeType>()
    );
}
