//@ compile-fail
//@ reference: layout.repr.transparent.constraint-field

#[repr(transparent)]
pub struct Foo(u128, u64);
