//@ compile-fail
//@ reference: layout.repr.transparent.constraint-exclusive

#[repr(transparent, C)]
struct Oops(u128);
