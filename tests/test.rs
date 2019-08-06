use proc_macro_attribute_order::{attr_0, attr_1, attr_2};

#[attr_0(u8)]
#[attr_1(u16)]
#[attr_2(u32)]
struct Hello;

#[attr_2(u32)]
#[attr_1(u16)]
#[attr_0(u8)]
struct Hello2;
