use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Lit, parse_macro_input};

// TODO: Macro or fn for all quotes
// TODO: Make biggest_discriminant better
// TODO: Example code
// TODO: Test against enums with properties

/// Implements ``TryFrom<T>`` on an enum to turn numeric unsigned values into an enum variant based on discriminant value.
///
/// Depending on the value of the biggest discriminant, ``TryFrom<u8>``, ``TryFrom<u16>``,
/// ``TryFrom<u32>``, ``TryFrom<u64>`` and ``TryFrom<usize>`` will be implemented.
#[proc_macro_derive(TryFromDiscriminant)]
pub fn enum_from_discriminant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Check if enum
    let enum_data = if let Data::Enum(data) = input.data {
        data
    } else {
        return TokenStream::from(
            syn::Error::new(name.span(), "only enums can derive `TryFromU8`").to_compile_error(),
        );
    };

    // Find biggest discriminant
    let mut biggest_discriminant = None;
    for variant in &enum_data.variants {
        if let Some((_, discriminant_expr)) = &variant.discriminant {
            if let Expr::Lit(expr) = discriminant_expr {
                if let Lit::Int(i) = &expr.lit {
                    let value = i.base10_parse::<u64>().unwrap();
                    biggest_discriminant = Some(value)
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
        } else if let Some(biggest) = biggest_discriminant {
            biggest_discriminant = Some(biggest + 1)
        } else {
            biggest_discriminant = Some(0)
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

    if biggest_discriminant <= (u8::MAX as u64) {
        let variants = variants.clone();
        implementations.push(
            quote! {
                impl TryFrom<u8> for #name {
                    type Error = ();

                    fn try_from(value: u8) -> Result<Self, Self::Error> {
                        Ok(match value {
                            #(i if i == Self::#variants as u8 => Self::#variants,)*
                            _ => return Err(())
                        })
                    }
                }
            }
            .into(),
        )
    }

    if biggest_discriminant <= (u16::MAX as u64) {
        let variants = variants.clone();
        implementations.push(
            quote! {
                impl TryFrom<u16> for #name {
                    type Error = ();

                    fn try_from(value: u16) -> Result<Self, Self::Error> {
                        Ok(match value {
                            #(i if i == Self::#variants as u16 => Self::#variants,)*
                            _ => return Err(())
                        })
                    }
                }
            }
            .into(),
        )
    }

    if biggest_discriminant <= (u32::MAX as u64) {
        let variants = variants.clone();
        implementations.push(
            quote! {
                impl TryFrom<u32> for #name {
                    type Error = ();

                    fn try_from(value: u32) -> Result<Self, Self::Error> {
                        Ok(match value {
                            #(i if i == Self::#variants as u32 => Self::#variants,)*
                            _ => return Err(())
                        })
                    }
                }
            }
            .into(),
        )
    }

    if biggest_discriminant <= (usize::MAX as u64) {
        let variants = variants.clone();
        implementations.push(
            quote! {
                impl TryFrom<usize> for #name {
                    type Error = ();

                    fn try_from(value: usize) -> Result<Self, Self::Error> {
                        Ok(match value {
                            #(i if i == Self::#variants as usize => Self::#variants,)*
                            _ => return Err(())
                        })
                    }
                }
            }
            .into(),
        )
    }

    // Pretty sure discriminants can't get any bigger than a u64
    implementations.push(
        quote! {
            impl TryFrom<u64> for #name {
                type Error = ();

                fn try_from(value: u64) -> Result<Self, Self::Error> {
                    Ok(match value {
                        #(i if i == Self::#variants as u64 => Self::#variants,)*
                        _ => return Err(())
                    })
                }
            }
        }
        .into(),
    );

    TokenStream::from_iter(implementations)
}
