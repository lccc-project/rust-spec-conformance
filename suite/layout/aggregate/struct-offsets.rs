//@ run-pass
//@ reference: layout.aggregate.struct-offsets

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

include!("aggregate-fields.rs");

fn test_fields_make_sense(a: &(usize, usize)) {
    assert!(a.0 <= a.1);
}

// order is `begin, end`
fn test_non_overlapping(a: &(usize, usize), b: &(usize, usize)) {
    assert!((a.1 <= b.0) || (b.1 <= a.0));
}

#[test]
fn test_fields_non_overlapping() {
    let fields = [
        span_of!(ReprRustStruct, x),
        span_of!(ReprRustStruct, y),
        span_of!(ReprRustStruct, z),
        span_of!(ReprRustStruct, a),
        span_of!(ReprRustStruct, b),
    ];

    test_non_overlapping(&fields[0], &fields[1]);
    test_non_overlapping(&fields[0], &fields[2]);
    test_non_overlapping(&fields[0], &fields[3]);
    test_non_overlapping(&fields[0], &fields[4]);
    test_non_overlapping(&fields[1], &fields[2]);
    test_non_overlapping(&fields[2], &fields[3]);
    test_non_overlapping(&fields[2], &fields[4]);
    test_non_overlapping(&fields[3], &fields[4]);
}

#[test]
fn test_fields_aligned() {
    assert_eq!(
        (core::mem::offset_of!(ReprRustStruct, x) % (core::mem::align_of::<i32>())),
        0
    );
    assert_eq!(
        (core::mem::offset_of!(ReprRustStruct, y) % (core::mem::align_of::<[u32; 4]>())),
        0
    );
    assert_eq!(
        (core::mem::offset_of!(ReprRustStruct, z) % (core::mem::align_of::<f32>())),
        0
    );
    assert_eq!(
        (core::mem::offset_of!(ReprRustStruct, a) % (core::mem::align_of::<u128>())),
        0
    );
    assert_eq!(
        (core::mem::offset_of!(ReprRustStruct, b) % (core::mem::align_of::<Overaligned>())),
        0
    );
}
