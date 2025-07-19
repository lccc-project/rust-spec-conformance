//@ compile-fail
//@ reference: items.static.init.omission

#[rustfmt::skip]
extern "C" {
    static FOO: u32 = 0;
}
