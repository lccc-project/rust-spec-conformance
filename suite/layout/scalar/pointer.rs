//@ run-pass
//@ reference: layout.pointer.thin

struct ReallyLargeType([u8; 8192]);

#[test]
fn test_pointer_size() {
    assert_eq!(
        core::mem::size_of::<*mut ()>(),
        core::mem::size_of::<usize>()
    );
    assert_eq!(
        core::mem::size_of::<*const i32>(),
        core::mem::size_of::<usize>()
    );
    assert_eq!(
        core::mem::size_of::<&ReallyLargeType>(),
        core::mem::size_of::<usize>()
    );
    assert_eq!(
        core::mem::size_of::<&mut &mut u128>(),
        core::mem::size_of::<usize>()
    );
}

#[test]
fn test_pointer_alignment() {
    assert_eq!(
        core::mem::align_of::<*mut ()>(),
        core::mem::align_of::<usize>()
    );
    assert_eq!(
        core::mem::align_of::<*const i32>(),
        core::mem::align_of::<usize>()
    );
    assert_eq!(
        core::mem::align_of::<&ReallyLargeType>(),
        core::mem::align_of::<usize>()
    );
    assert_eq!(
        core::mem::align_of::<&mut &mut u128>(),
        core::mem::align_of::<usize>()
    );
}
