//@ run-pass
//@ reference: layout.repr.rust.enum.size-align

#[repr(align(64))]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Overaligned(u8);

pub enum ReprRustEnum {
    A(u128, i32, f32),
    B { a: [u8; 32], b: Overaligned },
}

macro_rules! span_of_enum_fields{
    ($instance:expr, $ty:ident, $variant:ident :: $($field:tt),+ $(,)?) => {
        [$({
            const __FOO: (usize, usize) = {
                const fn size_of_val_const<T>(x: &T) -> usize{
                    core::mem::size_of::<T>()
                }
                let val = $instance;
                match &val {
                    $ty::$variant {$field: __field, ..} => {
                        let start = unsafe{ core::ptr::addr_of!(*__field).byte_offset_from(core::ptr::addr_of!(val)) as usize };
                        let end = start + size_of_val_const(__field);
                        (start, end)
                    }
                    _ => panic!(concat!("enum is not correct variant. Expected ", stringify!($variant)))
                }
            };

            __FOO
        }),*]
    };
}

fn test_fields_make_sense(a: &(usize, usize)) {
    assert!(a.0 <= a.1);
}

// order is `begin, end`
fn test_non_overlapping(a: &(usize, usize), b: &(usize, usize)) {
    assert!((a.1 <= b.0) || (b.1 <= a.0));
}

#[test]
fn test_fields_non_overlapping() {
    let fields_a = span_of_enum_fields!(ReprRustEnum::A(0,0,0.0), ReprRustEnum, A:: 0, 1, 2);
    let fields_b = span_of_enum_fields!(
        ReprRustEnum::B {
            a: [0; 32],
            b: Overaligned(0)
        },
        ReprRustEnum,
        B::a,
        b
    );

    test_fields_make_sense(&fields_a[0]);
    test_fields_make_sense(&fields_a[1]);
    test_fields_make_sense(&fields_a[2]);
    test_fields_make_sense(&fields_b[0]);
    test_fields_make_sense(&fields_b[1]);

    test_non_overlapping(&fields_a[0], &fields_a[1]);
    test_non_overlapping(&fields_a[0], &fields_a[2]);
    test_non_overlapping(&fields_a[1], &fields_a[2]);
    test_non_overlapping(&fields_b[0], &fields_b[1]);
}
