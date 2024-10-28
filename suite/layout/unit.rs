//@ run-pass
//@ reference: layout.tuple.unit

#[test]
fn test_unit_size() {
    assert_eq!(core::mem::size_of::<()>(), 0);
}

#[test]
fn test_unit_align() {
    assert_eq!(core::mem::align_of::<()>(), 1);
}
