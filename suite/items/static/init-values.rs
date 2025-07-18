//@ run-pass
//@ reference: items.static.init

static FOO: u32 = 0;

#[test]
fn foo() {
    assert_eq!(FOO, 0);
}

static BAR: u32 = 1;
#[test]
fn bar() {
    assert_eq!(BAR, 1);
}

static BAZ: u32 = 1337;

#[test]
fn baz() {
    assert_eq!(BAZ, 1337);
}

static SREF: &u32 = &BAZ;

#[test]
fn sref_val() {
    assert_eq!(SREF, &BAZ);
}

#[test]
fn sref_addr() {
    assert_eq!(SREF as *const u32, &raw const BAZ);
}
