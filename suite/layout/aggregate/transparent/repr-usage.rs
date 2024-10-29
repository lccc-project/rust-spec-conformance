//@ compile-only
//@ reference: layout.repr.transparent.constraint-field

#[repr(transparent)]
struct Transparent1(u32); // Single field ok

#[repr(transparent)]
struct Transparent2(u64, ()); // Multiple fields are allowed, as long as at most one of them is not a 1-ZST

#[repr(transparent)]
struct Transparent3(u64, [(); 47]); // Unit array is kinda abusrd, but legal

#[repr(transparent)]
struct Transparent4((), (), (), (), u128, (), (), (), ()); // The unit army has captured a u128.

#[repr(transparent)]
struct Transparent5((), (), (), (), (), (), (), ()); // u128 prisoner has escaped, army regroup

#[repr(transparent)]
struct Transparent6; // https://discord.com/channels/273534239310479360/957720175619215380/1300696177783603254

#[repr(transparent)]
struct Transparent7(u128, [u8; 0]); // u128 brought backup from the only 1-ZST they can trust.

#[repr(transparent)]
enum Transparent8 {
    Camp(u128, (), (), (), (), (), (), (), (), [u8; 0]), // unit army is captured
}
