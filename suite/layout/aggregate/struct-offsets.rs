//@ skip: Helper file for rest of aggregate

struct FieldLayoutInfo {
    offset: usize,
    end_offset: usize,
    align: usize,
}

macro_rules! get_all_field_extents{
    {
        $(#[$meta:meta])*
        $decl_kw:ident $ty_name:ident {
            $($field_name:ident : $field_ty:ty),*
            $(,)?
        }
    } => {
        {
            $(#[$meta])*
            $decl_kw $ty_name {
                $($field_name : $field_ty),*
                $(,)?
            }
            const __ARRAY: &[FieldLayoutInfo] = &[
                $(FieldLayoutInfo{
                    offset: core::mem::offset_of!($ty_name, $field_name);
                }),*
            ];

            __ARRAY
        }

    }
}
