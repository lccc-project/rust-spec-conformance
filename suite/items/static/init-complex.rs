//@ compile-only
//@ reference: items.static.init

static FOO: u32 = 1;

const BAR: u32 = 2;

static BAZ: u32 = (FOO + 1) * (BAR + 1) * (FOO + BAR) * (1 + 1);
