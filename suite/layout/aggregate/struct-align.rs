//@ run-pass
//@ reference: layout.aggregate.struct-size-align

#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct Overaligned(u8);

struct ReprRustStruct {
    x: i32,
    y: [u32; 4],
    z: f32,
    a: u128,
    b: Overaligned,
}

#[test]
fn test_alignment_contains_all_fields() {
    assert!(core::mem::align_of::<ReprRustStruct>() >= core::mem::align_of::<i32>());
    assert!(core::mem::align_of::<ReprRustStruct>() >= core::mem::align_of::<[u32; 4]>());
    assert!(core::mem::align_of::<ReprRustStruct>() >= core::mem::align_of::<f32>());
    assert!(core::mem::align_of::<ReprRustStruct>() >= core::mem::align_of::<u128>());
    assert!(core::mem::align_of::<ReprRustStruct>() >= core::mem::align_of::<Overaligned>());
}
