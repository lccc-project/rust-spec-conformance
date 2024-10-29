//@ run-pass
//@ reference: layout.repr.transparent.layout-abi

struct Unit;

#[repr(transparent)]
struct Transparent1<T>(T);

#[repr(transparent)]
struct Transparent2<T> {
    val: T,
    unit: (),
}

#[repr(transparent)]
struct Transparent3<T>(Unit, T);

#[repr(transparent)]
enum Transparent4<T> {
    SingleVariant(T),
}

fn check_transparent_size<T>() {
    assert_eq!(
        core::mem::size_of::<Transparent1<T>>(),
        core::mem::size_of::<T>()
    );
    assert_eq!(
        core::mem::size_of::<Transparent2<T>>(),
        core::mem::size_of::<T>()
    );
    assert_eq!(
        core::mem::size_of::<Transparent3<T>>(),
        core::mem::size_of::<T>()
    );
    assert_eq!(
        core::mem::size_of::<Transparent4<T>>(),
        core::mem::size_of::<T>()
    );
}

fn check_transparent_align<T>() {
    assert_eq!(
        core::mem::align_of::<Transparent1<T>>(),
        core::mem::align_of::<T>()
    );
    assert_eq!(
        core::mem::align_of::<Transparent2<T>>(),
        core::mem::align_of::<T>()
    );
    assert_eq!(
        core::mem::align_of::<Transparent3<T>>(),
        core::mem::align_of::<T>()
    );
    assert_eq!(
        core::mem::align_of::<Transparent4<T>>(),
        core::mem::align_of::<T>()
    );
}

#[test]
fn test_transparent_size() {
    check_transparent_size::<u8>();
    check_transparent_size::<u32>();
    check_transparent_size::<&str>();
    check_transparent_size::<()>();
}

#[test]
fn test_transparent_align() {
    check_transparent_align::<u8>();
    check_transparent_align::<u32>();
    check_transparent_align::<&str>();
    check_transparent_align::<()>();
}
