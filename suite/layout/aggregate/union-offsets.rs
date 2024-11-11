//@ run-pass
//@ reference: layout.aggregate.struct-offsets

#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct Overaligned(u8);

union ReprRustUnion {
    x: i32,
    y: [u32; 4],
    z: f32,
    a: u128,
    b: Overaligned,
}

#[test]
fn test_fields_aligned() {
    assert_eq!(
        (core::mem::offset_of!(ReprRustUnion, x) % (core::mem::align_of::<i32>())),
        0
    );
    assert_eq!(
        (core::mem::offset_of!(ReprRustUnion, y) % (core::mem::align_of::<[u32; 4]>())),
        0
    );
    assert_eq!(
        (core::mem::offset_of!(ReprRustUnion, z) % (core::mem::align_of::<f32>())),
        0
    );
    assert_eq!(
        (core::mem::offset_of!(ReprRustUnion, a) % (core::mem::align_of::<u128>())),
        0
    );
    assert_eq!(
        (core::mem::offset_of!(ReprRustUnion, b) % (core::mem::align_of::<Overaligned>())),
        0
    );
}
