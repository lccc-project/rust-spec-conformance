//@ compile-fail
//@ reference: items.static.init

fn not_const(x: u32) {
    static FOO: u32 = x;
}
