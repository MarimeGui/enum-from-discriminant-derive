use std::cmp::max;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Lit, parse_macro_input};

macro_rules! impl_try_from {
    ($number:ty, $name:ident, $biggest:ident, $variants:ident, $implementations:ident) => {
        if $biggest <= (<$number>::MAX as u64) {
            let variants = $variants.clone();
            $implementations.push(
                quote! {
                    impl TryFrom<$number> for #$name {
                        type Error = ();

                        fn try_from(value: $number) -> Result<Self, Self::Error> {
                            Ok(match value {
                                #(i if i == Self::#variants as $number => Self::#variants,)*
                                _ => return Err(())
                            })
                        }
                    }
                }
                .into(),
            )
        }
    };
}

/// Implements ``TryFrom<T>`` on an enum to turn numeric signed and unsigned values into an enum variant based on discriminant value.
///
/// Depending on the value of the biggest discriminant, ``TryFrom<u8>``, ``TryFrom<u16>``,
/// ``TryFrom<u32>``, ``TryFrom<u64>``, ``TryFrom<usize>``, ``TryFrom<i8>``, ``TryFrom<i16>``,
/// ``TryFrom<i32>``, ``TryFrom<i64>`` and ``TryFrom<isize>`` will be implemented.
///
/// Example:
/// ```rs
/// use enum_from_discriminant_derive::TryFromDiscriminant;
///
/// #[derive(TryFromDiscriminant)]
/// enum MyCoolEnum {
///     // Discriminant is implied to be 0 for FirstVariant...
///     FirstVariant,
///     // ... 1 for SecondVariant...
///     SecondVariant,
///     // ... and 16381 for ThirdVariant explicitly.
///     ThirdVariant = 16381
/// }
/// // As the biggest discriminant (16381) does not fit in a u8, `TryFrom<u8>` and `TryFrom<i8>` will NOT be implemented.
/// // However, all others will be.
///
/// fn is_second(value: u16) -> bool {
///     match MyCoolEnum::try_from(value).expect("not a valid discriminant") {
///         MyCoolEnum::SecondVariant => true,
///         _ => false,
///     }
/// }
/// ```
#[proc_macro_derive(TryFromDiscriminant)]
pub fn enum_from_discriminant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Check if enum
    let enum_data = if let Data::Enum(data) = input.data {
        data
    } else {
        return TokenStream::from(
            syn::Error::new(
                name.span(),
                "only enums can derive `TryFrom<_>` from discriminants",
            )
            .to_compile_error(),
        );
    };

    // Find biggest discriminant
    // In Rust, if the discriminant is not explicit, it will always be he previous discriminant plus 1, with the first variant being 0.
    let mut current_discriminant = None;
    let mut biggest_discriminant = None;
    for variant in &enum_data.variants {
        if let Some((_, discriminant_expr)) = &variant.discriminant {
            // We have an explicit value for the discriminant
            if let Expr::Lit(expr) = discriminant_expr {
                if let Lit::Int(i) = &expr.lit {
                    let value = i.base10_parse::<u64>().unwrap();
                    current_discriminant = Some(value)
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
        } else if let Some(current) = current_discriminant {
            // Not the first variant and the discriminant is implicit
            current_discriminant = Some(current + 1)
        } else {
            // This is the first variant and the value isn't explicit
            current_discriminant = Some(0)
        }

        let current = current_discriminant.unwrap();
        if let Some(biggest) = biggest_discriminant {
            biggest_discriminant = Some(max(biggest, current))
        } else {
            biggest_discriminant = Some(current)
        }
    }
    let biggest_discriminant = if let Some(b) = biggest_discriminant {
        b
    } else {
        return TokenStream::from(
            syn::Error::new(name.span(), "enum does not have any variants").to_compile_error(),
        );
    };

    // Implement all possible TryFrom
    let mut implementations: Vec<TokenStream> = Vec::with_capacity(enum_data.variants.len());
    let variants = enum_data.variants.into_iter().map(|v| v.ident);

    impl_try_from!(u8, name, biggest_discriminant, variants, implementations);
    impl_try_from!(u16, name, biggest_discriminant, variants, implementations);
    impl_try_from!(u32, name, biggest_discriminant, variants, implementations);
    impl_try_from!(usize, name, biggest_discriminant, variants, implementations);
    impl_try_from!(u64, name, biggest_discriminant, variants, implementations);

    impl_try_from!(i8, name, biggest_discriminant, variants, implementations);
    impl_try_from!(i16, name, biggest_discriminant, variants, implementations);
    impl_try_from!(i32, name, biggest_discriminant, variants, implementations);
    impl_try_from!(isize, name, biggest_discriminant, variants, implementations);
    impl_try_from!(i64, name, biggest_discriminant, variants, implementations);

    TokenStream::from_iter(implementations)
}
