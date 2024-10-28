//@ run-pass
//@ reference: layout.array

struct HugeType([u16; 4096]);

#[test]
fn test_array_sizes() {
    assert_eq!(core::mem::size_of::<[u8; 47]>(), 47);
    assert_eq!(
        core::mem::size_of::<[u16; 47]>(),
        core::mem::size_of::<u16>() * 47
    );
    assert_eq!(
        core::mem::size_of::<[HugeType; 47]>(),
        core::mem::size_of::<HugeType>() * 47
    );
}
