//@ skip: helper file

macro_rules! span_of {
    ($ty:ty , $field:tt) => {{
        let __field = unsafe { ::core::mem::zeroed::<$ty>() };

        (
            core::mem::offset_of!($ty, $field),
            core::mem::offset_of!($ty, $field) + core::mem::size_of_val(&__field.$field),
        )
    }};
}
