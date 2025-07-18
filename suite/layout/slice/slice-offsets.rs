//@ run-pass
//@ reference: layout.slice

fn check_slice_elem_offsets<T>() {
    let x = [const { unsafe { core::mem::zeroed::<T>() } }; 32];

    let x = (&x) as &[T];

    let addr = (&raw const *x).cast::<T>();

    let end = unsafe { &raw const x[32..] }.cast::<T>(); // Add single array

    assert_eq!(end, unsafe { addr.add(32) });

    assert_eq!(addr, &raw const x[0]);

    for i in [3, 4, 5, 7, 8, 9, 15, 16, 17, 31] {
        assert!((&raw const x[0]).is_aligned());
        assert_eq!(unsafe { addr.add(i) }, &raw const x[i]);
    }
}

#[cfg_attr(test, test)]
fn test_slice_offsets() {
    check_slice_elem_offsets::<u8>();
    check_slice_elem_offsets::<u16>();
    check_slice_elem_offsets::<u64>();
    check_slice_elem_offsets::<[u8; 63]>();
    check_slice_elem_offsets::<[u16; 17]>();
}

#[cfg(not(test))]
fn main() {
    test_slice_offsets();
}
