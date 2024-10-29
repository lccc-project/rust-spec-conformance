//@ compile-fail
//@ reference: layout.repr.transparent.constraint-exclusive

#[repr(transparent, align(16))]
struct Oops(u128);
