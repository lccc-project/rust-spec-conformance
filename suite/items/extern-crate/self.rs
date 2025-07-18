//@ compile-only

extern crate self as foo;

mod bar {
    pub const BAZ: u32 = 0;
}

const _: () = {
    foo::bar::BAZ;
};
