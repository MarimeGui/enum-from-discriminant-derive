use enum_from_discriminant_derive::TryFromDiscriminant;

#[test]
fn implicit_discriminants() {
    #[derive(Debug, TryFromDiscriminant, PartialEq, Eq)]
    enum SomeEnum {
        A,
        B,
        C,
        D,
    }

    assert_eq!(SomeEnum::try_from(2u8), Ok(SomeEnum::C));
    assert_eq!(SomeEnum::try_from(2u16), Ok(SomeEnum::C));
    assert_eq!(SomeEnum::try_from(2u32), Ok(SomeEnum::C));
    assert_eq!(SomeEnum::try_from(2u64), Ok(SomeEnum::C));
    assert_eq!(SomeEnum::try_from(2usize), Ok(SomeEnum::C));

    assert_eq!(SomeEnum::try_from(2i8), Ok(SomeEnum::C));
    assert_eq!(SomeEnum::try_from(2i16), Ok(SomeEnum::C));
    assert_eq!(SomeEnum::try_from(2i32), Ok(SomeEnum::C));
    assert_eq!(SomeEnum::try_from(2i64), Ok(SomeEnum::C));
    assert_eq!(SomeEnum::try_from(2isize), Ok(SomeEnum::C));
}

#[test]
fn explicit_discriminants() {
    #[derive(Debug, TryFromDiscriminant, PartialEq, Eq)]
    enum SomeEnum {
        A = 0,
        B = 1,
        C = 2,
        D = 3,
    }

    assert_eq!(SomeEnum::try_from(2), Ok(SomeEnum::C))
}

#[test]
fn mixed_discriminants() {
    #[derive(Debug, TryFromDiscriminant, PartialEq, Eq)]
    enum SomeEnum {
        A,
        B = 1,
        C = 2003,
        D = 153,
        E,
    }

    assert_eq!(SomeEnum::try_from(2004u16), Err(()));
    assert_eq!(SomeEnum::try_from(153u16), Ok(SomeEnum::D));
    assert_eq!(SomeEnum::try_from(154u16), Ok(SomeEnum::E));
}
