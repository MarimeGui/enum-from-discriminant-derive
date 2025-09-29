# `enum-from-discriminant-derive`
Crate that provides a single Proc Macro to use on enums, `TryFromDiscriminant`.

It implements ``TryFrom<T>`` on an enum to turn numeric signed and unsigned values into an enum variant based on discriminant value.

Depending on the value of the biggest discriminant, ``TryFrom<u8>``, ``TryFrom<u16>``,
``TryFrom<u32>``, ``TryFrom<u64>``, ``TryFrom<usize>``, ``TryFrom<i8>``, ``TryFrom<i16>``,
``TryFrom<i32>``, ``TryFrom<i64>`` and ``TryFrom<isize>`` will be implemented.

Example:
```rs
use enum_from_discriminant_derive::TryFromDiscriminant;

#[derive(TryFromDiscriminant)]
enum MyCoolEnum {
    // Discriminant is implied to be 0 for FirstVariant...
    FirstVariant,
    // ... 1 for SecondVariant...
    SecondVariant,
    // ... and 16381 for ThirdVariant explicitly.
    ThirdVariant = 16381
}

// As the biggest discriminant (16381) does not fit in a u8, `TryFrom<u8>` and `TryFrom<i8>` will NOT be implemented.
// However, all others will be.

fn is_second(value: u16) -> bool {
    match MyCoolEnum::try_from(value).expect("not a valid discriminant") {
        MyCoolEnum::SecondVariant => true,
        _ => false,
    }
}
```
