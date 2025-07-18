//@ run-pass
//@ reference: layout.aggregate.struct-size-align


#[repr(align(64))]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Overaligned(u8);

#[allow(dead_code)]
union ReprRustUnion {
    x: i32,
    y: [u32; 4],
    z: f32,
    a: u128,
    b: Overaligned,
}

#[cfg_attr(test, test)]
fn test_alignment_contains_all_fields() {
    assert!(core::mem::align_of::<ReprRustUnion>() >= core::mem::align_of::<i32>());
    assert!(core::mem::align_of::<ReprRustUnion>() >= core::mem::align_of::<[u32; 4]>());
    assert!(core::mem::align_of::<ReprRustUnion>() >= core::mem::align_of::<f32>());
    assert!(core::mem::align_of::<ReprRustUnion>() >= core::mem::align_of::<u128>());
    assert!(core::mem::align_of::<ReprRustUnion>() >= core::mem::align_of::<Overaligned>());
}

#[cfg(not(test))]
fn main() {
    test_alignment_contains_all_fields();
}
