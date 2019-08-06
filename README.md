# proc_macro_attribute_order

Shows the order of proc_macro_attribute execution for different attributes on the same item.

```rust
// attr_* appends a new field to the struct

#[attr_0(u8)]
#[attr_1(u16)]
#[attr_2(u32)]
struct Hello;

#[attr_2(u32)]
#[attr_1(u16)]
#[attr_0(u8)]
struct Hello2;
```

generates:

```rust
// cargo expand --test test
struct Hello(u8, u16, u32);
struct Hello2(u32, u16, u8);
```
