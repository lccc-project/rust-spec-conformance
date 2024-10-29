//@ compile-fail
//@ reference: layout.repr.transparent.constraint-field

#[repr(transparent)]
enum Foo {
    Bar(u128),
    Baz(u128),
}
